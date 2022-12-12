use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Default)]
struct SignalSlider {
    pub idx: usize,
    buf: [char; 4],
}

impl SignalSlider {
    fn push(&mut self, c: char) {
        self.buf[self.idx % 4] = c;
        self.idx += 1;
    }

    fn locked(&self) -> bool {
        for (i, &left) in self.buf.iter().enumerate() {
            for j in i+1..self.buf.len() {
                if left == self.buf[j] {
                    return false;
                }
            }
        }
        true
    }
}

fn find_packet_start(signal: &Vec<char>) -> usize {
    let mut slider = SignalSlider::default();
    for i in 0..4 {
        slider.push(signal[i]);
    }
    for i in 4..signal.len() {
        slider.push(signal[i]);
        if slider.locked() {
            break;
        }
    }
    slider.idx
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    for read_line in reader.lines() {
        let line = read_line?;
        println!("Signal start: {}", find_packet_start(&line.chars().collect()));
    }

    Ok(())
}
