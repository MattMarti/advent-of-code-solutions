use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct SignalSlider<const WINDOW: usize> {
    pub idx: usize,
    pub buf: [char; WINDOW],
}

impl<const WINDOW: usize> SignalSlider<WINDOW> {
    fn default() -> Self {
        Self {
            idx: 0,
            buf: [' '; WINDOW],
        }
    }

    fn push(&mut self, c: char) {
        self.buf[self.idx % WINDOW] = c;
        self.idx += 1;
    }

    fn locked(&self) -> bool {
        for (i, &left) in self.buf.iter().enumerate() {
            for j in i + 1..self.buf.len() {
                if left == self.buf[j] {
                    return false;
                }
            }
        }
        true
    }

    pub fn find_start(&mut self, signal: &Vec<char>) -> usize {
        self.idx = 0;
        for &c in signal.iter().take(self.buf.len()) {
            self.push(c);
        }
        for &c in signal.iter().skip(self.buf.len()) {
            self.push(c);
            if self.locked() {
                break;
            }
        }
        self.idx
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut signal: String = String::default();
    for read_line in reader.lines() {
        let line = read_line?;
        signal.push_str(&line);
    }
    println!("Length of input: {}", signal.len());
    let mut packet_slider = SignalSlider::<4>::default();
    let mut message_slider = SignalSlider::<14>::default();
    println!(
        "Packet start: {}",
        packet_slider.find_start(&signal.chars().collect())
    );
    println!(
        "Message start: {}",
        message_slider.find_start(&signal.chars().collect())
    );

    Ok(())
}
