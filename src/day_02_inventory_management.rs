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

// Assume lengths are the same
fn get_same_letters(a: &str, b: &str) -> String {
    let mut same_chars = String::from("");
    for (ac, bc) in a.chars().zip(b.chars()) {
        if ac == bc {
            same_chars.push(ac);
        }
    }
    same_chars
}

fn find_similar_ids(args: &[String]) -> Option<String> {
    for i in 0..args.len() {
        for j in i + 1..args.len() {
            let same_chars = get_same_letters(&args[i], &args[j]);
            if same_chars.len() + 1 == args[i].len() {
                return Some(same_chars);
            }
        }
    }
    None
}

pub fn run(args: &[String]) {
    let ids = load_ids(&args[0]).unwrap();

    let checksum = compute_checksum(&ids);
    println!("Checksum: {}", checksum);

    let same_letters = find_similar_ids(&ids).unwrap();
    println!("Same letters: \"{}\"", same_letters);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_same_letters() {
        let a = "fghij";
        let b = "fguij";
        assert_eq!(get_same_letters(a, b), "fgij");
    }

    #[test]
    fn test_find_similar() {
        let input: Vec<String> = vec![
            String::from("abcde"),
            String::from("fghij"),
            String::from("klmno"),
            String::from("pqrst"),
            String::from("fguij"),
            String::from("axcye"),
            String::from("wvxyz"),
        ];
        let id = find_similar_ids(&input).unwrap();
        assert_eq!(id, "fgij");
    }
}
