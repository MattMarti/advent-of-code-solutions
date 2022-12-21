use env_logger;
use lazy_static::lazy_static;
use log::LevelFilter;
use log::{debug, info, trace};
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Bounds {
    pub fn from_points(points: &[Point]) -> Self {
        let mut min_x: i32 = std::i32::MAX;
        let mut max_x: i32 = std::i32::MIN;
        let mut min_y: i32 = std::i32::MAX;
        let mut max_y: i32 = std::i32::MIN;
        for p in points.iter() {
            if p.x < min_x {
                min_x = p.x;
            }
            if max_x < p.x {
                max_x = p.x;
            }
            if p.y < min_y {
                min_y = p.y;
            }
            if max_y < p.y {
                max_y = p.y;
            }
        }
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

#[derive(Debug)]
struct Sensor {
    loc: Point,
    range: i32,
}

impl Sensor {
    pub fn min_x(&self, row: i32) -> i32 {
        self.loc.x - self.row_range(row)
    }

    pub fn max_x(&self, row: i32) -> i32 {
        self.loc.x + self.row_range(row)
    }

    pub fn row_range(&self, row: i32) -> i32 {
        self.range - (row - self.loc.y).abs()
    }
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i32,
    end: i32,
}

fn in_range(start: i32, x: i32, end: i32) -> bool {
    start <= x && x <= end
}

impl Range {
    pub fn from_sensor_row(sensor: &Sensor, row: i32) -> Option<Self> {
        if sensor.row_range(row) < 0 {
            return None;
        }
        Some(Self {
            start: sensor.min_x(row),
            end: sensor.max_x(row),
        })
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        in_range(self.start, other.start, self.end) || in_range(self.start, other.end, self.end)
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

    for range in ranges.iter().skip(1) {
        let mut found_overlap = false;
        for overlap in overlapped_ranges.iter_mut() {
            match Range::from_overlapping(range, overlap) {
                Some(r) => {
                    *overlap = r.clone();
                    found_overlap = true;
                }
                None => (),
            };
        }
        if !found_overlap {
            overlapped_ranges.push(*range);
        }
    }
    if overlapped_ranges.len() == ranges.len() {
        return overlapped_ranges;
    }
    get_overlapping_ranges(&overlapped_ranges)
}

fn count_no_possible_beacons(
    row: i32,
    sensors: &[Sensor],
    beacons: &[Point],
    bounds: &Bounds,
) -> usize {
    let mut ranges = Vec::<Range>::new();
    for sensor in sensors.iter() {
        match Range::from_sensor_row(&sensor, row) {
            Some(r) => ranges.push(r),
            None => (),
        };
    }
    // Get all the overlapping ranges
    let overlaps = get_overlapping_ranges(&ranges);
    trace!("Overlapping ranges: {:?}", overlaps);

    // Subtract overlapping range lengths from max_x
    let mut num_points: usize = 0;
    for r in overlaps.iter() {
        num_points += (r.end - r.start) as usize + 1;
    }

    // Subtract beacon positions outside of ranges
    for b in beacons.iter() {
        if b.y != row {
            continue;
        }
        let mut outside_ranges = true;
        for r in overlaps.iter() {
            outside_ranges &= !in_range(r.start, b.x, r.end);
        }
        if !outside_ranges {
            trace!("Beacon contained in row");
            num_points -= 1;
        }
    }
    num_points
}

fn parse_coord(s: &str) -> Point {
    lazy_static! {
        static ref POINT_RE: Regex = Regex::new(r"-?\d+").unwrap();
    }
    let mut matches = POINT_RE.find_iter(s);
    Point {
        x: matches.next().unwrap().as_str().parse::<i32>().unwrap(),
        y: matches.next().unwrap().as_str().parse::<i32>().unwrap(),
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
    let fname = &args[0];
    info!("Filename: {}", fname);

    let (sensors, beacons) = load_sensors(fname);
    trace!("{:?}", sensors);

    let bounds = Bounds::from_points(&beacons);
    trace!("Min x: {}", bounds.min_x);
    trace!("Max x: {}", bounds.max_x);
    trace!("Min y: {}", bounds.min_y);
    trace!("Max y: {}", bounds.max_y);

    if args.len() < 2 {
        panic!("No row specified");
    }
    let row = match args[1].parse::<i32>() {
        Ok(x) => x,
        Err(e) => panic!("Could not parse row \"{}\": {}", &args[1], e),
    };

    let num_no_beacon = count_no_possible_beacons(row, &sensors, &beacons, &bounds);
    info!("Part 1: {} positions can't have beacon.", num_no_beacon);
}
