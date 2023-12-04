use crate::load_file_lines;

use std::collections::HashSet;

extern crate colored;
use colored::*;

struct Card {
    winning_nums: HashSet<i32>,
    user_nums: HashSet<i32>,
}

impl Card {
    pub fn from_str(s: &str) -> Self {
        let win_num_start = s.chars().position(|c| c == ':').unwrap() + 2;
        let win_num_end = s.chars().position(|c| c == '|').unwrap() - 2;
        let user_num_start = s.chars().position(|c| c == '|').unwrap() + 1;
        Self {
            winning_nums: s.chars().into_iter()
                .take(win_num_end)
                .skip(win_num_start)
                .collect::<String>()
                .split(" ")
                .filter_map(|n| n.parse().ok())
                .collect(),
            user_nums: s.chars().into_iter()
                .skip(user_num_start)
                .collect::<String>()
                .split(" ")
                .filter_map(|n| n.parse().ok())
                .collect(),
        }
    }
}


pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();
    let cards: Vec<Card> = lines.iter().map(|s| Card::from_str(&s)).collect();

    let mut total_score = 0;
    for c in cards.iter() {
        let mut count = 0;
        for win_num in c.winning_nums.iter() {
            if c.user_nums.contains(win_num) {
                count += 1;
            }
        }
    }
    println!("{} {}", "merry".red(), "christmas!".green());
}
