use crate::load_file_lines;

extern crate colored;
use colored::*;

struct Card {
    winning_nums: Vec<i32>,
    user_nums: Vec<i32>,
}

impl Card {
    pub fn from_str(s: &str) -> Self {
        let win_num_start = s.chars().position(|c| c == ':').unwrap() + 1;
        let win_num_end = s.chars().position(|c| c == '|').unwrap() - 1;
        let user_num_start = s.chars().position(|c| c == '|').unwrap() + 1;
        Self {
            winning_nums: s
                .chars()
                .take(win_num_end)
                .skip(win_num_start)
                .collect::<String>()
                .split(' ')
                .filter_map(|n| n.parse().ok())
                .collect(),
            user_nums: s
                .chars()
                .skip(user_num_start)
                .collect::<String>()
                .split(' ')
                .filter_map(|n| n.parse().ok())
                .collect(),
        }
    }
}

fn print_first_part(line: &str) -> Result<(), &'static str> {
    if let Some(idx) = line.chars().position(|c| c == ':') {
        print!("{}: ", line.chars().take(idx).collect::<String>());
    } else {
        return Err("Parse error: Line is missing colon.");
    }
    Ok(())
}

pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();
    let mut total_score = 0;
    for (i, line) in lines.iter().enumerate() {
        // Print first half
        if let Err(msg) = print_first_part(&line) {
            println!("Error on line {i}: {}", msg);
            continue;
        }

        // Print winning numbers
        let card = Card::from_str(line);
        let mut score = 0;
        let mut max_score = 1;
        for win_num in card.winning_nums.iter() {
            let output = format!("{:2} ", win_num);
            max_score *= 2;
            if card.user_nums.contains(win_num) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
                print!("{}", output.green());
            } else {
                print!("{}", output);
            }
        }
        max_score /= 2;
        if score == max_score {
            print!("\r");
            let _ = print_first_part(&line).unwrap();
            for num in card.winning_nums.iter() {
                let s = format!("{:2} ", num);
                print!("{}", s.blue());
            }
        }
        total_score += score;

        // Print user numbers
        print!("|");
        for user_num in card.user_nums.iter() {
            let output = format!(" {:2}", user_num);
            if card.winning_nums.contains(user_num) {
                print!("{}", output.green());
            } else {
                print!("{}", output);
            }
        }

        // Finish
        println!(" ({score})");
    }

    println!("Total score: {total_score}");
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
