use core::iter::zip;
use lazy_static::lazy_static;
use regex::Regex;

use crate::load_file_lines;

#[derive(Default, Debug)]
struct CubeCounts {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeCounts {
    pub fn from_str(line: &str) -> Self {
        lazy_static! {
            static ref RE_COLORS: Regex =
                Regex::new(r"(?<count>\d+) (?<color>((blue)|(red)|(green)))").unwrap();
        };
        let mut obj = Self::default();
        for cap in RE_COLORS.captures_iter(line) {
            let color = cap["color"].to_owned();
            let count = cap["count"].parse::<_>().unwrap();
            match color.as_str() {
                "red" => obj.red = count,
                "green" => obj.green = count,
                "blue" => obj.blue = count,
                _ => println!("WARNING: Bad color {}", color),
            }
        }
        obj
    }
}

// game > round > set
fn load_game_results(path: &str) -> Vec<Vec<CubeCounts>> {
    let mut game_results = Vec::<Vec<CubeCounts>>::new();
    for line in load_file_lines(path).unwrap().iter() {
        let mut round_results = Vec::<CubeCounts>::new();

        let mut split_idxs = Vec::<_>::new();
        for (i, c) in line.chars().enumerate() {
            if c == ':' || c == ';' {
                split_idxs.push(i);
            }
        }
        let num_chars = line.chars().count();
        split_idxs.push(num_chars);
        for (&start, &end) in zip(split_idxs.iter().take(num_chars), split_idxs.iter().skip(1)) {
            let substr = line.chars().take(end).skip(start).collect::<String>();
            round_results.push(CubeCounts::from_str(&substr));
        }
        game_results.push(round_results);
    }
    game_results
}

fn is_possible(candidate: &CubeCounts, actual: &CubeCounts) -> bool {
    actual.red <= candidate.red && actual.green <= candidate.green && actual.blue <= candidate.blue
}

pub fn run(args: &[String]) {
    let game_results = load_game_results(&args[0]);

    let part_1_candidate = CubeCounts {
        red: 12,
        green: 13,
        blue: 14,
    };

    if args.contains(&String::from("debug")) {
        for (i, round) in game_results.iter().enumerate() {
            println!("Game {i}:");
            for res in round.iter() {
                print!("- {:?}", res);
                if !is_possible(&part_1_candidate, res) {
                    print!(" - IMPOSSIBLE");
                }
                println!();
            }
        }
    }

    // Find impossible games
    let mut part_1_checksum = 0;
    for (i, round) in game_results.iter().enumerate() {
        if !round.iter().any(|res| !is_possible(&part_1_candidate, res)) {
            part_1_checksum += i + 1;
        }
    }
    println!("Part 1 checksum: {}", part_1_checksum);

    // Find fewest cubes
    let mut part_2_checksum = 0;
    for round in game_results.iter() {
        let mut fewest_counts = CubeCounts {
            red: 0,
            green: 0,
            blue: 0,
        };
        for result in round {
            fewest_counts.red = usize::max(fewest_counts.red, result.red);
            fewest_counts.green = usize::max(fewest_counts.green, result.green);
            fewest_counts.blue = usize::max(fewest_counts.blue, result.blue);
        }
        let power = fewest_counts.red * fewest_counts.green * fewest_counts.blue;
        part_2_checksum += power;
    }
    println!("Part 2 checksum: {}", part_2_checksum);
}
