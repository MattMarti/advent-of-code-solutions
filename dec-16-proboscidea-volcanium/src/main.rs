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
    pub total_pressure_released: usize,
    pub current_node: String,
    pub map: HashMap<String, Node>,
}

impl World {
    pub fn new() -> Self {
        Self {
            time_left: 30,
            total_pressure_released: 0,
            current_node: "AA".to_string(),
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, node: &Node) {
        self.map.insert(name.to_string(), node.clone());
    }

    pub fn decrement_time(&mut self, amount: usize) -> bool {
        if amount > self.time_left {
            return false;
        }
        self.time_left -= amount;
        for node in self.map.values() {
            if node.is_open {
                self.total_pressure_released += amount * node.rate;
            }
        }
        debug!("Time left: {}. Pressure released: {}", self.time_left, self.total_pressure_released);
        true
    }

    pub fn follow_path_to_node(&mut self, path: &[String]) -> bool {
        if self.time_left < path.len() {
            trace!("Not enugh time left: {} < {}", self.time_left, path.len());
            return false;
        }
        if path.len() == 0 {
            return true;
        }
        // TODO Check that the start node is the current node
        self.current_node = path.last().unwrap().clone();
        debug!("Set current node to {}", self.current_node);
        self.decrement_time(path.len() - 1);
        true
    }

    pub fn open_current_node(&mut self) -> bool {
        trace!("Opening {}", self.current_node);
        if !self.decrement_time(1) {
            return false;
        }
        self.map.get_mut(&self.current_node).unwrap().is_open = true;
        true
    }

    pub fn find_path_to_next_most_valuable_node(&self) -> Vec<String> {
        let mut path_to_best_node = Vec::<String>::new();
        let mut most_pressure_released = 0;
        for key in self.map.keys() {
            trace!("Checking {}", key);
            let node = self.map.get(key).unwrap();
            if node.rate == 0 || node.is_open {
                trace!("- Not gonna open");
                continue;
            }
            let path = self.dijkstra_path_to_node(key);
            let movement_cost = path.len();
            if self.time_left <= movement_cost {
                trace!("- Costs {} but only have {} left", movement_cost, self.time_left);
                continue;
            }
            let amount_released = (self.time_left - movement_cost) * node.rate;
            trace!("- Could release {}", amount_released);
            if amount_released > most_pressure_released {
                path_to_best_node = path.clone();
                most_pressure_released = amount_released;
            }
        }
        debug!("Next movement: {:?}", path_to_best_node);
        path_to_best_node
    }

    fn dijkstra_path_to_node(&self, target_name: &str) -> Vec<String> {
        let mut all_nodes = HashMap::<String, DijkstraNode>::new();
        for key in self.map.keys() {
            all_nodes.insert(key.clone(), DijkstraNode::new());
        }
        let mut nodes_to_check: Vec<String> = vec![self.current_node.clone()];
        while !nodes_to_check.is_empty() {
            nodes_to_check.sort();

            let current_loc = nodes_to_check.pop().unwrap();
            let current_node = all_nodes.get(&current_loc).unwrap().clone();

            for neighbor_name in self.map.get(&current_loc).unwrap().links.iter() {
                let mut neighbor = all_nodes.get_mut(neighbor_name).unwrap();
                const MOVEMENT_COST: usize = 1;
                if neighbor.parent == None
                    || current_node.local_goal < neighbor.local_goal + MOVEMENT_COST
                {
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
        path.push(target_name.to_string());
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

impl PartialEq for DijkstraNode {
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
    if args.len() != 1 {
        println!("Args: <fname>");
        return;
    }

    let fname = &args[0];
    info!("Filename: {}", fname);

    let mut world = load_nodes(fname).unwrap();
    trace!("Loaded {:?}", world);

    let mut out_of_time = false;
    while world.time_left > 0 {
        let path = world.find_path_to_next_most_valuable_node();

        // TODO How to exclude nodes
        if world.follow_path_to_node(&path) {
            world.open_current_node();
        }
        else {
            // TODO Find a node that doesn't make you run out of time
            break;
        }
        if out_of_time {
            break;
        }
    }
    info!("Part 1: Total pressure released: {}", world.total_pressure_released);
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

        let path = world.dijkstra_path_to_node("I");
        assert_eq!(path.len(), 4);
        assert_eq!(path[0], "A");
        assert_eq!(path[1], "E");
        assert_eq!(path[2], "F");
        assert_eq!(path[3], "I");

        // Move from A to I
        let start_time = world.time_left;
        assert!(world.map.contains_key(&"I".to_string()));
        assert_eq!(world.current_node, "A".to_string());
        world.follow_path_to_node(&path);

        // Make sure time left decreased accordingly
        assert_eq!(world.current_node, "I");
        assert_eq!(world.time_left, start_time - 3);
    }
}
