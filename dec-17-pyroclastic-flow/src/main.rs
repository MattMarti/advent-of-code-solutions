use log::LevelFilter;
use log::{debug, error, info, trace};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct LoadError {
    pub details: String,
}

impl LoadError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for LoadError {
    fn description(&self) -> &str {
        &self.details
    }
}

// Functions that load data from files should return results.
fn load_movement(fname: &str) -> Result<Vec<Direction>, LoadError> {
    let file = match File::open(fname) {
        Ok(f) => f,
        Err(e) => {
            return Err(LoadError::new(&format!(
                "Failed to open \"{}\": {}",
                fname, e
            )))
        }
    };
    let reader = BufReader::new(file);
    let mut dirs = Vec::<Direction>::new();
    for (line_num, read_line) in reader.lines().enumerate() {
        let line = match read_line {
            Ok(x) => x,
            Err(e) => return Err(LoadError::new(&format!("Failed to parse line: {}", e))),
        };
        if line.is_empty() {
            continue;
        }
        for (col, c) in line.chars().enumerate() {
            match c {
                '<' => dirs.push(Direction::Left),
                '>' => dirs.push(Direction::Right),
                _ => {
                    return Err(LoadError::new(&format!(
                        "Invalid character '{}' at line {}, col {}",
                        c,
                        line_num + 1,
                        col + 1
                    )))
                }
            };
        }
    }
    Ok(dirs)
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

    let dirs = match load_movement(fname) {
        Ok(x) => x,
        Err(e) => {
            error!("Failed to parse file!");
            error!("{}", e.details);
            return;
        }
    };
}
