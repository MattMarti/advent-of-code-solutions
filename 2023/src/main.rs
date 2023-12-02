use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

mod dec_01_trebuchet;
mod dec_02_cube_conundrum;

pub fn load_file_lines(path: &str) -> io::Result<Vec<String>> {
    let mut lines = Vec::<_>::new();
    println!("Opening {}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line_input in reader.lines() {
        if let Ok(line) = line_input {
            lines.push(line);
        } else {
            break;
        }
    }
    Ok(lines)
}

struct ProgramOption {
    pub names: HashSet<String>,
    pub func: fn(&[String]) -> (),
    pub hint: String,
}

fn print_help(options: &[ProgramOption]) {
    println!("Usage: main [problem] [input file]");
    println!("Arguments: ");
    for opt in options {
        print!(" [");
        let mut sep = "";
        for name in opt.names.iter() {
            print!("{sep}{name}");
            sep = ", ";
        }
        println!("] {}", opt.hint);
    }
}

fn foo(args: &[String]) {
    println!("Yay");
    let mut sep = "";
    for a in args {
        print!("{sep}{a}");
        sep = ", ";
    }
    println!();
}

#[macro_export]
macro_rules! cmdset {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(format!("{}", $x));
            )*
            temp_vec.into_iter().collect()
        }
    };
}

fn main() {
    let options = vec![
        ProgramOption {
            names: cmdset!["foo"],
            func: foo,
            hint: String::from("[file]"),
        },
        ProgramOption {
            names: cmdset!["day-01", "1", "trebuchet"],
            func: dec_01_trebuchet::run,
            hint: String::from("[file] [part_1, part_2]"),
        },
        ProgramOption {
            names: cmdset!["day-02", "2", "cube-conundrum"],
            func: dec_02_cube_conundrum::run,
            hint: String::from("[file] [debug]"),
        },
    ];
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        print_help(&options);
        return;
    }
    let cmd = &args[0];
    if let Some(opt) = options.iter().find(|&o| o.names.contains(cmd)) {
        (opt.func)(&args[1..]);
    } else {
        println!("Unrecognized argument: {cmd}");
        print_help(&options);
    }
}
