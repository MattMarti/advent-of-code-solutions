use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Timestamp {
    year: usize,
    month: usize,
    day: usize,
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
            year: caps["year"].parse().unwrap(),
            month: caps["month"].parse().unwrap(),
            day: caps["day"].parse().unwrap(),
            hour: caps["hour"].parse().unwrap(),
            minute: caps["minute"].parse().unwrap(),
        })
    }

    pub fn diff_minutes(&self, other: &Timestamp) -> usize {
        let diff_year = self.year - other.year;
        let diff_month = self.month - other.month + 12 * diff_year;
        let diff_day = self.day - other.day + 31 * diff_month; // This will turn into spaghetti
        let diff_hour = self.hour - other.hour + 24 * diff_day;
        self.minute - other.minute + 60 * diff_hour
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

// Assumes all guard ids are the same
fn count_sleep(log_entries: &[LogEntry]) -> usize {
    let mut total_time = 0;
    let mut last_asleep_time: &Timestamp = &log_entries[0].timestamp;
    for entry in &log_entries[1..] {
        if entry.action == GuardAction::FallAsleep {
            last_asleep_time = &entry.timestamp;
        } else if entry.action == GuardAction::WakeUp {
            total_time += entry.timestamp.diff_minutes(last_asleep_time);
        }
    }
    total_time
}

fn count_total_minutes_slept(log_entries: &[LogEntry]) -> HashMap<usize, usize> {
    let mut sleep_count = HashMap::<usize, usize>::new();
    let mut i = 0;
    while i < log_entries.len() {
        let entry = &log_entries[i];
        if !sleep_count.contains_key(&entry.guard_id) {
            let id: usize = entry.guard_id;
            sleep_count.insert(id, 0);
        }
        let j = i + get_shift_range(&log_entries[i..]);
        *sleep_count.get_mut(&entry.guard_id).unwrap() += count_sleep(&log_entries[i..j]);
        i = j;
    }
    sleep_count
}

fn find_most_minutes_slept(log_entries: &[LogEntry]) -> (usize, usize) {
    let sleep_count = count_total_minutes_slept(log_entries);
    let mut most_sleep = 0;
    let mut sleepiest_guard = 0;
    for (guard_id, count) in sleep_count {
        if count > most_sleep {
            sleepiest_guard = guard_id;
            most_sleep = count;
        }
    }
    (sleepiest_guard, most_sleep)
}

pub fn run(args: &[String]) {
    let log_entries = load_log_entries(&args[0]).unwrap();
    println!("Loaded {} log entries", log_entries.len());

    let (guard, amount) = find_most_minutes_slept(&log_entries);
    println!("Guard {} slept the most at {} minutes.", guard, amount);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_timestamp_from_str() {
        let input = "[1519-11-02 01:28] Guard #42 begins shift";
        let timestamp = Timestamp::new(&input).unwrap();
        assert_eq!(timestamp.year, 1519);
        assert_eq!(timestamp.month, 11);
        assert_eq!(timestamp.day, 2);
        assert_eq!(timestamp.hour, 1);
        assert_eq!(timestamp.minute, 28);
    }

    fn test_timestamp_from_str_bad_str() {
        let input = "asdf";
        let timestamp = Timestamp::new(&input);
        assert!(timestamp.is_none());
    }
}
