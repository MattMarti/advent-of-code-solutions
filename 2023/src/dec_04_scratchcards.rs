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
        let win_num_start = s.chars().position(|c| c == ':').unwrap() + 1;
        let win_num_end = s.chars().position(|c| c == '|').unwrap() - 1;
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
    for (i, c) in cards.iter().enumerate() {
        let mut score = 0;
        for win_num in c.winning_nums.iter() {
            if c.user_nums.contains(win_num) {
                if score == 0 { score = 1;}
                else { score *= 2;}
            }
        }
        println!("Card {}: {}", i + 1, score);
        total_score += score;
    }
    println!("Card score: {total_score}");
    println!("{} {}", "merry".red(), "christmas!".green());
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "Card 1: 3 6 4 | 5 82 23 3 54 6";
        let card = Card::from_str(&line);
        assert!(card.winning_nums.contains(&3));
        assert!(card.winning_nums.contains(&6));
        assert!(card.winning_nums.contains(&4));
        assert!(card.user_nums.contains(&5));
        assert!(card.user_nums.contains(&82));
        assert!(card.user_nums.contains(&23));
        assert!(card.user_nums.contains(&3));
        assert!(card.user_nums.contains(&54));
        assert!(card.user_nums.contains(&6));
    }
}
