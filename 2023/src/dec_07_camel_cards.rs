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
        let raw_longest: usize = if self.part_2 {
            *counts
                .iter()
                .filter_map(|(k, v)| match k {
                    'J' => None,
                    _ => Some(v),
                })
                .max()
                .unwrap_or(&0)
        } else {
            *counts.values().max().unwrap()
        };
        let j_counts = *counts.get(&'J').unwrap_or(&0);
        let longest = if self.part_2 {
            raw_longest + j_counts
        } else {
            raw_longest
        };
        let num_unique = if self.part_2 {
            counts.len() - if j_counts > 0 { 1 } else { 0 }
        } else {
            counts.len()
        };
        if longest == 5 {
            hand_types::FIVE_OF_KIND
        } else if longest == 4 {
            hand_types::FOUR_OF_KIND
        } else if longest == 3 && num_unique == 2 {
            hand_types::FULL_HOUSE
        } else if longest == 3 && num_unique == 3 {
            hand_types::THREE_OF_KIND
        } else if longest == 2 && num_unique == 3 {
            hand_types::TWO_PAIR
        } else if longest == 2 && num_unique == 4 {
            hand_types::ONE_PAIR
        } else if longest == 1 {
            hand_types::HIGH_CARD
        } else {
            panic!("Unhandled case! {}", self.cards.iter().collect::<String>());
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength() == other.strength()
    }
}

impl Eq for Hand {}

fn card_value(c: char, is_part_2: bool) -> usize {
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
        'J' => {
            if is_part_2 {
                0
            } else {
                10
            }
        }
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
                    .map(|c| card_value(*c, self.part_2))
                    .zip(other.cards.iter().map(|c| card_value(*c, self.part_2)))
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
    use rstest::rstest;

    #[rstest]
    #[case("AAAAA", hand_types::FIVE_OF_KIND)]
    #[case("AA8AA", hand_types::FOUR_OF_KIND)]
    #[case("23332", hand_types::FULL_HOUSE)]
    #[case("TTT98", hand_types::THREE_OF_KIND)]
    #[case("23432", hand_types::TWO_PAIR)]
    #[case("A23A4", hand_types::ONE_PAIR)]
    #[case("23456", hand_types::HIGH_CARD)]
    #[case("JJ7K7", hand_types::TWO_PAIR)]
    fn test_card_types(#[case] hand: &str, #[case] hand_type: usize) {
        assert_eq!(
            Hand {
                cards: hand.chars().collect(),
                bid: 0,
                part_2: false,
            }
            .strength(),
            hand_type
        );
    }

    #[rstest]
    #[case("QJJQQ", hand_types::FIVE_OF_KIND)]
    #[case("QJJQ2", hand_types::FOUR_OF_KIND)]
    #[case("AAAAA", hand_types::FIVE_OF_KIND)]
    #[case("AAAAJ", hand_types::FIVE_OF_KIND)]
    #[case("AAAJJ", hand_types::FIVE_OF_KIND)]
    #[case("AAJJJ", hand_types::FIVE_OF_KIND)]
    #[case("AJJJJ", hand_types::FIVE_OF_KIND)]
    #[case("JJJJJ", hand_types::FIVE_OF_KIND)]
    #[case("AA8AA", hand_types::FOUR_OF_KIND)]
    #[case("AA8JA", hand_types::FOUR_OF_KIND)]
    #[case("AA8JJ", hand_types::FOUR_OF_KIND)]
    #[case("23332", hand_types::FULL_HOUSE)]
    #[case("23J32", hand_types::FULL_HOUSE)]
    #[case("2333J", hand_types::FOUR_OF_KIND)]
    #[case("JJ332", hand_types::FOUR_OF_KIND)]
    #[case("TTT98", hand_types::THREE_OF_KIND)]
    #[case("TTJ98", hand_types::THREE_OF_KIND)]
    #[case("TTTJ8", hand_types::FOUR_OF_KIND)]
    #[case("23432", hand_types::TWO_PAIR)]
    #[case("J3432", hand_types::THREE_OF_KIND)]
    #[case("23J32", hand_types::FULL_HOUSE)]
    #[case("A23A4", hand_types::ONE_PAIR)]
    #[case("A23J4", hand_types::ONE_PAIR)]
    #[case("A23AJ", hand_types::THREE_OF_KIND)]
    #[case("23456", hand_types::HIGH_CARD)]
    #[case("J3456", hand_types::ONE_PAIR)]
    #[case("JJ7K7", hand_types::FOUR_OF_KIND)]
    fn test_card_types_part_2(#[case] hand: &str, #[case] hand_type: usize) {
        assert_eq!(
            Hand {
                cards: hand.chars().collect(),
                bid: 0,
                part_2: true,
            }
            .strength(),
            hand_type
        );
    }

    #[rstest]
    #[case("JKKK2", "QQQQ2")]
    fn test_card_order_part_2(#[case] lesser: &str, #[case] greater: &str) {
        assert!(
            Hand {
                cards: lesser.chars().collect(),
                bid: 0,
                part_2: true,
            } < Hand {
                cards: greater.chars().collect(),
                bid: 0,
                part_2: true,
            }
        );
    }
}
