use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn load_ids(path: &str) -> io::Result<Vec<String>> {
    let mut ids = Vec::<String>::new();
    println!("Opening {}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line_input in reader.lines() {
        let line = line_input?;
        if line.is_empty() {
            break;
        }
        ids.push(line);
    }
    Ok(ids)
}

fn compute_checksum(ids: &[String]) -> u64 {
    let mut num_two_repeats = 0;
    let mut num_three_repeats = 0;
    for id in ids {
        let mut chars: [u32; 26] = Default::default();
        for c in id.chars() {
            let index = c as usize - 'a' as usize;
            chars[index] += 1;
        }

        // Count two
        for count in chars {
            if count == 2 {
                num_two_repeats += 1;
                break;
            }
        }

        // Count three
        for count in chars {
            if count == 3 {
                num_three_repeats += 1;
                break;
            }
        }
    }
    num_two_repeats * num_three_repeats
}

pub fn run(args: &[String]) {
    let ids = load_ids(&args[0]).unwrap();

    let checksum = compute_checksum(&ids);
    println!("Checksum: {}", checksum)
}
