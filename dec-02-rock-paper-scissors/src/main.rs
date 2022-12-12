use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

enum Hand {
    Rock,
    Paper,
    Scissors,
}

fn parse_line(line: String) -> (Hand, Hand) {
    let hands: Vec<&str> = line.split(' ').collect();
    let opponent_hand = match hands[0] {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => panic!("Inavlid strategy value: {}", hands[1]),
    };
    let player_hand = match hands[1] {
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissors,
        _ => panic!("Inavlid strategy value: {}", hands[1]),
    };
    (opponent_hand, player_hand)
}

fn points_for_hand(hand: &Hand) -> u32 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn points_for_outcome(opponent_hand: &Hand, player_hand: &Hand) -> u32 {
    match opponent_hand {
        Hand::Rock => match player_hand {
            Hand::Rock => 3,
            Hand::Paper => 6,
            Hand::Scissors => 0,
        },
        Hand::Paper => match player_hand {
            Hand::Rock => 0,
            Hand::Paper => 3,
            Hand::Scissors => 6,
        },
        Hand::Scissors => match player_hand {
            Hand::Rock => 6,
            Hand::Paper => 0,
            Hand::Scissors => 3,
        },
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut total_points: u32 = 0;
    for read_line in reader.lines() {
        let line = read_line?;
        let (opponent_hand, player_hand) = parse_line(line);
        let mut round_points = 0;
        round_points += points_for_hand(&player_hand);
        round_points += points_for_outcome(&opponent_hand, &player_hand);
        total_points += round_points;
    }
    println!("Total points: {}", total_points);
    Ok(())
}
