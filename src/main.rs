use std::env;

use advent_of_code_2018 as ac;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let cmd = &args[0];
    let func_args = &args[1..];
    match cmd.as_str() {
        "day01" => ac::chronal_calibration::run(func_args),
        "chronal_calibration" => ac::chronal_calibration::run(func_args),
        "day02" => ac::inventory_management::run(func_args),
        "inventory_management" => ac::inventory_management::run(func_args),
        _ => println!("Unrecognized command: {}", cmd),
    }
}
