use crate::load_file_lines;

use lazy_static::lazy_static;
use regex::Regex;

enum Direction {
    Left,
    Right,
}

struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    pub fn from_str(line: &str) -> Result<Self, String> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?<name>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)")
                    .unwrap();
        };

        if let Some(cap) = RE.captures(line) {
            Ok(Self {
                name: cap["name"].to_owned(),
                left: cap["left"].to_owned(),
                right: cap["right"].to_owned(),
            })
        } else {
            Err(format!("Failed to parse map declaration from: {}", line))
        }
    }
}

pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();

    let directions: Vec<Direction> = lines[0]
        .chars()
        .map(|c| {
            if c == 'R' {
                Direction::Right
            } else {
                Direction::Left
            }
        })
        .collect();

    let nodes: Vec<Node> = lines
        .iter()
        .skip(2)
        .filter_map(|line| Node::from_str(line).ok())
        .collect();

    let mut num_steps = 0;
    let mut i = 0;
    let mut curr_name = "AAA";
    while curr_name != "ZZZ" {
        if i >= directions.len() {
            i = 0;
        }
        let curr_node = nodes.iter().find(|n| n.name == curr_name).unwrap();
        curr_name = match directions[i] {
            Direction::Left => &curr_node.left,
            Direction::Right => &curr_node.right,
        };
        i += 1;
        num_steps += 1;
    }
    println!("Number of steps (part 1): {num_steps}");
}
