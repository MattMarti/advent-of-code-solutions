use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const DEFUALT_MAX_ITER: u64 = 1024;

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

fn find_first_repeated_freq(values: &[i32], max_iter: u64) -> Option<i32> {
    let mut running_freq: i32 = 0;
    let mut previous_freq_values = HashSet::from([running_freq]);
    for _ in 0..max_iter {
        for v in values {
            running_freq += v;
            if previous_freq_values.contains(&running_freq) {
                return Some(running_freq);
            }
            previous_freq_values.insert(running_freq);
        }
    }
    None
}

pub fn run(args: &[String]) {
    let values = get_file_contents(&args[0]).unwrap();
    let result_frequency: i32 = values.iter().sum();
    println!("Resulting Frequency: {}", result_frequency);

    let max_iter: u64 = match args.get(1) {
        Some(x) => x.parse::<u64>().unwrap(),
        None => DEFUALT_MAX_ITER,
    };

    match find_first_repeated_freq(&values, max_iter) {
        Some(x) => println!("First repeated value is: {}", x),
        None => println!("No repeated frequency values!"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_a() {
        let values: Vec<i32> = vec![1, -2, 3, 1, 1, -2];
        let repeated = find_first_repeated_freq(&values, DEFUALT_MAX_ITER).unwrap();
        assert_eq!(repeated, 2);
    }

    #[test]
    fn test_example_b() {
        let values: Vec<i32> = vec![1, -1];
        let repeated = find_first_repeated_freq(&values, DEFUALT_MAX_ITER).unwrap();
        assert_eq!(repeated, 0);
    }

    #[test]
    fn test_example_c() {
        let values: Vec<i32> = vec![3, 3, 4, -2, -4];
        let repeated = find_first_repeated_freq(&values, DEFUALT_MAX_ITER).unwrap();
        assert_eq!(repeated, 10);
    }

    #[test]
    fn test_example_d() {
        let values: Vec<i32> = vec![-6, 3, 8, 5, -6];
        let repeated = find_first_repeated_freq(&values, DEFUALT_MAX_ITER).unwrap();
        assert_eq!(repeated, 5);
    }

    #[test]
    fn test_example_e() {
        let values: Vec<i32> = vec![7, 7, -2, -7, -4];
        let repeated = find_first_repeated_freq(&values, DEFUALT_MAX_ITER).unwrap();
        assert_eq!(repeated, 14);
    }
}
