use crate::load_file_lines;

use core::iter::Zip;
use core::slice::Iter;
use std::io;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

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

    pub fn len(&self) -> usize {
        self.known_springs.len()
    }
}

fn make_guess_str(str_len: usize, seqs: &[usize], spaces: &[usize]) -> Option<String> {
    let mut guess = vec!['.'; str_len];
    let mut idx = 0;
    for (i, len) in seqs.iter().enumerate() {
        idx += spaces[i];
        if idx + len > guess.len() {
            return None;
        }
        guess.iter_mut().skip(idx).take(*len).for_each(|c| *c = '#');
        idx += len;
    }
    Some(guess.iter().collect())
}

fn count_combos(known: &str, seqs: &[usize], period_ms: u64) -> usize {
    let mut spaces = vec![1; seqs.len()];
    spaces[0] = 0;

    let mut num_matches = 0;
    let mut idx: i32 = spaces.len() as i32 - 1;
    'main_loop: while idx >= 0 {
        if spaces.iter().sum::<usize>() + seqs.iter().sum::<usize>() > known.len() {
            while spaces.iter().sum::<usize>() + seqs.iter().sum::<usize>() > known.len() {
                idx -= 1;
                if idx < 0 {
                    break 'main_loop;
                }
                spaces
                    .iter_mut()
                    .skip(idx as usize + 1)
                    .for_each(|s| *s = 1);
                spaces[idx as usize] += 1;
            }
            idx = spaces.len() as i32 - 1;
        }
        let guess = make_guess_str(known.chars().count(), seqs, &spaces);
        if let Some(line) = guess {
            if is_match(known, &line) {
                num_matches += 1;
            }
            spaces[idx as usize] += 1;
            if period_ms != 0 {
                print!("\r{} : {} {}", known, line, num_matches);
                let _ = io::stdout().flush();
                sleep(Duration::from_millis(period_ms));
            }
        } else {
            println!("\nBad guess: seqs {:?}, spaces {:?}", seqs, spaces);
        }
        idx = spaces.len() as i32 - 1;
    }
    if period_ms != 0 {
        println!();
    }
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

fn make_longer_spring_data(sc: &SpringCollection, times_copy: usize) -> SpringCollection {
    let mut known_springs = Vec::with_capacity(sc.len());
    let mut adjacents = Vec::with_capacity(sc.len());
    for (known, seqs) in sc.iter() {
        let mut long_known = known.clone();
        let mut long_seqs = seqs.clone();
        for _ in 1..times_copy {
            long_known += "?";
            long_known += known;
            long_seqs.extend(seqs);
        }
        known_springs.push(long_known);
        adjacents.push(long_seqs);
    }
    SpringCollection {
        known_springs,
        adjacents,
    }
}

pub fn run(args: &[String]) {
    let spring_data = SpringCollection::from_file(&args[0]).unwrap();
    let period_ms = args.get(1).unwrap_or(&String::from("0")).parse().unwrap();
    {
        let start = Instant::now();
        let num_combos: usize = spring_data
            .iter()
            .map(|(types, seqs)| count_combos(types, seqs, period_ms))
            .sum();
        let duration = start.elapsed();
        println!("Sum of possible row combinations (part 1): {}", num_combos);
        println!("Solved in {:?}", duration);
    }
    {
        const TIMES_COPY: usize = 5;
        let long_spring_data = make_longer_spring_data(&spring_data, TIMES_COPY);
        let start = Instant::now();
        let mut num_long_combos: u64 = 0;
        for (i, (types, seqs)) in long_spring_data.iter().enumerate() {
            num_long_combos += count_combos(types, seqs, period_ms) as u64;
            let progress = (i + 1) as f32 / long_spring_data.len() as f32;
            let avg_duration = start.elapsed().as_secs_f64() / (i + 1) as f64;
            print!(
                "\rProgress: {:.2}%, avg iter time: {:.3?}",
                100.0 * progress,
                avg_duration
            );
            let _ = io::stdout().flush();
        }
        println!();
        let duration = start.elapsed();
        println!(
            "Sum of possible longer row combinations (part 2): {}",
            num_long_combos
        );
        println!("Solved in {:?}", duration);
    }
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
        assert_eq!(count_combos(known, seqs, 0), count);
    }
}
