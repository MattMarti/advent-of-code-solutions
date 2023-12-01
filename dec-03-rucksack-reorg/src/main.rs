use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_common_item(rucksack_group: &[String], repeated_items: &[char]) -> char {
    // Find shortest group
    let mut idx_shortest = 0;
    let mut len_shortest = usize::MAX;
    for (i, group) in rucksack_group.iter().enumerate() {
        if group.len() < len_shortest {
            idx_shortest = i;
            len_shortest = group.len();
        }
    }

    // Check each item to see if it's repeated in other groups
    let common_items: Vec<char> = rucksack_group[idx_shortest].chars().collect();
    for item in common_items {
        if repeated_items.contains(&item) {
            continue;
        }
        let mut contains_item = true;
        for (i, group) in rucksack_group.iter().enumerate() {
            if i == idx_shortest {
                continue;
            }
            contains_item &= group.contains(item);
        }
        if contains_item {
            return item;
        }
    }
    panic!("No common items found in item groups");
}

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
    let mut rucksack_group: [String; 3] = Default::default();
    let mut repeated_items: [char; 3] = Default::default();
    let mut group_idx: usize = 0;
    let mut total_group_item_value: u32 = 0;
    for read_line in reader.lines() {
        // Check duplicate letter
        let line = read_line?;
        let (left, right) = get_compartments(&line);
        let item = find_repeated_item(&left, &right);
        total_value += calc_item_value(item);

        // Check group for common item
        rucksack_group[group_idx] = line;
        repeated_items[group_idx] = item;
        group_idx = (group_idx + 1) % 3;
        if group_idx == 0 {
            let item = get_common_item(&rucksack_group, &repeated_items);
            total_group_item_value += calc_item_value(item);
        }
    }
    println!(
        "Total value of items repeated across compartments: {}",
        total_value
    );
    println!(
        "Total value of common items from groups: {}",
        total_group_item_value
    );
    Ok(())
}
