use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::marker::Copy;

#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

fn parse_line(line: String) -> (Hand, Hand) {
    use Hand::*;
    let hands: Vec<&str> = line.split(' ').collect();
    let opponent_hand = match hands[0] {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        x => panic!("Inavlid strategy value: {}", x),
    };
    let player_hand = match hands[1] {
        "X" => match opponent_hand {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
        "Y" => opponent_hand,
        "Z" => match opponent_hand {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
        x => panic!("Inavlid strategy value: {}", x),
    };
    (opponent_hand, player_hand)
}

fn points_for_hand(hand: &Hand) -> u32 {
    use Hand::*;
    match hand {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn points_for_outcome(opponent_hand: &Hand, player_hand: &Hand) -> u32 {
    use Hand::*;
    match opponent_hand {
        Rock => match player_hand {
            Rock => 3,
            Paper => 6,
            Scissors => 0,
        },
        Paper => match player_hand {
            Rock => 0,
            Paper => 3,
            Scissors => 6,
        },
        Scissors => match player_hand {
            Rock => 6,
            Paper => 0,
            Scissors => 3,
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
