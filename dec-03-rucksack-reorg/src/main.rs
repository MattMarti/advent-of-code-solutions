use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_compartments(line: &String) -> (String, String) {
    let n = line.len();
    if n % 2 == 1 {
        panic!("Can't divide string in two parts: {}", line);
    }
    (line[..n / 2].to_string(), line[n / 2..].to_string())
}

fn find_repeated_item(left: &str, right: &str) -> char {
    for s in left.chars() {
        if right.contains(s) {
            return s;
        }
    }
    panic!("Found no repeated strings")
}

fn calc_item_value(item: char) -> u32 {
    if ('A'..='Z').contains(&item) {
        item as u32 - 'A' as u32 + 27
    } else if ('a'..='z').contains(&item) {
        item as u32 - 'a' as u32 + 1
    } else {
        panic!("Unsupported character: {}", item);
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut total_value: u32 = 0;
    for read_line in reader.lines() {
        let line = read_line?;
        let (left, right) = get_compartments(&line);
        let item = find_repeated_item(&left, &right);
        total_value += calc_item_value(item);
    }
    println!("Total value in sacks: {}", total_value);
    Ok(())
}
