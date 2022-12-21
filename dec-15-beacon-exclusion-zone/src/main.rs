use lazy_static::lazy_static;
use log::LevelFilter;
use log::{debug, error, info, trace};
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
struct Bounds {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

#[derive(Debug)]
struct Sensor {
    loc: Point,
    range: i64,
}

impl Sensor {
    pub fn min_x(&self, row: i64) -> i64 {
        self.loc.x - self.row_range(row)
    }

    pub fn max_x(&self, row: i64) -> i64 {
        self.loc.x + self.row_range(row)
    }

    pub fn row_range(&self, row: i64) -> i64 {
        self.range - (row - self.loc.y).abs()
    }
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i64,
    end: i64,
}

fn in_range(start: i64, x: i64, end: i64) -> bool {
    start <= x && x <= end
}

impl Range {
    pub fn from_sensor_row(sensor: &Sensor, row: i64) -> Option<Self> {
        if sensor.row_range(row) < 0 {
            return None;
        }
        Some(Self {
            start: sensor.min_x(row),
            end: sensor.max_x(row),
        })
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        in_range(self.start, other.start, self.end)
            || in_range(self.start, other.end, self.end)
            || in_range(other.start, self.start, other.end)
            || in_range(other.start, self.end, other.end)
    }

    pub fn from_overlapping(a: &Range, b: &Range) -> Option<Self> {
        if a.overlaps(b) {
            return Some(Range {
                start: std::cmp::min(a.start, b.start),
                end: std::cmp::max(a.end, b.end),
            });
        }
        None
    }
}

fn get_overlapping_ranges(ranges: &[Range]) -> Vec<Range> {
    if ranges.len() < 2 {
        return ranges.to_vec();
    }
    trace!("Ranges: {:?}", ranges);
    let mut overlapped_ranges = Vec::<Range>::new();
    overlapped_ranges.push(ranges[0]);

    let mut no_overlaps = true;
    for range in ranges.iter().skip(1) {
        let mut found_overlap = false;
        for overlap in overlapped_ranges.iter_mut() {
            if let Some(r) = Range::from_overlapping(range, overlap) {
                *overlap = r;
                found_overlap = true;
                no_overlaps = false;
            };
        }
        if !found_overlap {
            overlapped_ranges.push(*range);
        }
    }
    if no_overlaps {
        return overlapped_ranges;
    }
    get_overlapping_ranges(&overlapped_ranges)
}

fn get_scanned_ranges_in_row(row: i64, sensors: &[Sensor]) -> Vec<Range> {
    let mut ranges = Vec::<Range>::new();
    for sensor in sensors.iter() {
        if let Some(r) = Range::from_sensor_row(sensor, row) {
            ranges.push(r);
        };
    }
    // Get all the overlapping ranges
    let mut overlaps = get_overlapping_ranges(&ranges);
    overlaps.sort_by(|a: &Range, b: &Range| a.start.cmp(&b.start));
    trace!("Overlapping ranges: {:?}", overlaps);
    overlaps
}

fn count_not_possible_beacons(row: i64, sensors: &[Sensor], beacons: &[Point]) -> usize {
    let scanned_ranges = get_scanned_ranges_in_row(row, sensors);
    debug!("Scanned ranges: {:?}", scanned_ranges);

    // Can't be in scan lines
    let mut num_points: usize = 0;
    for r in scanned_ranges.iter() {
        num_points += (r.end - r.start) as usize + 1;
    }

    // If there's a beacon in a scan, then there is a beacon
    for b in beacons.iter() {
        if b.y != row {
            continue;
        }
        let mut already_scanned = false;
        for r in scanned_ranges.iter() {
            already_scanned |= in_range(r.start, b.x, r.end);
        }
        if already_scanned {
            num_points -= 1;
        }
    }
    num_points
}

fn find_lost_beacon(bounds: Bounds, sensors: &[Sensor], beacons: &[Point]) -> Option<Point> {
    for row in bounds.min_y..=bounds.max_y {
        trace!("--- ROW {} ---", row);
        let scanned_ranges: Vec<_> = get_scanned_ranges_in_row(row, sensors)
            .iter_mut()
            .filter(|r| bounds.min_x <= r.end && r.start <= bounds.max_x)
            .map(|r| Range {
                start: std::cmp::max(r.start, bounds.min_x),
                end: std::cmp::min(r.end, bounds.max_x),
            })
            .collect();
        if scanned_ranges.len() == 1
            && scanned_ranges[0].end - scanned_ranges[0].start == bounds.max_x - bounds.min_x
        {
            continue;
        }
        debug!("Scanned ranges: {:?}", scanned_ranges);

        let mut num_not_possible = 0;
        for range in scanned_ranges.iter() {
            num_not_possible += range.end - range.start;
        }
        for b in beacons.iter() {
            if b.y != row {
                continue;
            }
            let mut already_scanned = false;
            for r in scanned_ranges.iter() {
                already_scanned |= in_range(r.start, b.x, r.end);
            }
            if !already_scanned {
                num_not_possible += 1;
            }
        }
        let num_possible = (bounds.max_x - bounds.min_x) - num_not_possible;
        debug!("Row {} has possible: {}", row, num_possible);
        let mut candidates = Vec::<i64>::new();
        for i in 1..scanned_ranges.len() {
            let start = scanned_ranges[i - 1].end + 1;
            let end = scanned_ranges[i].start - 1;
            for j in start..=end {
                candidates.push(j)
            }
        }
        trace!("Candidate positions: {:?}", candidates);
        for &col in candidates.iter() {
            trace!("Checking point ({}, {})", col, row);
            let mut already_beacon = false;
            for b in beacons.iter() {
                if b.y != row {
                    continue;
                }
                already_beacon |= b.x == col;
            }
            if !already_beacon {
                return Some(Point { x: col, y: row });
            } else {
                trace!("Beacon at ({}, {})", col, row);
            }
        }
    }
    None
}

fn parse_coord(s: &str) -> Point {
    lazy_static! {
        static ref POINT_RE: Regex = Regex::new(r"-?\d+").unwrap();
    }
    let mut matches = POINT_RE.find_iter(s);
    Point {
        x: matches.next().unwrap().as_str().parse::<i64>().unwrap(),
        y: matches.next().unwrap().as_str().parse::<i64>().unwrap(),
    }
}

// Functions that load data from files should return results.
fn load_sensors(fname: &str) -> (Vec<Sensor>, Vec<Point>) {
    let file = match File::open(fname) {
        Ok(f) => f,
        Err(err) => panic!("Could not open file \"{}\": {}", fname, err),
    };
    let reader = BufReader::new(file);
    let mut sensors = Vec::<Sensor>::new();
    let mut beacons = Vec::<Point>::new();
    for read_line in reader.lines() {
        let line = read_line.unwrap();
        if line.is_empty() {
            continue;
        }
        let line_iter: Vec<&str> = line.split(": ").collect();
        let sensor_loc = parse_coord(line_iter[0]);
        let beacon_loc = parse_coord(line_iter[1]);

        let range = (sensor_loc.x - beacon_loc.x).abs() + (sensor_loc.y - beacon_loc.y).abs();

        sensors.push(Sensor {
            loc: sensor_loc,
            range,
        });
        let mut beacon_already_found = false;
        for b in beacons.iter() {
            beacon_already_found |= beacon_loc.x == b.x && beacon_loc.y == b.y;
        }
        if !beacon_already_found {
            beacons.push(beacon_loc);
        }
    }
    (sensors, beacons)
}

fn setup_logger() {
    let env = env_logger::Env::new().filter("RUST_LOG");
    env_logger::builder()
        .format_timestamp(None)
        .format_indent(None)
        .format_target(false)
        .format_level(false)
        .filter_level(LevelFilter::Info)
        .parse_env(env)
        .init();
}

fn main() {
    setup_logger();
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 || 5 < args.len() {
        println!("Args: <fname> row <i64> bound <i64>")
    }

    let fname = &args[0];
    info!("Filename: {}", fname);

    let (sensors, beacons) = load_sensors(fname);
    trace!("{:?}", sensors);
    trace!("{:?}", beacons);

    let mut i = 1;
    while i < args.len() {
        if i + 1 >= args.len() {
            panic!("Expected argument for {}", args[i]);
        }
        let value = match args[i + 1].parse::<i64>() {
            Ok(x) => x,
            Err(e) => panic!("Could not parse int from \"{}\": {}", &args[i + 1], e),
        };

        if args[i] == "row" {
            let row = value;
            let num_no_beacon = count_not_possible_beacons(row, &sensors, &beacons);
            info!("Part 1: {} positions can't have beacon.", num_no_beacon);
        } else if args[i] == "bound" {
            let max = value;
            let bounds = Bounds {
                min_x: 0,
                max_x: max,
                min_y: 0,
                max_y: max,
            };
            match find_lost_beacon(bounds, &sensors, &beacons) {
                Some(p) => {
                    info!("Part 2: Beacon location: {:?}", p);
                    info!("Part 2: Beacon frequency: {:?}", p.x * max + p.y);
                }
                None => error!("Part 2: Could not find beacon location!"),
            };
        } else {
            println!("Unrecognized argument: \"{}\"", args[i]);
        }
        i += 2;
    }

    if args.len() < 2 {
        panic!("No row specified");
    }
}
