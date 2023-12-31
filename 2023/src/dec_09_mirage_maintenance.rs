use crate::load_file_lines;

fn extrapolate_one(signal: &[i64]) -> (i64, i64) {
    let mut hist: Vec<Vec<i64>> = Vec::with_capacity(signal.len());
    hist.push(vec![0; signal.len()]);
    hist[0][..signal.len()].clone_from_slice(signal);
    for i in 1..signal.len() {
        let mut curr_row: Vec<i64> = Vec::with_capacity(signal.len() - i);
        let last_row = &hist[i - 1];
        for j in 0..last_row.len() - 1 {
            curr_row.push(last_row[j + 1] - last_row[j]);
        }
        if curr_row.iter().all(|x| *x == 0) {
            break;
        }
        hist.push(curr_row);
    }
    let mut before = hist.last().unwrap()[0];
    for i in (0..hist.len() - 1).rev() {
        before = hist[i][0] - before;
    }
    let mut after = *hist.last().unwrap().last().unwrap();
    for i in (0..hist.len() - 1).rev() {
        after += *hist[i].last().unwrap();
    }
    (before, after)
}

pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();
    let dataset: Vec<Vec<i64>> = lines
        .iter()
        .map(|l| l.split(' ').filter_map(|n| n.parse().ok()).collect())
        .collect();

    let extrapolated_values: Vec<(i64, i64)> = dataset
        .iter()
        .map(|signal| extrapolate_one(signal))
        .collect();
    let checksum_1: i64 = extrapolated_values.iter().map(|(_, a)| *a).sum();
    println!("Sum of extrapolated values after (part 1): {checksum_1}");

    let checksum_2: i64 = extrapolated_values.iter().map(|(b, _)| *b).sum();
    println!("Sum of extrapolated values before (part 2): {checksum_2}");
}
