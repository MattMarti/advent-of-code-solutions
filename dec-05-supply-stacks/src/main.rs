use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Movement {
    num: usize,
    src: usize,
    dest: usize,
}

impl Movement {
    pub fn from_line(line: &String) -> Self {
        let line_iter: Vec<&str> = line.split(' ').collect();
        Self {
            num: line_iter[1].to_string().parse::<usize>().unwrap(),
            src: line_iter[3].to_string().parse::<usize>().unwrap() - 1,
            dest: line_iter[5].to_string().parse::<usize>().unwrap() - 1,
        }
    }
}

fn move_crate(stacks: &mut Vec<Vec<char>>, mv: &Movement) {
    for _ in 0..mv.num {
        let elf_crate = stacks[mv.src].pop().unwrap();
        stacks[mv.dest].push(elf_crate);
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut end_of_header_reached = false;
    for read_line in reader.lines() {
        let line = read_line?;
        if line.is_empty() {
            // End of header
            continue;
        }
        if !end_of_header_reached {
            if stacks.len() == 0 {
                let num_crates: usize = (line.len() + 1) / 3 + 1;
                stacks = vec![Vec::new(); num_crates];
            }
            for (vec_idx, line_idx) in (1..line.len()).step_by(4).enumerate() {
                print!("{}, {}", vec_idx, line_idx);
                match line.chars().nth(line_idx) {
                    Some(c) => {
                        println!(": {}", c);
                        if c.is_alphabetic() {
                            stacks[vec_idx].insert(0, c);
                        } else if c.is_numeric() {
                            end_of_header_reached = true;
                            break;
                        }
                    }
                    None => (),
                }
            }
        } else {
            // Do movement
            let mv = Movement::from_line(&line);
            move_crate(&mut stacks, &mv);
        }
    }
    print_stacks(&stacks);
    println!("Top of stacks:");
    for (i, vec) in stacks.iter_mut().enumerate() {
        print!("{}: ", i);
        match &vec.pop() {
            Some(top) => println!("{}", top),
            None => println!("empty!"),
        }
    }
    Ok(())
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for (i, stack) in stacks.iter().enumerate() {
        print!("{}:", i);
        for c in stack.iter() {
            print!(" [{}]", c);
        }
        println!("");
    }
}
