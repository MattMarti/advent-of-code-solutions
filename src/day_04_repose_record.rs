use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Timestamp {
    //year: usize,
    //month: usize,
    //day: usize,
    hour: usize,
    minute: usize,
}

impl Timestamp {
    pub fn new(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\]"
            )
            .unwrap();
        };
        RE.captures_iter(s).next().map(|caps| Self {
            //year: caps["year"].parse().unwrap(),
            //month: caps["month"].parse().unwrap(),
            //day: caps["day"].parse().unwrap(),
            hour: caps["hour"].parse().unwrap(),
            minute: caps["minute"].parse().unwrap(),
        })
    }
}

#[derive(PartialEq)]
enum GuardAction {
    BeginShift,
    FallAsleep,
    WakeUp,
}

struct LogEntry {
    guard_id: usize,
    action: GuardAction,
    timestamp: Timestamp,
}

impl LogEntry {
    pub fn new_begin_shift_entry(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"\[.{16}\] Guard #(?P<id>\d+) begins shift").unwrap();
        };
        RE.captures_iter(s).next().map(|cap| Self {
            guard_id: cap["id"].parse::<usize>().unwrap(),
            action: GuardAction::BeginShift,
            timestamp: Timestamp::new(s).unwrap(),
        })
    }

    pub fn new_falls_asleep_entry(s: &str, last_id: usize) -> Self {
        Self {
            guard_id: last_id,
            action: GuardAction::FallAsleep,
            timestamp: Timestamp::new(s).unwrap(),
        }
    }

    pub fn new_wake_up_entry(s: &str, last_id: usize) -> Self {
        Self {
            guard_id: last_id,
            action: GuardAction::WakeUp,
            timestamp: Timestamp::new(s).unwrap(),
        }
    }
}

// Assume raw_logs are sorted
fn make_log_entries(raw_logs: Vec<String>) -> Vec<LogEntry> {
    let mut log_entries = Vec::<LogEntry>::new();
    let mut last_id = 0;
    for s in raw_logs.iter() {
        let entry = match s.chars().nth(19).unwrap() {
            'G' => LogEntry::new_begin_shift_entry(s).unwrap(),
            'f' => LogEntry::new_falls_asleep_entry(s, last_id),
            'w' => LogEntry::new_wake_up_entry(s, last_id),
            _ => {
                println!("Bad entry: \"{}\"", s);
                continue;
            }
        };
        last_id = entry.guard_id;
        log_entries.push(entry);
    }
    log_entries
}

