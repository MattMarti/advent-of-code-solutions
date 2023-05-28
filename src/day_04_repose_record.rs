use lazy_static::lazy_static;
use regex::Regex;
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
            year: caps["year"].parse::<usize>().unwrap(),
            month: caps["month"].parse::<usize>().unwrap(),
            day: caps["day"].parse::<usize>().unwrap(),
            hour: caps["hour"].parse::<usize>().unwrap(),
            minute: caps["minute"].parse::<usize>().unwrap(),
        })
    }
}

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

pub fn run(args: &[String]) {
    let log_entries = load_log_entries(&args[0]).unwrap();

    println!("Loaded {} log entries", log_entries.len());
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
