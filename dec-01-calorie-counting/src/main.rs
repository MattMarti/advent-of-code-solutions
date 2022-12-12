use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec::Vec;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut calorie_totals = Vec::<u32>::new();
    calorie_totals.push(0);
    let mut elf_index: usize = 0;
    for read_line in reader.lines() {
        let line = read_line?;
        if line == "" {
            elf_index += 1;
            calorie_totals.push(0);
            continue;
        }
        calorie_totals[elf_index] += line.parse::<u32>().unwrap();
    }
    let mut most_fed_elves: [usize; 3] = [0; 3];
    let mut most_calories: [u32; 3] = [0; 3];
    most_fed_elves[0] = 1 as usize;
    most_calories[0] = calorie_totals[0];
    for i in 1..calorie_totals.len() {
        for j in 0..3 {
            if calorie_totals[i] > most_calories[j] {
                for k in (j + 1..most_calories.len()).rev() {
                    most_fed_elves[k] = most_fed_elves[k - 1];
                    most_calories[k] = most_calories[k - 1];
                }
                most_fed_elves[j] = i + 1;
                most_calories[j] = calorie_totals[i];
                break;
            }
        }
    }
    println!("There are {} elves", calorie_totals.len());
    println!(
        "The most fed elves are #s {}, {}, {}",
        most_fed_elves[0], most_fed_elves[1], most_fed_elves[2]
    );
    println!(
        "They have a combined {} calories.",
        most_calories.iter().sum::<u32>()
    );
    let max = most_calories.iter().max().unwrap();
    println!("The elf with the most has {} calories.", max);
    Ok(())
}
