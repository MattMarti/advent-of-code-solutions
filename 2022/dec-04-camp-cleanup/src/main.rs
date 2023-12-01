use std::cmp;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Default)]
struct Range {
    min: u32,
    max: u32,
}

impl Range {
    pub fn from_str(s: &str) -> Self {
        let binding = s.to_string();
        let minmax: Vec<&str> = binding.split('-').collect();
        Self {
            min: minmax[0].to_string().parse::<u32>().unwrap(),
            max: minmax[1].to_string().parse::<u32>().unwrap(),
        }
    }

    pub fn contains(&self, cmp: &Range) -> bool {
        self.min <= cmp.min && cmp.max <= self.max
    }

    pub fn len(&self) -> u32 {
        self.max - self.min + 1
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        let collective_len = self.len() + other.len();
        let total_len = cmp::max(self.max, other.max) - cmp::min(self.min, other.min) + 1;
        total_len < collective_len
    }
}

fn get_ranges(line: &str) -> (Range, Range) {
    let leftright: Vec<&str> = line.split(',').collect();
    (Range::from_str(leftright[0]), Range::from_str(leftright[1]))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut num_contains = 0;
    let mut num_overlaps = 0;
    for read_line in reader.lines() {
        let line = read_line?;
        let (left, right) = get_ranges(&line);
        if left.contains(&right) || right.contains(&left) {
            num_contains += 1;
        }
        if left.overlaps(&right) {
            num_overlaps += 1;
        }
    }
    println!("Number of strict subset ranges: {}", num_contains);
    println!("Number of overlapping ranges: {}", num_overlaps);
    Ok(())
}
