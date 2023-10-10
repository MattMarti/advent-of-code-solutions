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

fn find_guard_with_most_sleep(log_entries: &[LogEntry]) -> (usize, usize) {
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

struct SleepGrid {
    pub guard_ids: Vec<usize>,
    pub asleep: Vec<Vec<bool>>,
}

impl SleepGrid {
    fn new(log_entries: &[LogEntry]) -> Self {
        let mut s = Self {
            guard_ids: Vec::<usize>::default(),
            asleep: Vec::<Vec<bool>>::default(),
        };
        let mut i = 0;
        while i < log_entries.len() {
            let entry = &log_entries[i];
            s.guard_ids.push(entry.guard_id);

            let j = i + get_shift_range(&log_entries[i..]);
            s.asleep
                .push(SleepGrid::get_sleep_flags(&log_entries[i..j]));

            i = j;
        }
        s
    }

    fn get_sleep_flags(log_entries: &[LogEntry]) -> Vec<bool> {
        let mut sleep_flags: Vec<bool> = vec![false; 120];
        let mut last_asleep_time: &Timestamp = &log_entries[0].timestamp;
        for entry in &log_entries[1..] {
            if entry.action == GuardAction::FallAsleep {
                last_asleep_time = &entry.timestamp;
            } else if entry.action == GuardAction::WakeUp {
                let start = SleepGrid::timestamp_to_sleep_index(last_asleep_time);
                let end = SleepGrid::timestamp_to_sleep_index(&entry.timestamp);
                for i in start..end {
                    sleep_flags[i] = true;
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
}

fn find_most_slept_on_minute(grid: &SleepGrid, guard_id: usize) -> (usize, usize) {
    let mut min_counts: Vec<usize> = vec![0; 120];
    for i in 0..grid.guard_ids.len() {
        if grid.guard_ids[i] != guard_id {
            continue;
        }
        for j in 0..grid.asleep[i].len() {
            if grid.asleep[i][j] {
                min_counts[j] += 1;
            }
        }
    }
    let mut most_min_count = 0;
    let mut most_min_index = 0;
    for (i, &count) in min_counts.iter().enumerate() {
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

pub fn run(args: &[String]) {
    let log_entries = load_log_entries(&args[0]).unwrap();
    println!("Loaded {} log entries", log_entries.len());

    let sleep_grid = SleepGrid::new(&log_entries);
    sleep_grid.print();

    let (guard, amount) = find_guard_with_most_sleep(&log_entries);
    println!("Guard {} slept the most at {} minutes.", guard, amount);

    let (most_slept_time, amount) = find_most_slept_on_minute(&sleep_grid, guard);
    println!(
        "Most slept on minute was {} at {} times.",
        most_slept_time, amount
    );

    println!("Part 1 hash: {}", guard * most_slept_time);
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
