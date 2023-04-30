use std::env;

use advent_of_code_2018::chronal_calibration;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let cmd = &args[0];
    match cmd.as_str() {
        "day01" => chronal_calibration::run(&args[1..]),
        "chronal_calibration" => chronal_calibration::run(&args[1..]),
        _ => println!("Unrecognized command: {}", cmd),
    }
}
