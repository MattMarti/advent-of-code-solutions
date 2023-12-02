use lazy_static::lazy_static;
use regex::Regex;

use crate::load_file_lines;


#[derive(Default)]
struct CubeCounts {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeCounts {
    pub fn from_line(line: &str) -> Self {
        lazy_static! {
            static ref RE_COLORS: Regex = Regex::new(
                r"(?<count>\d+) (?<color>((blue)|(red)|(green)))"
            ).unwrap();
        };
        let mut obj = Self::default();
        for cap in RE_COLORS.captures_iter(line) {
            let color = cap["color"].to_owned();
            let count = cap["count"].parse::<_>().unwrap();
            match color.as_str() {
                "red" => obj.red = count,
                "green" => obj.green = count,
                "blue" => obj.blue = count,
                _ => println!("WARNING: Bad color {}", color)
            }
        }
        obj
    }
}

fn load_game_results(path: &str) -> Vec<Vec<CubeCounts>> {
    let mut game_results = Vec::<Vec::<CubeCounts>>::new();
    for line in load_file_lines(path).unwrap().iter() {
        let mut round_results = Vec::<CubeCounts>::new();
            // Find where each round begins and ends
            // Make counts for each round
            game_results.push(round_results);
    }
    game_results
}


pub fn run(args: &[String]) {
    let game_results = load_game_results(&args[0]);
}
