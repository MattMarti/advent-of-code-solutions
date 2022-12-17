use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::option::Option;

struct Packet {
    value: Option<u8>,
    sub_packets: Vec<Packet>,
}

impl Packet {
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

        // Iterate through the string
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
                i == end;
            } else if char_i == '[' {
                let (next_index, packet) = Self::from_str_slice(&s[i + 1..]);
                packets.push(packet);
                i += next_index;
            } else if char_i == ']' {
                return (
                    i + 1,
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
            },
        };
        Ok(())
    }
}

//impl Clone for Packet {/
//
//}
//
//impl Copy for Packet {
//
//}
//
//impl PartialEq for Packet {
//
//}
//
//impl Eq for Packet {
//
//}

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
    for i in 0..left_packets.len() {
        println!("---");
        println!("{:?}", left_packets[i]);
        println!("{:?}", right_packets[i]);
    }

    Ok(())
}
