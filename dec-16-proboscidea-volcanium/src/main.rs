use lazy_static::lazy_static;
use log::LevelFilter;
use log::{debug, error, info, trace};
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone)]
struct Node {
    rate: usize,
    is_open: bool,
    links: Vec<String>,
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
    time_left: u64,
    map: HashMap<String, Node>,
}

impl World {
    pub fn new() -> Self {
        Self {
            time_left: 30,
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, node: &Node) {
        self.map.insert(name.to_string(), node.clone());
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
        println!("Args: <fname> row <i64> bound <i64>")
    }

    let fname = &args[0];
    info!("Filename: {}", fname);

    let world = load_nodes(fname);
    trace!("Loaded {:?}", world);
}
