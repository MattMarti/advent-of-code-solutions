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

    pub fn spin_clock(&mut self) -> bool {
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

struct Display {
    pixels: Vec<char>,
    num_rows: usize,
    num_cols: usize,
    sprite_pos: usize,
    row: usize,
    clock: usize
}

impl Display {
    pub fn new() -> Self {
        let rows = 6;
        let cols = 40;
        Self {
            num_rows: rows,
            num_cols: cols,
            row: 0,
            pixels: vec!['.'; rows * cols],
            sprite_pos: 1,
            clock: 0,
        }
    }

    fn next_row(&mut self) {
        self.row = (self.row + 1) % self.num_rows;
        self.sprite_pos = self.row * self.num_cols + 1;
        println!("Next row ({})", self.sprite_pos);
    }

    fn set_pixel(&mut self) {
        let sprite_idx = self.clock % 3;
        let draw_index = self.sprite_pos + sprite_idx - 1;
        self.pixels[draw_index] = '#';
    }

    pub fn spin_clock(&mut self) {
        if self.clock % self.num_cols == 0 {
            self.next_row();
        }
        self.set_pixel();
        self.clock += 1;
    }

    pub fn set_sprite_position(&mut self, index: usize) {
        if index == 0 || index > self.pixels.len() - 1 {
            panic!("Sprite index out of bounds with {}", index);
        }
        self.sprite_pos = index;
    }

    pub fn draw(&self) {
        for row in 0..self.num_rows {
            let start = self.num_cols * row;
            let end = start + self.num_cols;
            println!(
                "{}",
                &self.pixels[start..end].into_iter().collect::<String>()
            );
        }
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
    let mut display = Display::new();
    for read_line in reader.lines() {
        let line = read_line?;
        cpu.add_instruction(&line);
        while cpu.spin_clock() {
            if check_idx < check_cycles.len() && cpu.cycle == check_cycles[check_idx] {
                println!("Cycle {}, register {}", cpu.cycle, cpu.register);
                total += cpu.register * cpu.cycle as i32;
                check_idx += 1;
            }
            if cpu.register > 1 {
                display.set_sprite_position(cpu.register as usize);
            }
            display.spin_clock();
        }
    }
    display.draw();
    println!("Signal total: {}", total);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
