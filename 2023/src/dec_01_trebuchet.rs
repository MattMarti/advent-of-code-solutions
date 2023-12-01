use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_file_contents(path: &str) -> io::Result<Vec<String>> {
    let mut values = Vec::<String>::new();
    println!("Opening {}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line_input in reader.lines() {
        if let Ok(line) = line_input {
            values.push(line);
        } else {
            break;
        }
    }
    Ok(values)
}

fn get_digits(lines: &[String]) -> Vec<i32> {
    let mut first_last_digits = Vec::<i32>::new();
    for line in lines {
        let mut digits = Vec::<i32>::new();
        for c in line.chars() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d as i32);
            }
        }
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        first_last_digits.push(10 * first + last);
    }
    first_last_digits
}

pub fn run(args: &[String]) {
    let lines = get_file_contents(&args[0]).unwrap();

    let digits = get_digits(&lines);
    println!("Sum of digits: {}", digits.iter().sum::<i32>());
}
