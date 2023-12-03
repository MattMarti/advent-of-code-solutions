use crate::load_file_lines;

fn is_special_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}

// Negative numbers represent no digits
fn get_digits(lines: &[String]) -> Vec<Vec<i8>> {
    let mut digits = Vec::with_capacity(lines.len());
    for line in lines.iter() {
        let mut row = Vec::with_capacity(line.chars().count());
        for (j, c) in line.chars().enumerate() {
            row.push(-1);
            if c.is_numeric() {
                row[j] = c.to_string().parse::<i8>().unwrap();
            }
        }
        digits.push(row);
    }
    digits
}

#[derive(Debug, Clone, Copy)]
struct NumCoord {
    row: usize,
    start: usize,
    end: usize,
}

impl NumCoord {
    pub fn contains(&self, coord: (usize, usize)) -> bool {
        self.row == coord.0 && (self.start <= coord.1 && coord.1 < self.end)
    }
}

fn get_number_coords(digits: &[Vec<i8>]) -> Vec<NumCoord> {
    let mut num_coords = Vec::new();
    for (i, row_digits) in digits.iter().enumerate() {
        let mut j = 0;
        while j < row_digits.len() {
            if row_digits[j] != -1 {
                let start = j;
                while j < row_digits.len() && row_digits[j] != -1 {
                    j += 1;
                }
                num_coords.push(NumCoord {
                    row: i,
                    start,
                    end: j,
                });
            } else {
                j += 1;
            }
        }
    }
    num_coords
}

fn get_adjacent_symbols(lines: &[String], coord: &NumCoord) -> Vec<char> {
    let i_start = i32::max(coord.row as i32 - 1, 0) as usize;
    let i_end = coord.row + 2;
    let j_start = i32::max(coord.start as i32 - 1, 0) as usize;
    let j_end = coord.end + 1;
    let mut adjacent_symbols = Vec::new();
    for line in lines.iter().take(i_end).skip(i_start) {
        for c in line.chars().take(j_end).skip(j_start) {
            if is_special_symbol(c) {
                adjacent_symbols.push(c);
            }
        }
    }
    adjacent_symbols
}

// TODO Should probably return a result lol
fn parse_num(lines: &[String], coord: &NumCoord) -> i64 {
    let row = coord.row;
    let start = coord.start;
    let end = coord.end;
    let substr = lines[row].chars().take(end).skip(start).collect::<String>();
    substr.parse().unwrap()
}

fn find_non_adjacent_nums(lines: &[String], verbose: bool) -> Vec<i64> {
    let mut nums = Vec::new();
    let digit_coords = get_number_coords(&get_digits(lines));
    if verbose {
        println!("Digit coordinates:");
        for coord in digit_coords.iter() {
            println!("- {} : [{}, {}]", coord.row, coord.start, coord.end);
        }
    }
    for coord in digit_coords.iter() {
        if !get_adjacent_symbols(lines, coord).is_empty() {
            nums.push(parse_num(lines, coord));
        }
    }
    nums
}

fn find_boardering_nums(sym_coord: (usize, usize), num_coords: &[NumCoord]) -> Vec<NumCoord> {
    let mut adjacent_nums = Vec::new();
    let i_start = i32::max(sym_coord.0 as i32 - 1, 0) as usize;
    let i_end = sym_coord.0 + 2;
    let j_start = i32::max(sym_coord.1 as i32 - 1, 0) as usize;
    let j_end = sym_coord.1 + 2;
    for coord in num_coords.iter() {
        // TOOD There's a more efficient way lmao
        'coord_check: for i in i_start..i_end {
            for j in j_start..j_end {
                if coord.contains((i, j)) {
                    adjacent_nums.push(*coord);
                    break 'coord_check;
                }
            }
        }
    }
    adjacent_nums
}

// A gear ratio is a star with two adjacent numbers
fn find_gear_ratios(lines: &[String]) -> Vec<i64> {
    let mut gear_ratios = Vec::new();
    let all_num_coords = get_number_coords(&get_digits(lines));
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '*' {
                let nums = find_boardering_nums((i, j), &all_num_coords);
                if nums.len() == 2 {
                    let first = parse_num(lines, &nums[0]);
                    let second = parse_num(lines, &nums[1]);
                    gear_ratios.push(first * second);
                }
            }
        }
    }
    gear_ratios
}

pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();

    let verbose = args.contains(&String::from("debug"));

    let lonely_nums = find_non_adjacent_nums(&lines, verbose);
    println!("Part 1 checksum: {}", lonely_nums.iter().sum::<i64>());

    let gear_ratios = find_gear_ratios(&lines);
    println!("Part 2 checksum: {}", gear_ratios.iter().sum::<i64>());
}
