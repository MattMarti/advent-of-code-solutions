use std::env;
use std::fs::File;
use std::io::{self, prelude::*, stdin, BufReader};
use std::{thread, time};

fn delay() {
    thread::sleep(time::Duration::from_millis(100));

    //let mut tmp = String::new();
    //let _ = stdin().read_line(&mut tmp);

    println!();
    println!();
}

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

    pub fn spin_once(&mut self) -> bool {
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
    row: usize,
    clock: usize,
}

impl Display {
    pub fn new() -> Self {
        let rows = 6;
        let cols = 40;
        Self {
            num_rows: rows,
            num_cols: cols,
            row: rows - 1,
            pixels: vec!['.'; rows * cols],
            clock: 0,
        }
    }

    fn spin_row(&mut self) {
        if self.clock % self.num_cols == 0 {
            self.row = (self.row + 1) % self.num_rows;
        }
    }

    fn set_pixel_with_sprite(&mut self, sprite_pos: i32) {
        let pixel_index = self.clock % self.pixels.len();
        let row_index = (pixel_index % self.num_cols) as i32;
        if sprite_pos - 1 <= row_index && row_index <= sprite_pos + 1 {
            self.pixels[pixel_index] = '#';
        }
    }

    pub fn spin_once(&mut self, sprite_pos: i32) {
        self.set_pixel_with_sprite(sprite_pos);
        self.spin_row();
        self.clock += 1;
    }

    pub fn draw_sprite(&self, sprite_pos: i32) {
        let mut sprite: Vec<char> = vec!['.'; self.num_cols];
        for i in sprite_pos - 1..sprite_pos + 2 {
            if 0 <= i && i < sprite.len() as i32 {
                sprite[i as usize] = '#';
            }
        }
        println!("{}", sprite.iter().collect::<String>());
        let debug_line: Vec<char> = vec!['-'; self.num_cols];
        println!("{}", debug_line.iter().collect::<String>())
    }

    pub fn draw(&self) {
        for row in 0..self.num_rows {
            let start = self.num_cols * row;
            let end = start + self.num_cols;
            println!("{}", &self.pixels[start..end].iter().collect::<String>());
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
    for line_input in reader.lines() {
        let line = line_input?;
        cpu.add_instruction(&line);
        while cpu.spin_once() {
            if check_idx < check_cycles.len() && cpu.cycle == check_cycles[check_idx] {
                println!("Cycle {}, register {}", cpu.cycle, cpu.register);
                total += cpu.register * cpu.cycle as i32;
                check_idx += 1;
            }

            display.spin_once(cpu.register);
            display.draw_sprite(cpu.register);
            display.draw();

            delay();
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
