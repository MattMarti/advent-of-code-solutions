use crate::load_file_lines;

use core::iter::Zip;
use core::slice::Iter;
use std::io;
use std::thread::sleep;
use std::time::Duration;

struct SpringCollection {
    known_springs: Vec<String>,
    adjacents: Vec<Vec<usize>>,
}

impl SpringCollection {
    // TODO Probably better to make your own result type
    pub fn from_file(path: &str) -> io::Result<Self> {
        let lines = load_file_lines(path)?;
        let mut known_springs = Vec::with_capacity(lines.len());
        let mut adjacents = Vec::with_capacity(lines.len());
        for (i, line) in lines.iter().enumerate() {
            let mut iter = line.split(' ');
            if let Some(type_defs) = iter.next() {
                known_springs.push(type_defs.to_owned());
            } else {
                let msg = format!("Failed to parse spring types at line {}", i + 1);
                return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
            }
            if let Some(seq_def) = iter.next() {
                let mut seq = Vec::with_capacity(10); // Guestimate
                for num in seq_def.split(',') {
                    if let Ok(value) = num.parse() {
                        seq.push(value);
                    } else {
                        let msg = format!(
                            "Failed to parse adjacents def character '{}' at line {}",
                            num,
                            i + 1
                        );
                        return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
                    }
                }
                adjacents.push(seq);
            } else {
                let msg = format!("Failed to parse spring types at line {}", i + 1);
                return Err(io::Error::new(io::ErrorKind::InvalidData, msg));
            }
        }
        Ok(Self {
            known_springs,
            adjacents,
        })
    }

    pub fn iter(&self) -> Zip<Iter<String>, Iter<Vec<usize>>> {
        self.known_springs.iter().zip(self.adjacents.iter())
    }
}

fn make_guess_str(len: usize, seqs: &[usize], spaces: &[usize]) -> Option<String> {
    let mut guess = vec!['.'; len];
    let mut idx = 0;
    for (i, len) in seqs.iter().enumerate() {
        idx += spaces[i];
        if idx + len > guess.len() {
            return None;
        }
        for j in idx..idx + len {
            guess[j] = '#';
        }
        idx += len;
    }
    Some(guess.iter().collect())
}

fn count_combos(known: &str, seqs: &[usize]) -> usize {
    let mut spaces = vec![1; seqs.len()];
    spaces[0] = 0;

    let mut num_matches = 0;
    let mut num_failed: i32 = 0;
    let mut idx: i32 = spaces.len() as i32 - 1;
    while idx >= 0 {
        let guess = make_guess_str(known.chars().count(), seqs, &spaces);
        if let Some(line) = guess {
            num_failed = 0;
            if is_match(known, &line) {
                num_matches += 1;
            }
            spaces[idx as usize] += 1;
            println!("{} : {} {} {} {}", known, line, num_matches, idx, num_failed);
            sleep(Duration::from_millis(250));
        } else {
            idx -= 1;
            num_failed += 1;

            println!("{} {}", idx, num_failed);

            if num_failed as usize == spaces.len() {
                break;
            }
            spaces.iter_mut().skip(idx as usize + 1).for_each(|s| *s = 1);
            spaces[idx as usize] += 1;
            idx = spaces.len() as i32 - 1;
            sleep(Duration::from_millis(250));
        }
    }
    println!();
    num_matches
}

fn is_match(known: &str, test: &str) -> bool {
    if known.len() != test.len() {
        return false;
    }
    for (k, t) in known.chars().zip(test.chars()) {
        if k == '?' {
            continue;
        }
        if k != t {
            return false;
        }
    }
    true
}

pub fn run(args: &[String]) {
    let spring_data = SpringCollection::from_file(&args[0]).unwrap();

    let num_combos: usize = spring_data
        .iter()
        .map(|(types, seqs)| count_combos(types, seqs))
        .sum();
    println!("Sum of possible row combinations (part 1): {}", num_combos);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("#.#.##", "#.#.##")]
    #[case("#.?.##", "#.#.##")]
    #[case("#?#.##", "#.#.##")]
    #[case("#?#.##", "###.##")]
    #[case("??#.#?", "#.#.##")]
    fn test_is_match_true(#[case] known: &str, #[case] candidate: &str) {
        assert!(is_match(known, candidate));
    }

    #[rstest]
    #[case("#.?.##", "#.####")]
    fn test_is_match_false(#[case] known: &str, #[case] candidate: &str) {
        assert!(!is_match(known, candidate));
    }

    #[rstest]
    #[case("#.?", &[1, 1, 3], 0)]
    #[case("#.?.##", &[1, 1, 3], 0)]
    fn test_count_combos(#[case] known: &str, #[case] seqs: &[usize], #[case] count: usize) {
        assert_eq!(count_combos(known, seqs), count);
    }
}
