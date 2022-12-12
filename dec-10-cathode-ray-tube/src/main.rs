use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
enum Instruction {
    ADDX,
    NOOP,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        match s {
            "ADDX" => Self::ADDX,
            "NOOP" => Self::NOOP,
            "addx" => Self::ADDX,
            "noop" => Self::NOOP,
            _ => panic!("Unsupported operation: {}", s),
        }
    }
}

struct Cpu {
    pub cycle: usize,
    remaining_cycles: usize,
    pub cmd: Instruction,
    pub register: i32,
    next_register: i32,
}

impl Cpu {
    fn new() -> Self {
        Self {
            cycle: 0,
            remaining_cycles: 0,
            cmd: Instruction::NOOP,
            register: 1,
            next_register: 0,
        }
    }

    pub fn spin(&mut self) -> bool {
        if self.remaining_cycles == 0 {
            self.register += self.next_register;
            return false;
        }
        self.remaining_cycles -= 1;
        self.cycle += 1;
        true
    }

    fn add_instruction(&mut self, line: &str) {
        self.cmd = Instruction::from_str(&line[0..4]);
        match self.cmd {
            Instruction::ADDX => self.new_addx(&line[5..]),
            Instruction::NOOP => self.new_noop(),
        }
    }

    fn new_noop(&mut self) {
        self.next_register = 0;
        self.remaining_cycles += 1;
    }

    fn new_addx(&mut self, line: &str) {
        self.next_register = line.to_string().parse::<i32>().unwrap();
        self.remaining_cycles += 2;
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut cpu = Cpu::new();
    let check_cycles: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let mut check_idx: usize = 0;
    let mut total = 0;
    for read_line in reader.lines() {
        let line = read_line?;
        cpu.add_instruction(&line);
        while cpu.spin() {
            if check_idx < check_cycles.len() && cpu.cycle == check_cycles[check_idx] {
                println!("Cycle {}, register {}", cpu.cycle, cpu.register);
                total += cpu.register * cpu.cycle as i32;
                check_idx += 1;
            }
        }
    }
    println!("Signal total: {}", total);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
