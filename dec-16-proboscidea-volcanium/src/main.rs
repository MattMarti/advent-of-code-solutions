use lazy_static::lazy_static;
use log::LevelFilter;
use log::{debug, error, info, trace};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone)]
struct Node {
    pub rate: usize,
    pub is_open: bool,
    pub links: Vec<String>,
}

impl Node {
    pub fn open(&mut self) {
        self.is_open = true;
    }
}

// Ought to return none if these unrwaps fail instead of `unwrap()`
fn parse_node(s: &str) -> Option<(String, Node)> {
    let ssep = s.split(' ').collect::<Vec<&str>>();
    let name = ssep[1].to_string();

    lazy_static! {
        static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
    }
    let rate = NUMBER
        .find(ssep[4])
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    lazy_static! {
        static ref NAME_LIST: Regex = Regex::new(r"([A-Z]{2}, )?[A-Z]{2}$").unwrap();
    }
    let links = NAME_LIST
        .find(s)
        .unwrap()
        .as_str()
        .split(", ")
        .map(str::to_string)
        .collect::<Vec<String>>();

    Some((
        name,
        Node {
            rate,
            is_open: false,
            links,
        },
    ))
}

#[derive(Debug)]
struct World {
    pub time_left: usize,
    pub current_node: String,
    pub map: HashMap<String, Node>,
}

impl World {
    pub fn new() -> Self {
        Self {
            time_left: 30,
            current_node: "AA".to_string(),
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, node: &Node) {
        self.map.insert(name.to_string(), node.clone());
    }

    // FN: Get next best node to open
    // From remaining time left, and time to get to other nodes, get total pressure released
    // Move there

    pub fn move_to_node(&mut self, target_name: &str) -> bool {
        let mut path = self.dijkstra_to_node(target_name);
        if self.time_left < path.len() {
            return false;
        }
        self.current_node = target_name.to_string();
        self.time_left -= path.len();
        true
    }

    fn dijkstra_to_node(&self, target_name: &str) -> Vec<String> {
        let mut all_nodes = HashMap::<String, DijkstraNode>::new();
        for key in self.map.keys() {
            println!("Adding {}", key);
            all_nodes.insert(key.clone(), DijkstraNode::new());
        }
        let mut nodes_to_check: Vec<String> = vec![self.current_node.clone()];
        while !nodes_to_check.is_empty() {
            nodes_to_check.sort();

            let current_loc = nodes_to_check.pop().unwrap();
            println!("Checking {}", current_loc);
            let current_node = all_nodes.get(&current_loc).unwrap().clone();

            for neighbor_name in self.map.get(&current_loc).unwrap().links.iter() {
                println!("- Neighbor {}", neighbor_name);
                let mut neighbor = all_nodes.get_mut(neighbor_name).unwrap();
                const MOVEMENT_COST: usize = 1;
                if neighbor.parent == None
                    || current_node.local_goal < neighbor.local_goal + MOVEMENT_COST
                {
                    println!("- - Setting parent: {}", current_loc);
                    neighbor.parent = Some(current_loc.clone());
                    neighbor.local_goal = current_node.local_goal + MOVEMENT_COST;
                    // No global goal calc b/c using Dijkstra's Algorithm
                }
                if !neighbor.visited {
                    neighbor.visited = true;
                    nodes_to_check.push(neighbor_name.clone());
                }
            }
        }
        let mut path = Vec::<String>::new();
        path.push(all_nodes.get(target_name).unwrap().clone().parent.unwrap());
        while path.last().unwrap() != &self.current_node {
            let current = path.last().unwrap();
            match &all_nodes.get(current).unwrap().parent {
                Some(parent) => path.push(parent.clone()),
                None => break,
            }
        }
        path.reverse();
        path
    }
}

#[derive(Clone)]
struct DijkstraNode {
    pub local_goal: usize,
    pub visited: bool,
    pub parent: Option<String>,
}

impl DijkstraNode {
    pub fn new() -> Self {
        Self {
            local_goal: 0,
            visited: false,
            parent: None,
        }
    }
}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.local_goal.partial_cmp(&other.local_goal)
    }
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(x) if x == Ordering::Equal => true,
            Some(_) => false,
            None => false,
        }
    }
}

// Functions that load data from files should return results.
fn load_nodes(fname: &str) -> Option<World> {
    let file = match File::open(fname) {
        Ok(f) => f,
        Err(err) => panic!("Could not open file \"{}\": {}", fname, err),
    };
    let reader = BufReader::new(file);
    let mut world = World::new();
    for read_line in reader.lines() {
        let line = read_line.unwrap();
        if line.is_empty() {
            continue;
        }
        match parse_node(&line) {
            Some((name, node)) => world.add(name.as_str(), &node),
            None => return None,
        };
    }
    Some(world)
}

fn setup_logger() {
    let env = env_logger::Env::new().filter("RUST_LOG");
    env_logger::builder()
        .format_timestamp(None)
        .format_indent(None)
        .format_target(false)
        .format_level(false)
        .filter_level(LevelFilter::Info)
        .parse_env(env)
        .init();
}

fn main() {
    setup_logger();
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 || 5 < args.len() {
        println!("Args: <fname> row <i64> bound <i64>");
        return;
    }

    let fname = &args[0];
    info!("Filename: {}", fname);

    let world = load_nodes(fname);
    trace!("Loaded {:?}", world);
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! string_vec {
        ($($x:expr),*) => (vec![$($x.to_string()), *]);
    }

    #[test]
    fn test_dijkstra_movement() {
        // Set up a graph with two paths
        //
        //   B - C - D
        //  /    /     \
        // A    G - H - I
        //  \  /       /
        //    E ---- F
        //
        let mut world = World::new();
        world.current_node = "A".to_string();
        world.add(
            "A",
            &Node {
                rate: 0,
                is_open: false,
                links: string_vec!["B", "E"],
            },
        );
        world.add(
            "B",
            &Node {
                rate: 0,
                is_open: false,
                links: string_vec!["A", "C"],
            },
        );
        world.add(
            "C",
            &Node {
                rate: 0,
                is_open: false,
                links: string_vec!["B", "D", "G"],
            },
        );
        world.add(
            "D",
            &Node {
                rate: 0,
                is_open: false,
                links: string_vec!["C", "I"],
            },
        );
        world.add(
            "E",
            &Node {
                rate: 0,
                is_open: false,
                links: string_vec!["A", "G", "F"],
            },
        );
        world.add(
            "F",
            &Node {
                rate: 0,
                is_open: false,
                links: string_vec!["E", "I"],
            },
        );
        world.add(
            "G",
            &Node {
                rate: 0,
                is_open: false,
                links: string_vec!["E", "C", "H"],
            },
        );
        world.add(
            "H",
            &Node {
                rate: 0,
                is_open: false,
                links: string_vec!["G", "I"],
            },
        );
        world.add(
            "I",
            &Node {
                rate: 0,
                is_open: false,
                links: string_vec!["D", "F", "H"],
            },
        );

        // Move from A to I
        let start_time = world.time_left;
        assert!(world.map.contains_key(&"I".to_string()));
        assert_eq!(world.current_node, "A".to_string());
        world.move_to_node("I");

        // Make sure time left decreased accordingly
        assert_eq!(world.current_node, "I");
        assert_eq!(world.time_left, start_time - 3);
    }
}
