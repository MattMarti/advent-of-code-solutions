use core::fmt::Debug;
use queues::*;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, stdin, BufReader};

fn delay() {
    //thread::sleep(time::Duration::from_millis(100));

    let mut tmp = String::new();
    let _ = stdin().read_line(&mut tmp);

    println!();
}

#[derive(Debug)]
enum Operation {
    Add,
    Mult,
}

impl Operation {
    pub fn from_str(s: &str) -> Self {
        match s {
            "+" => Operation::Add,
            "*" => Operation::Mult,
            _ => panic!("Unsupported operation {}", s),
        }
    }
}

struct MonkeyThrow {
    pub dest: usize,
    pub value: u64,
}

struct Monkey {
    items: Queue<u64>,
    operation: Operation,
    argument: Option<u64>,
    pub discriminator: u64,
    pub reducer: u64,
    throw_target_a: usize,
    throw_target_b: usize,
    num_inspected: u64,
}

impl Monkey {
    pub fn from_code(code: &[String]) -> Self {
        let operation_data = code[1].split_whitespace().skip(4).collect::<Vec<&str>>();
        let operator_argument = match operation_data[1] {
            "old" => None,
            x => Some(x.to_string().parse::<u64>().unwrap()),
        };
        Self {
            items: Monkey::read_item_list(&code[0].split_whitespace().skip(2).collect::<String>()),
            operation: Operation::from_str(operation_data[0]),
            argument: operator_argument,
            discriminator: code[2]
                .split_whitespace()
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            reducer: 1,
            throw_target_a: code[3]
                .split_whitespace()
                .last()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap(),
            throw_target_b: code[4]
                .split_whitespace()
                .last()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap(),
            num_inspected: 0,
        }
    }

    fn read_item_list(list_values: &String) -> Queue<u64> {
        let mut queue = Queue::<u64>::new();
        for v in list_values.split(",") {
            queue.add(v.to_string().parse::<u64>().unwrap());
        }
        queue
    }

    pub fn has_items(&self) -> bool {
        self.items.size() > 0
    }

    pub fn pop_next_throw(&mut self) -> MonkeyThrow {
        let mut value = self.items.remove().unwrap();
        //println!("  Inspecting item of {}", value);
        let op_arg = match self.argument {
            Some(x) => x,
            None => value,
        };
        value = match self.operation {
            Operation::Add => value + op_arg,
            Operation::Mult => value * op_arg,
        };
        //println!("  New value is {}", value);
        // Monkey inspect
        value %= self.reducer;
        self.num_inspected += 1;
        //println!("  Adjusting value to {}", value);
        let next_monkey = match value % self.discriminator {
            0 => self.throw_target_a,
            _ => self.throw_target_b,
        };
        //println!(
        //    "  Next target is {} (disc {})",
        //    next_monkey, self.discriminator
        //);
        MonkeyThrow {
            dest: next_monkey,
            value: value,
        }
    }

    pub fn push(&mut self, new_value: u64) {
        self.items.add(new_value);
    }

    pub fn get_num_inspected(&self) -> u64 {
        self.num_inspected
    }
}

impl Debug for Monkey {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        println!("Monkey: {{");
        println!("  items: {:?}", self.items);
        println!("  operation: {:?}", self.operation);
        println!("  constant: {:?}", self.argument);
        println!("  discriminator: {:?}", self.discriminator);
        println!("  reducer: {:?}", self.reducer);
        println!("  target_a: {:?}", self.throw_target_a);
        println!("  target_b: {:?}", self.throw_target_b);
        println!("  insepcted: {}", self.num_inspected);
        println!("}}");
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    println!("Reading monkeys");
    let mut lines: Vec<String> = Vec::<String>::new();
    for line_input in reader.lines() {
        let line = line_input?;
        if line.is_empty() {
            continue;
        }
        lines.push(line);
    }

    println!("Compiling monkeys");
    let mut monkeys = Vec::<Monkey>::new();
    let mut i: usize = 0;
    while i < lines.len() {
        if &lines[i][..6] == "Monkey" {
            monkeys.push(Monkey::from_code(&lines[i + 1..i + 6]));
            i += 6;
        } else {
            i += 1;
        }
    }

    println!("Calculating reducer");
    let mut reducer = 1;
    for monkey in monkeys.iter() {
        reducer *= monkey.discriminator;
    }
    for monkey in monkeys.iter_mut() {
        monkey.reducer = reducer;
    }

    println!("Printing monkeys");
    for monkey in monkeys.iter() {
        println!("{:?}", monkey);
    }

    //let num_rounds: usize = 20;
    let num_rounds: usize = 10000;
    println!("Playing");
    for round in 1..num_rounds + 1 {
        for i in 0..monkeys.len() {
            //println!("Monkey {}:", i);
            while monkeys[i].has_items() {
                let throw = monkeys[i].pop_next_throw();
                //println!(" Throwing value of {} to {}", throw.value, throw.dest);
                monkeys[throw.dest].push(throw.value);
            }
        }

        if round % 1000 == 0 || round == 1 {
            println!("------------------------");
            println!("Round {} monkey results:", round);
            for monk in monkeys.iter() {
                println!("{:?}", monk);
            }
            println!("------------------------");
            //delay();
        }
    }

    println!("Amount monkey business: {}", calc_monkey_business(&monkeys));

    Ok(())
}

fn calc_monkey_business(monkeys: &[Monkey]) -> u64 {
    let mut most_insepcted = Vec::<u64>::new();
    let most = std::cmp::max(monkeys[0].num_inspected, monkeys[1].num_inspected);
    let next_most = std::cmp::min(monkeys[0].num_inspected, monkeys[1].num_inspected);
    most_insepcted.push(most);
    most_insepcted.push(next_most);
    for m in monkeys.iter().skip(2) {
        if most_insepcted[1] < m.num_inspected {
            most_insepcted[1] = m.num_inspected;
        } else if most_insepcted[0] < m.num_inspected {
            most_insepcted[1] = most_insepcted[0];
            most_insepcted[0] = m.num_inspected;
        }
    }
    most_insepcted[0] * most_insepcted[1]
}

#[cfg(test)]
mod test {
    use super::*;
}