fn load_log_entries(path: &str) -> io::Result<Vec<LogEntry>> {
    println!("Opening {}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::<String>::new();
    for line_input in reader.lines() {
        let line = line_input?;
        if line.is_empty() {
            continue;
        }
        lines.push(line);
    }
    lines.sort();
    Ok(make_log_entries(lines))
}

fn get_shift_range(log_entries: &[LogEntry]) -> usize {
    let mut j = 1;
    for entry in &log_entries[1..] {
        if entry.action == GuardAction::BeginShift {
            break;
        }
        j += 1;
    }
    j
}

struct SleepGrid {
    pub guard_ids: Vec<usize>,
    pub asleep: Vec<Vec<bool>>,
    sleep_counts: HashMap<usize, Vec<usize>>,
}

impl SleepGrid {
    fn new(log_entries: &[LogEntry]) -> Self {
        let mut s = Self {
            guard_ids: Vec::<usize>::default(),
            asleep: Vec::<Vec<bool>>::default(),
            sleep_counts: HashMap::<usize, Vec<usize>>::default(),
        };
        let mut i = 0;
        while i < log_entries.len() {
            let entry = &log_entries[i];
            s.guard_ids.push(entry.guard_id);

            let j = i + get_shift_range(&log_entries[i..]);
            let flags = SleepGrid::calc_sleep_flags(&log_entries[i..j]);

            let counts = s
                .sleep_counts
                .entry(entry.guard_id)
                .or_insert_with(|| vec![0; 120]);
            for (flag, count) in flags.iter().zip(counts.iter_mut()) {
                *count += *flag as usize;
            }

            s.asleep.push(flags);
            i = j;
        }
        s
    }

    fn calc_sleep_flags(log_entries: &[LogEntry]) -> Vec<bool> {
        let mut sleep_flags: Vec<bool> = vec![false; 120];
        let mut last_asleep_time: &Timestamp = &log_entries[0].timestamp;
        for entry in &log_entries[1..] {
            if entry.action == GuardAction::FallAsleep {
                last_asleep_time = &entry.timestamp;
            } else if entry.action == GuardAction::WakeUp {
                let start = SleepGrid::timestamp_to_sleep_index(last_asleep_time);
                let end = SleepGrid::timestamp_to_sleep_index(&entry.timestamp);
                for f in sleep_flags.iter_mut().take(end).skip(start) {
                    *f = true;
                }
            }
        }
        sleep_flags
    }

    fn timestamp_to_sleep_index(t: &Timestamp) -> usize {
        if t.hour == 23 {
            t.minute
        } else if t.hour == 0 {
            t.minute + 60
        } else {
            panic!("Attempt to convert timestamp to index outside of expected range (23:00 to 1:00): {:02}:{:02}", t.hour, t.minute);
        }
    }

    fn print(&self) {
        for i in 0..self.guard_ids.len() {
            print!("{:>4} ", self.guard_ids[i]);
            for j in 0..self.asleep[i].len() {
                if self.asleep[i][j] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn get_sleepiest_guard(&self) -> (usize, usize) {
        let mut guard_id = 0;
        let mut most_sleep = 0;
        for (id, counts) in self.sleep_counts.iter() {
            let total = counts.iter().sum();
            if total > most_sleep {
                most_sleep = total;
                guard_id = *id;
            }
        }
        (guard_id, most_sleep)
    }

    fn get_most_slept_on_minute(&self, guard_id: usize) -> (usize, usize) {
        let mut most_min_count = 0;
        let mut most_min_index = 0;
        for (i, &count) in self.sleep_counts.get(&guard_id).unwrap().iter().enumerate() {
            if count > most_min_count {
                most_min_count = count;
                most_min_index = i;
            }
        }
        if most_min_index < 60 {
            panic!("Most sleep found before midnight! Problem does not support this");
        }
        (most_min_index - 60, most_min_count)
    }

    fn get_most_frequent_slept_minute(&self) -> (usize, usize) {
        let mut max_frequency = 0;
        let mut most_frequent_minute = 0;
        let mut guard = 0;
        for (id, counts) in self.sleep_counts.iter() {
            let mut max_count = 0;
            let mut index_of_max = 0;
            for (i, c) in counts.iter().enumerate() {
                if *c > max_count {
                    max_count = *c;
                    index_of_max = i;
                }
            }
            if max_count > max_frequency {
                max_frequency = max_count;
                most_frequent_minute = index_of_max;
                guard = *id;
            }
        }
        (guard, most_frequent_minute - 60)
    }
}

pub fn run(args: &[String]) {
    let log_entries = load_log_entries(&args[0]).unwrap();
    println!("Loaded {} log entries", log_entries.len());

    let sleep_grid = SleepGrid::new(&log_entries);
    sleep_grid.print();

    let (guard, amount) = sleep_grid.get_sleepiest_guard();
    println!("Guard {} slept the most at {} minutes.", guard, amount);

    let (most_slept_time, amount) = sleep_grid.get_most_slept_on_minute(guard);
    println!(
        "Most slept on minute was {} at {} times.",
        most_slept_time, amount
    );
    println!("Part 1 hash: {}", guard * most_slept_time);

    let (most_frequent_guard, most_frequent_minute) = sleep_grid.get_most_frequent_slept_minute();
    println!("Part 2");
    println!("Most frequent guard: {}", most_frequent_guard);
    println!("Most frequent minute: {}", most_frequent_minute);
    println!(
        "Part 2 hash: {}",
        most_frequent_guard * most_frequent_minute
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_timestamp_from_str() {
        let input = "[1519-11-02 01:28] Guard #42 begins shift";
        let timestamp = Timestamp::new(&input).unwrap();
        //assert_eq!(timestamp.year, 1519);
        //assert_eq!(timestamp.month, 11);
        //assert_eq!(timestamp.day, 2);
        assert_eq!(timestamp.hour, 1);
        assert_eq!(timestamp.minute, 28);
    }

    #[test]
    fn test_timestamp_from_str_bad_str() {
        let input = "asdf";
        let timestamp = Timestamp::new(&input);
        assert!(timestamp.is_none());
    }
}
