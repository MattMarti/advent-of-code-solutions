use std::fs::File;
use std::env;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut current_elf: u32 = 0;
    let mut current_calories: u32 = 0;
    let mut most_fed_elf: u32 = 0;
    let mut most_calories: u32 = 0;
    for read_line in reader.lines() {
        let line = read_line?;
        if line == "" {
            current_elf += 1;
            current_calories = 0;
            continue;
        }
        current_calories += line.parse::<u32>().unwrap();
        if current_calories > most_calories {
            most_fed_elf = current_elf;
            most_calories = current_calories;
        }
    }
    println!("There are {} elves", current_elf);
    println!("The most fed elf is # {}", most_fed_elf + 1);
    println!("It has {} calories worth of food.", most_calories);
    Ok(())
}
