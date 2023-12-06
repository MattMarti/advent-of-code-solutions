use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

use crate::load_file_lines;

struct AlmanacRange {
    src: usize,
    dest: usize,
    range: usize,
}

impl AlmanacRange {
    pub fn from_str(line: &str) -> Self {
        let values: Vec<&str> = line.split(' ').collect();
        Self {
            dest: values[0].parse().unwrap(),
            src: values[1].parse().unwrap(),
            range: values[2].parse().unwrap(),
        }
    }
}

impl fmt::Debug for AlmanacRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Range")
            .field("d", &self.dest)
            .field("s", &self.src)
            .field("r", &self.range)
            .finish()
    }
}

#[derive(Debug)]
struct AlmanacMap {
    src_type: String,
    dest_type: String,
    ranges: Vec<AlmanacRange>,
}

impl AlmanacMap {
    pub fn from_decl(range_decl: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?<src_type>[A-Za-z]+)-to-(?<dest_type>[A-Za-z]+) map:$").unwrap();
        };
        if let Some(cap) = RE.captures(range_decl) {
            Some(Self {
                src_type: cap["src_type"].to_owned(),
                dest_type: cap["dest_type"].to_owned(),
                ranges: Vec::new(),
            })
        } else {
            println!("Failed to parse map declaration from: {range_decl}");
            None
        }
    }

    pub fn transform(&self, n: usize) -> usize {
        for r in self.ranges.iter() {
            if r.src <= n && n < r.src + r.range {
                return n - r.src + r.dest;
            }
        }
        n
    }
}

fn parse_maps(lines: &[String]) -> Vec<AlmanacMap> {
    let mut maps = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];
        if !line.contains(" map:") {
            i += 1;
            continue;
        }
        let mut map = AlmanacMap::from_decl(&lines[i]).unwrap();
        let mut j = i + 1;
        while j < lines.len() {
            let line = &lines[j];
            if line.is_empty() {
                i = j;
                break;
            }
            map.ranges.push(AlmanacRange::from_str(line));
            j += 1;
        }
        maps.push(map);
        i += 1;
    }
    maps
}

fn parse_seeds(line: &str) -> Vec<usize> {
    let start_idx = line.chars().position(|c| c == ':').unwrap() + 1;
    line.chars()
        .skip(start_idx)
        .collect::<String>()
        .split(' ')
        .filter_map(|n| n.parse().ok())
        .collect()
}

pub fn run(args: &[String]) {
    let verbose = args.contains(&String::from("debug"));
    let lines = load_file_lines(&args[0]).unwrap();

    let seeds = parse_seeds(&lines[0]);
    let maps = parse_maps(&lines[2..]);
    if verbose {
        for m in maps.iter() {
            println!("{:?}", m);
        }
    }

    println!("Seeds:");
    println!("{:?}", seeds);
    let mut values = seeds.clone();
    for m in maps.iter() {
        // TODO Assuming map traversal is in order
        println!("{}:", m.dest_type);
        values.iter_mut().for_each(|v| *v = m.transform(*v));
        println!("{:?}", values);
    }
    println!("Lowest location: {}", values.iter().min().unwrap());
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_map_delcaration() {
        let line = "source-to-dest map:";
        let map = AlmanacMap::from_decl(&line).unwrap();
        assert_eq!(map.src_type, "source");
        assert_eq!(map.dest_type, "dest");
    }
}
