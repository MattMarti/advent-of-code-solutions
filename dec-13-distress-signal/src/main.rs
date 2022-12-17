use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::option::Option;
use std::cmp::Ordering;

struct Packet {
    pub value: Option<u8>,
    pub sub_packets: Vec<Packet>,
}

impl Packet {
    pub fn from_value(value: u8) -> Self{
        Self {
            value: Some(value),
            sub_packets: Vec::<>::new(),
        }
    }

    pub fn from_values_vec(values: &Vec<u8>) -> Self {
        let mut sub_packets = Vec::<Self>::new();
        for &v in values.iter() {
            sub_packets.push(Self::from_value(v));
        }

        Self {
            value: None,
            sub_packets: sub_packets,
        }
    }

    fn from_str_slice(s: &str) -> (usize, Self) {
        let re_only_number = Regex::new(r"^\d+$").unwrap();
        if re_only_number.is_match(s) {
            return (
                s.len(),
                Self {
                    value: Some(s.to_string().parse::<u8>().unwrap()),
                    sub_packets: Vec::<Packet>::new(),
                },
            );
        }

        let chars = s.chars();
        let mut packets = Vec::<Packet>::new();
        let mut i = 0;
        while i < s.len() {
            let char_i = s[i..i + 1].chars().next().unwrap();

            // TODO do this with a match
            // If number, go to base case
            if char_i.is_digit(10) {
                let re_number = Regex::new(r"^\d+").unwrap();
                let digit_match = re_number.find(&s[i..]).unwrap();
                let (end, packet) = Self::from_str_slice(&s[i..i + digit_match.end()]);
                packets.push(packet);
                i += end - 1;
            } else if char_i == '[' {
                let (end, packet) = Self::from_str_slice(&s[i + 1..]);
                packets.push(packet);
                i += end + 1;
            } else if char_i == ']' {
                return (
                    i,
                    Self {
                        value: None,
                        sub_packets: packets,
                    },
                );
            }
            i += 1;
        }
        panic!("Unclosed bracket");
    }

    pub fn from_str(s: &str) -> Self {
        let (_, this) = Self::from_str_slice(&s[1..]);
        this
    }

    pub fn is_num(&self) -> bool {
        !self.value.is_none()
    }

    pub fn less_than(&self, other: &Self) -> Option<bool> {
        println!("- Compare {:?} vs {:?}: ", self, other);
        print!("- ");
        if self.is_num() && other.is_num() {
            let lhs: u8 = self.value.unwrap();
            let rhs: u8 = other.value.unwrap();
            if lhs < rhs {
                println!("Less");
                return Some(true);
            } else if lhs > rhs {
                println!("More");
                return Some(false);
            } else {
                println!("Eq");
                return None;
            }
        }
        else if self.is_num() {
            println!("Add list to LHS");
            let new_lhs = Packet::from_values_vec(&vec![self.value.unwrap()]);
            return new_lhs.less_than(&other);
        }
        else if other.is_num() {
            println!("Add list to RHS");
            let new_rhs = Packet::from_values_vec(&vec![other.value.unwrap()]);
            return self.less_than(&new_rhs);
        }
        for i in 0..other.sub_packets.len() {
            if i == self.sub_packets.len() {
                return Some(true);
            }
            let lhs_packet = &self.sub_packets[i];
            let rhs_packet = &other.sub_packets[i];
            let cmp = lhs_packet.less_than(rhs_packet);
            match cmp {
                Some(x) => {
                    return Some(x);
                },
                None => {}, // continue
            }
        }
        if self.sub_packets.len() > other.sub_packets.len() {
            return Some(false);
        }
        None
    }
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.value {
            Some(x) => return write!(f, "{}", x),
            _ => {
                write!(f, "[");
                if !self.sub_packets.is_empty() {
                    write!(f, "{:?}", self.sub_packets[0]);
                    for sub_packet in self.sub_packets.iter().skip(1) {
                        write!(f, ",{:?}", sub_packet);
                    }
                }
                write!(f, "]");
            }
        };
        Ok(())
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        false // TODO Implement this
    }

    fn ne(&self, other: &Self) -> bool {
        !self.ne(other)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.less_than(other) {
            Some(x) => {
                match x {
                    true => Some(Ordering::Less),
                    false => Some(Ordering::Greater),
                }
            },
            None => Some(Ordering::Equal)
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut left_packets = Vec::<Packet>::new();
    let mut right_packets = Vec::<Packet>::new();
    for (row, read_line) in reader.lines().enumerate() {
        let line = read_line?;
        if line.is_empty() {
            continue;
        }

        let first_char = line.chars().next().unwrap();
        if first_char != '[' {
            panic!(
                "First character in line not a bracket! Was '{}'",
                first_char
            );
        }

        if left_packets.len() == right_packets.len() {
            left_packets.push(Packet::from_str(&line));
        } else {
            right_packets.push(Packet::from_str(&line));
        }
    }

    // TODO count number sorted
    let mut total_ordered = 0;
    for i in 0..left_packets.len() {
        println!("---");
        println!("{:?}", left_packets[i]);
        println!("{:?}", right_packets[i]);
        if left_packets[i] < right_packets[i] {
            println!("Is LT");
            total_ordered += i + 1;
        }
    }
    println!("Sum of ordered indices: {}", total_ordered);

    Ok(())
}
