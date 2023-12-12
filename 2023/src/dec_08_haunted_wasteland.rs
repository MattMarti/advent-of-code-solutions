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

fn lowest_common_denom(nums: &[usize]) -> usize {
    let mut prod = nums[0];
    for n in nums.iter().skip(1) {
        if prod % n != 0 {
            prod *= n
        }
    }
    prod
}

fn count_steps_to_z(directions: &[Direction], nodes: &[Node], start: &str) -> usize {
    let mut num_steps = 0;
    let mut i = 0;
    let mut curr_name = start;
    while !curr_name.ends_with('Z') {
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
    num_steps
}

pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();
    let debug_mode = args.contains(&"debug".to_owned());
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

    let start_names: Vec<&String> = nodes
        .iter()
        .filter(|n| n.name.ends_with('A'))
        .map(|n| &n.name)
        .collect();

    if start_names.contains(&&String::from("AAA")) {
        println!(
            "Number of steps (part 1): {}",
            count_steps_to_z(&directions, &nodes, "AAA")
        );
    }

    // Assume that the first z value is the only one in the cycle
    // TODO Visualize this
    if debug_mode {
        println!("Num Steps:");
    }
    let mut num_steps = vec![0; start_names.len()];
    for (name, steps) in start_names.iter().zip(num_steps.iter_mut()) {
        println!("Checking {}", name);
        *steps = count_steps_to_z(&directions, &nodes, name);
        if debug_mode {
            println!("- {name}: {steps}");
        }
    }
    println!(
        "Number of steps until all z's are reached (part 2): {}",
        lowest_common_denom(&num_steps)
    );
}
