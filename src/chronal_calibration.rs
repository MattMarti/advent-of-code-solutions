use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_file_contents(path: &str) -> io::Result<Vec<i32>> {
    let mut values = Vec::<i32>::new();
    println!("Opening {}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line_input in reader.lines() {
        let line = line_input?;
        if line.is_empty() {
            break;
        }
        values.push(line.parse::<i32>().unwrap());
    }
    Ok(values)
}

pub fn run(args: &[String]) {
    let values = get_file_contents(&args[0]).unwrap();
    let result_frequency: i32 = values.iter().sum();

    println!("Resulting Frequency: {}", result_frequency);
}

