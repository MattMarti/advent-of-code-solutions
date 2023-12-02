use crate::load_file_lines;

fn get_digits(lines: &[String], check_words: bool) -> Vec<i64> {
    let take_values = if check_words { 2 } else { 1 };
    let all_patterns: Vec<Vec<String>> = vec![
        vec!["0".to_owned(), "zero".to_owned()],
        vec!["1".to_owned(), "one".to_owned()],
        vec!["2".to_owned(), "two".to_owned()],
        vec!["3".to_owned(), "three".to_owned()],
        vec!["4".to_owned(), "four".to_owned()],
        vec!["5".to_owned(), "five".to_owned()],
        vec!["6".to_owned(), "six".to_owned()],
        vec!["7".to_owned(), "seven".to_owned()],
        vec!["8".to_owned(), "eight".to_owned()],
        vec!["9".to_owned(), "nine".to_owned()],
    ];
    let mut first_last_digits = Vec::<_>::new();
    for line in lines {
        println!("LINE: {line}");
        let mut digits = Vec::<i32>::new();
        for i in 0..line.len() {
            for (d, patterns) in all_patterns.iter().enumerate() {
                let mut match_found = false;
                for p in patterns.iter().take(take_values) {
                    if i + p.len() > line.len() {
                        continue;
                    }
                    let substr = &line[i..i + p.len()];
                    if substr == p {
                        digits.push(d as i32);
                        match_found = true;
                        break;
                    }
                }
                if match_found {
                    break;
                }
            }
        }
        if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
            println!("- [{first}, {last}]");
            first_last_digits.push((10 * first + last) as i64);
        } else {
            println!("WARNING: No digits found in {}", line);
        }
    }
    first_last_digits
}

pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();

    let check_words = args.len() >= 2 && args[1] == "part_2";

    let digits = get_digits(&lines, check_words);
    println!("Sum of digits: {}", digits.iter().sum::<i64>());
}
