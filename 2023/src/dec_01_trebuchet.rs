use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use regex::Regex;

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
    let mut first_last_digits = Vec::<_>::new();
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

fn get_spelled_out_digits(lines: &[String]) -> Vec<i64> {
    let patterns: Vec<&str> = vec![
        "0|(zero)",
        "1|(one)",
        "2|(two)",
        "3|(three)",
        "4|(four)",
        "5|(five)",
        "6|(six)",
        "7|(seven)",
        "8|(eight)",
        "9|(nine)",
    ];
    let total_pattern = patterns.join("|");
    let total_re = Regex::new(&total_pattern).unwrap();
    let num_re: Vec<Regex> = vec![
        Regex::new(patterns[0]).unwrap(),
        Regex::new(patterns[1]).unwrap(),
        Regex::new(patterns[2]).unwrap(),
        Regex::new(patterns[3]).unwrap(),
        Regex::new(patterns[4]).unwrap(),
        Regex::new(patterns[5]).unwrap(),
        Regex::new(patterns[6]).unwrap(),
        Regex::new(patterns[7]).unwrap(),
        Regex::new(patterns[8]).unwrap(),
        Regex::new(patterns[9]).unwrap(),
    ];
    let mut first_last_digits = Vec::<_>::new();
    for line in lines {
        println!("LINE: {line}");
        let mut digits = Vec::<i32>::new();
        let mut last_end_idx = -1;
        for m in total_re.find_iter(line) {
            let value = m.as_str();
            let start_idx = m.start() as i32;
            for (i, re) in num_re.iter().enumerate() {
                if re.find(value).is_some() {
                    println!("- Detect {value}: [{start_idx}, {}]", m.end());
                    if !digits.is_empty() && start_idx < last_end_idx {
                        let current_digit = digits.last_mut().unwrap();
                        *current_digit *= 10;
                        *current_digit += i as i32;
                    }
                    else {
                        digits.push(i as i32);
                    }
                    break;
                }
            }
            last_end_idx = m.end() as i32;
        }
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        println!("- [{first}, {last}]");
        first_last_digits.push((10 * first + last) as i64);
    }
    first_last_digits
}

pub fn run(args: &[String]) {
    let lines = get_file_contents(&args[0]).unwrap();

    if args[1] == "part_1" {
        let digits = get_digits(&lines);
        println!("Sum of digits: {}", digits.iter().sum::<i32>());
    }

    if args[1] == "part_2" {
        let spelled_digits = get_spelled_out_digits(&lines);
        println!(
            "Sum of spelled out digits: {}",
            spelled_digits.iter().sum::<i64>()
        );
    }
}
