use crate::load_file_lines;

fn parse_array(line: &str) -> Vec<i32> {
    let idx = line.chars().position(|c| c == ':').unwrap() + 1;
    line.chars()
        .skip(idx)
        .collect::<String>()
        .split(' ')
        .filter_map(|v| v.parse().ok())
        .collect()
}

fn parse_ignore_spaces(line: &str) -> i64 {
    let idx = line.chars().position(|c| c == ':').unwrap() + 1;
    let num = line
        .chars()
        .skip(idx)
        .collect::<String>()
        .split(' ')
        .collect::<Vec<&str>>()
        .join("");
    println!("Trying to parse {}", num);
    num.parse().unwrap()
}

pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();

    let times = parse_array(&lines[0]);
    let distances = parse_array(&lines[1]);

    let mut part_1_checksum = 1;
    for i in 0..times.len() {
        let time_limit = times[i];
        let distance_goal = distances[i];

        let mut sims_won = 0;
        for t in 0..10000 {
            let accel = 1;
            let start_vel = t * accel;
            let distance = start_vel * (time_limit - t);
            if distance > distance_goal {
                sims_won += 1;
            }
        }
        part_1_checksum *= sims_won;
    }
    println!("Product of sims won (part 1): {part_1_checksum}");

    // Part 2!
    // d = v*(tl - t)
    // d = t*(tl - t)
    // t^2 - tl*t + d = 0
    let time_limit = parse_ignore_spaces(&lines[0]);
    let distance_goal = parse_ignore_spaces(&lines[1]);
    let a = 1.0;
    let b = time_limit as f64;
    let c = distance_goal as f64;
    let min_accel_time = (-b - f64::sqrt(b * b - 4.0 * a * c)) / (2.0 * a);
    let max_accel_time = (-b + f64::sqrt(b * b - 4.0 * a * c)) / (2.0 * a);
    println!(
        "Number of win sims: {}",
        (f64::floor(max_accel_time) - f64::ceil(min_accel_time)) as i32 + 1
    );
}
