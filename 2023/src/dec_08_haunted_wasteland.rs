use std::collections::{HashMap, HashSet};

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
            static ref RE: Regex = Regex::new(
                r"(?<name>[A-Z0-9]{3}) = \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)"
            )
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

fn count_steps_to_z<'n>(
    directions: &[Direction],
    nodes: &'n [Node],
    start: &'n str,
) -> HashMap<&'n str, usize> {
    let mut i = 0;
    let mut curr_name = start;
    let mut step_counts = HashMap::new();
    let mut num_steps = 0;
    loop {
        if i >= directions.len() {
            i = 0;
        }
        let curr_node = nodes.iter().find(|n| n.name == curr_name).unwrap();
        curr_name = match directions[i] {
            Direction::Left => &curr_node.left,
            Direction::Right => &curr_node.right,
        };
        if curr_name.ends_with('Z') {
            if step_counts.contains_key(curr_name) {
                break;
            }
            step_counts.insert(curr_name, num_steps + 1);
        }
        i += 1;
        num_steps += 1;
    }
    step_counts
}

fn get_factors(x: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let mut x_value = x;
    // TODO Maybe there's a better way to get multiples
    for i in 2..(x + 1) {
        if x_value % i == 0 {
            factors.push(i);
            x_value /= i;
        }
    }
    factors
}

fn get_least_common_multiple(values: &[usize]) -> usize {
    let mut gc_factors = Vec::<usize>::new();
    for (a, b) in values
        .iter()
        .take(values.len() - 1)
        .zip(values.iter().skip(1))
    {
        gc_factors.push(
            **get_factors(*a)
                .iter()
                .collect::<HashSet<&usize>>()
                .union(&get_factors(*b).iter().collect::<HashSet<&usize>>())
                .max()
                .unwrap_or(&&1),
        );
    }

    let mut lcm = values[0];
    for (x, gcf) in values.iter().skip(1).zip(gc_factors.iter()) {
        lcm *= x / gcf;
    }
    lcm
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

    let start_names: Vec<&str> = nodes
        .iter()
        .filter(|n| n.name.ends_with('A'))
        .map(|n| n.name.as_str())
        .collect();

    if start_names.contains(&"AAA") {
        println!(
            "Number of steps (part 1): {}",
            count_steps_to_z(&directions, &nodes, "AAA")
                .get("ZZZ")
                .unwrap()
        );
    }

    let path_counts: Vec<HashMap<&str, usize>> = start_names
        .iter()
        .map(|n| count_steps_to_z(&directions, &nodes, n))
        .collect();

    for (start, map) in start_names.iter().zip(path_counts.iter()) {
        println!("{}:", start);
        for (k, v) in map.iter() {
            println!("- {}: {}", k, v);
        }
    }

    // Assuming that each path has only one end
    let counts: Vec<usize> = path_counts
        .iter()
        .map(|map| *map.values().next().unwrap())
        .collect();

    let lcm = get_least_common_multiple(&counts);
    println!(
        "Number of steps until all z's are reached (part 2): {}",
        lcm
    );
}
