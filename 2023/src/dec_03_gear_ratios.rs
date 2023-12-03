use crate::load_file_lines;

const NUM_SYMBOLS: &'static [char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
const NON_SPECIAL_SYMBOLS: &'static [char] =
    &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.'];

fn get_symbol_locs(lines: &[String]) -> Vec<Vec<bool>> {
    let mut symbol_locs = Vec::<_>::new();
    symbol_locs.reserve(lines.len());
    for line in lines.iter() {
        let mut row = Vec::<_>::new();
        row.reserve(line.chars().count());
        for c in line.chars() {
            row.push(!NON_SPECIAL_SYMBOLS.contains(&c));
        }
        symbol_locs.push(row);
    }
    symbol_locs
}

// Negative numbers represent no digits
fn get_digits(lines: &[String]) -> Vec<Vec<i8>> {
    let mut digits = Vec::<_>::new();
    digits.reserve(lines.len());
    for line in lines.iter() {
        let mut row = Vec::<_>::new();
        row.reserve(line.chars().count());
        for (j, c) in line.chars().enumerate() {
            row.push(-1);
            if NUM_SYMBOLS.contains(&c) {
                row[j] = c.to_string().parse::<i8>().unwrap();
            }
        }
        digits.push(row);
    }
    digits
}

#[derive(Debug)]
struct NumCoord {
    row: i32,
    start: i32,
    end: i32,
}

fn get_number_coords(digits: &[Vec<i8>]) -> Vec<NumCoord> {
    let mut num_coords = Vec::<_>::new();
    for (i, row_digits) in digits.iter().enumerate() {
        let mut j = 0;
        while j < row_digits.len() {
            if row_digits[j] != -1 {
                let start = j;
                while j < row_digits.len() && row_digits[j] != -1 {
                    j += 1;
                }
                num_coords.push(NumCoord {
                    row: i as i32,
                    start: start as i32,
                    end: j as i32,
                });
            } else {
                j += 1;
            }
        }
    }
    num_coords
}

fn find_non_adjacent_nums(lines: &[String], verbose: bool) -> Vec<i64> {
    let mut nums = Vec::<_>::new();
    let spec_symbols = get_symbol_locs(&lines);
    let digit_coords = get_number_coords(&get_digits(&lines));
    if verbose {
        println!("Digit coordinates:");
        for coord in digit_coords.iter() {
            println!("- {} : [{}, {}]", coord.row, coord.start, coord.end);
        }
    }
    let range_i = spec_symbols.len() as i32;
    let range_j = spec_symbols[0].len() as i32;
    for coord in digit_coords.iter() {
        let min_i = i32::max(coord.row - 1, 0) as usize;
        let max_i = i32::min(coord.row + 2, range_i) as usize;
        let min_j = i32::max(coord.start - 1, 0) as usize;
        let max_j = i32::min(coord.end + 1, range_j) as usize;
        let mut has_adjacent_symbols = false;
        for i in min_i..max_i {
            for j in min_j..max_j {
                has_adjacent_symbols |= spec_symbols[i][j];
            }
        }
        if has_adjacent_symbols {
            let row = coord.row as usize;
            let start = coord.start as usize;
            let end = coord.end as usize;
            let substr = lines[row].chars().take(end).skip(start).collect::<String>();
            let value = substr.parse::<_>().unwrap();
            nums.push(value);
        }
    }
    nums
}

pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();

    let verbose = args.contains(&String::from("debug"));

    let lonely_nums = find_non_adjacent_nums(&lines, verbose);
    println!("Part 1 checksum: {}", lonely_nums.iter().sum::<i64>());
}
