use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct SignalSlider {

}

fn find_packet_start(signal: &str) -> usize {


    0
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    for read_line in reader.lines() {
        let line = read_line?;
        println!("Signal start: {}", find_packet_start(&line));
    }

    Ok(())
}
