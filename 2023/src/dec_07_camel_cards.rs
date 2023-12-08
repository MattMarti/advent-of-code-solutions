use crate::load_file_lines;

use std::cmp::Ordering;
use std::collections::HashMap;

// TODO Lol should be an enum
mod hand_types {
    pub const FIVE_OF_KIND: usize = 7;
    pub const FOUR_OF_KIND: usize = 6;
    pub const FULL_HOUSE: usize = 5;
    pub const THREE_OF_KIND: usize = 4;
    pub const TWO_PAIR: usize = 3;
    pub const ONE_PAIR: usize = 2;
    pub const HIGH_CARD: usize = 1;
}

struct Hand {
    pub cards: Vec<char>,
    pub bid: usize,
    pub part_2: bool,
}

impl Hand {
    pub fn from_str(line: &str) -> Self {
        let split: Vec<&str> = line.split(' ').collect();
        Self {
            cards: split[0].chars().collect(),
            bid: split[1].parse().unwrap(),
            part_2: false,
        }
    }

    pub fn strength(&self) -> usize {
        let mut counts = HashMap::<char, usize>::new();
        for c in self.cards.iter() {
            if !counts.contains_key(c) {
                counts.insert(*c, 0);
            }
            *counts.get_mut(c).unwrap() += 1;
        }
        let raw_longest = counts
            .iter()
            .filter_map(|(k, v)| if *k != 'J' { Some(*v) } else { None })
            .max()
            .unwrap_or(0);
        let longest: usize = if self.part_2 {
            let j_counts: usize = if counts.contains_key(&'J') {
                *counts.get(&'J').unwrap()
            } else {
                0
            };
            raw_longest + j_counts
        } else {
            raw_longest
        };
        if longest == 5 {
            hand_types::FIVE_OF_KIND
        } else if longest == 4 {
            hand_types::FOUR_OF_KIND
        } else if longest == 2 {
            hand_types::ONE_PAIR
        } else if longest == 1 {
            hand_types::HIGH_CARD
        } else if longest == 3 && counts.len() == 2 {
            hand_types::FULL_HOUSE
        } else if longest == 3 && counts.len() == 3 {
            hand_types::THREE_OF_KIND
        } else {
            // counts.len() == 3 {
            hand_types::TWO_PAIR
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength() == other.strength()
    }
}

impl Eq for Hand {}

fn card_value(c: char) -> usize {
    match c {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 0,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0, // Bad card
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.strength().cmp(&other.strength()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (s, o) in self
                    .cards
                    .iter()
                    .map(|c| card_value(*c))
                    .zip(other.cards.iter().map(|c| card_value(*c)))
                {
                    match s.cmp(&o) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => (),
                    }
                }
                Ordering::Equal
            }
        }
    }
}

fn calc_total_winnings(hands: &[Hand]) -> usize {
    let mut bid_sum: usize = 0;
    let mut bid_multiplier = 1;
    for bid in hands.iter().map(|h| h.bid) {
        bid_sum += bid * bid_multiplier;
        bid_multiplier += 1;
    }
    bid_sum
}

pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();

    let mut hands = Vec::with_capacity(lines.len());
    for line in lines.iter() {
        hands.push(Hand::from_str(line));
    }

    hands.sort();
    if args.contains(&"debug".to_owned()) {
        println!("Sorted hands:");
        for h in hands.iter() {
            println!("{} {}", h.cards.iter().collect::<String>(), h.bid);
        }
    }

    println!("Total winnings (part 1): {}", calc_total_winnings(&hands));

    for h in hands.iter_mut() {
        h.part_2 = true;
    }
    hands.sort();

    println!("Total winnings (part 2): {}", calc_total_winnings(&hands));
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_card_types() {
        assert_eq!(
            Hand {
                cards: "AAAAA".chars().collect(),
                bid: 0
            }
            .strength(),
            hand_types::FIVE_OF_KIND
        );
        assert_eq!(
            Hand {
                cards: "AA8AA".chars().collect(),
                bid: 0
            }
            .strength(),
            hand_types::FOUR_OF_KIND
        );
        assert_eq!(
            Hand {
                cards: "23332".chars().collect(),
                bid: 0
            }
            .strength(),
            hand_types::FULL_HOUSE
        );
        assert_eq!(
            Hand {
                cards: "TTT98".chars().collect(),
                bid: 0
            }
            .strength(),
            hand_types::THREE_OF_KIND
        );
        assert_eq!(
            Hand {
                cards: "23432".chars().collect(),
                bid: 0
            }
            .strength(),
            hand_types::TWO_PAIR
        );
        assert_eq!(
            Hand {
                cards: "A23A4".chars().collect(),
                bid: 0
            }
            .strength(),
            hand_types::ONE_PAIR
        );
        assert_eq!(
            Hand {
                cards: "23456".chars().collect(),
                bid: 0
            }
            .strength(),
            hand_types::HIGH_CARD
        );
    }
}
