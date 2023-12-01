use std::collections::HashSet;
use std::env;

mod dec_01_what;

struct ProgramOption {
    pub names: HashSet<String>,
    pub func: fn(&[String]) -> (),
}

fn print_help(options: &[ProgramOption]) {
    println!("Usage: main [problem] [input file]");
    println!("Arguments: ");
    for opt in options {
        print!("  ");
        let mut sep = "";
        for name in opt.names.iter() {
            print!("{sep}{name}");
            sep = ", ";
        }
        println!();
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
        },
        ProgramOption {
            names: cmdset!["day-01", "1", "what"],
            func: dec_01_what::run,
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
