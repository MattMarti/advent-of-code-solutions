use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::option::Option;
use std::cmp::Ordering;

#[derive(Clone, Eq)]
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
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_num() && other.is_num() {
            let lhs: u8 = self.value.unwrap();
            let rhs: u8 = other.value.unwrap();
            if lhs < rhs {
                return Some(Ordering::Less);
            } else if lhs > rhs {
                return Some(Ordering::Greater);
            } else {
                return None;
            }
        }
        else if self.is_num() {
            let new_lhs = Packet::from_values_vec(&vec![self.value.unwrap()]);
            return new_lhs.partial_cmp(&other);
        }
        else if other.is_num() {
            let new_rhs = Packet::from_values_vec(&vec![other.value.unwrap()]);
            return self.partial_cmp(&new_rhs);
        }
        for i in 0..other.sub_packets.len() {
            if i == self.sub_packets.len() {
                return Some(Ordering::Less);
            }
            let lhs_packet = &self.sub_packets[i];
            let rhs_packet = &other.sub_packets[i];
            let cmp = lhs_packet.partial_cmp(rhs_packet);
            match cmp {
                Some(x) => {
                    return Some(x);
                },
                None => {}, // continue
            }
        }
        if self.sub_packets.len() > other.sub_packets.len() {
            return Some(Ordering::Greater);
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
        self.partial_cmp(other).is_none()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.ne(other)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(x) => x,
            None => Ordering::Equal,
        }
    }
}

fn load_packets(fname: &str) -> (Vec<Packet>, Vec<Packet>) {
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);
    let mut left_packets = Vec::<Packet>::new();
    let mut right_packets = Vec::<Packet>::new();
    for (row, read_line) in reader.lines().enumerate() {
        let line = read_line.unwrap();
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
    (left_packets, right_packets)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);

    let (left_packets, right_packets) = load_packets(&fname);

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
    println!("(Part 1) Sum of ordered indices: {}", total_ordered);

    // Construct vector of all packets, including divider packets
    let mut all_packets = Vec::<Packet>::new();
    for lhs in left_packets.iter() {
        all_packets.push(lhs.clone());
    }
    for rhs in left_packets.iter() {
        all_packets.push(rhs.clone());
    }
    let (divider_left, divider_right) = load_packets(&"divider-packets.txt");
    all_packets.extend(divider_left.clone());
    all_packets.extend(divider_right.clone());
    all_packets.sort();

    // Find divider packets
    let mut first_idx = 0;
    let mut second_idx = 0;
    for (i, packet) in all_packets.iter().enumerate() {
        if packet.clone() == divider_left[0] {
            first_idx = i + 1;
        }
        else if packet.clone() == divider_right[0] {
            second_idx = i + 1;
        }
    }
    println!("(Part 2) Decoder key indices: {}", first_idx * second_idx);
}
