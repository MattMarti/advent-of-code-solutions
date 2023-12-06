use crate::load_file_lines;

fn parse_array(line: &str) -> Vec<i32> {
    let idx = line.chars().position(|c| c == ':').unwrap();
    line.chars()
        .skip(idx)
        .collect::<String>()
        .split(' ')
        .filter_map(|v| v.parse().ok())
        .collect()
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
}
