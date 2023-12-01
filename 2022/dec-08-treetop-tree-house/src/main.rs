use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn visibility_above(forest: &[Vec<u8>], row: usize, col: usize) -> (bool, usize) {
    for i in (0..row).rev() {
        if forest[row][col] <= forest[i][col] {
            return (false, row - i);
        }
    }
    (true, row)
}

fn visibility_below(forest: &[Vec<u8>], row: usize, col: usize) -> (bool, usize) {
    for i in row + 1..forest.len() {
        if forest[row][col] <= forest[i][col] {
            return (false, i - row);
        }
    }
    (true, forest.len() - row - 1)
}

fn visibility_left(forest: &[Vec<u8>], row: usize, col: usize) -> (bool, usize) {
    for i in (0..col).rev() {
        if forest[row][col] <= forest[row][i] {
            return (false, col - i);
        }
    }
    (true, col)
}

fn visibility_right(forest: &[Vec<u8>], row: usize, col: usize) -> (bool, usize) {
    for i in col + 1..forest[row].len() {
        if forest[row][col] <= forest[row][i] {
            return (false, i - col);
        }
    }
    (true, forest[row].len() - col - 1)
}

fn visibility_from_outside(forest: &[Vec<u8>], row: usize, col: usize) -> (bool, usize) {
    let (va, sa) = visibility_above(forest, row, col);
    let (vb, sb) = visibility_below(forest, row, col);
    let (vl, sl) = visibility_left(forest, row, col);
    let (vr, sr) = visibility_right(forest, row, col);

    (va || vb || vl || vr, sa * sb * sl * sr)
}

fn count_trees_visible(forest: &[Vec<u8>]) -> (usize, usize) {
    let mut num_visible = 0;
    let mut best_score = 0;
    for (i, row) in forest.iter().enumerate() {
        for j in 0..row.len() {
            let (visible, score) = visibility_from_outside(forest, i, j);
            if visible {
                num_visible += 1;
            }
            if best_score < score {
                best_score = score;
            }
        }
    }
    (num_visible, best_score)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut forest: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();
    for read_line in reader.lines() {
        let line = read_line?;
        let tree_sizes: Vec<char> = line.chars().collect();
        forest.push(Vec::<u8>::new());
        for size in tree_sizes {
            forest
                .last_mut()
                .unwrap()
                .push(size.to_digit(10).unwrap() as u8);
        }
    }

    let (num_visible, best_score) = count_trees_visible(&forest);
    println!("Trees visible from outside: {}", num_visible);
    println!("Best visibility score: {}", best_score);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_visibility_above() {
        let forest: Vec<Vec<u8>> = vec![
            vec![1, 1, 3, 1],
            vec![1, 3, 1, 4],
            vec![3, 1, 4, 1],
            vec![1, 1, 4, 1],
        ];

        let (visible_0, score_0) = visibility_above(&forest, 0, 0);
        assert!(visible_0);
        assert_eq!(score_0, 0);

        let (visible_1, score_1) = visibility_above(&forest, 2, 0);
        assert!(visible_1);
        assert_eq!(score_1, 2);

        let (visible_2, score_2) = visibility_above(&forest, 1, 1);
        assert!(visible_2);
        assert_eq!(score_2, 1);

        let (visible_3, score_3) = visibility_above(&forest, 2, 2);
        assert!(visible_3);
        assert_eq!(score_3, 2);

        let (visible_4, score_4) = visibility_above(&forest, 3, 2);
        assert!(!visible_4);
        assert_eq!(score_4, 1);
    }

    #[test]
    fn test_visibility_below() {
        let forest: Vec<Vec<u8>> = vec![
            vec![1, 1, 3, 1],
            vec![1, 3, 2, 1],
            vec![1, 1, 4, 1],
            vec![1, 1, 1, 1],
        ];

        let (visible_0, score_0) = visibility_below(&forest, 0, 0);
        assert!(!visible_0);
        assert_eq!(score_0, 1);

        let (visible_1, score_1) = visibility_below(&forest, 1, 1);
        assert!(visible_1);
        assert_eq!(score_1, 2);

        let (visible_2, score_2) = visibility_below(&forest, 1, 2);
        assert!(!visible_2);
        assert_eq!(score_2, 1);

        let (visible_3, score_3) = visibility_below(&forest, 2, 2);
        assert!(visible_3);
        assert_eq!(score_3, 1);

        let (visible_4, score_4) = visibility_below(&forest, 0, 2);
        assert!(!visible_4);
        assert_eq!(score_4, 2);
    }

    #[test]
    fn test_visibility_left() {
        let forest: Vec<Vec<u8>> = vec![
            vec![1, 1, 1, 1],
            vec![1, 2, 1, 1],
            vec![1, 1, 3, 3],
            vec![1, 1, 1, 1],
        ];

        let (visible_0, score_0) = visibility_left(&forest, 0, 0);
        assert!(visible_0);
        assert_eq!(score_0, 0);

        let (visible_1, score_1) = visibility_left(&forest, 1, 1);
        assert!(visible_1);
        assert_eq!(score_1, 1);

        let (visible_2, score_2) = visibility_left(&forest, 2, 2);
        assert!(visible_2);
        assert_eq!(score_2, 2);

        let (visible_3, score_3) = visibility_left(&forest, 1, 2);
        assert!(!visible_3);
        assert_eq!(score_3, 1);

        let (visible_4, score_4) = visibility_left(&forest, 2, 3);
        assert!(!visible_4);
        assert_eq!(score_4, 1);
    }

    #[test]
    fn test_visibility_right() {
        let forest: Vec<Vec<u8>> = vec![
            vec![1, 1, 1, 1],
            vec![1, 2, 1, 1],
            vec![4, 1, 3, 1],
            vec![1, 1, 1, 1],
        ];

        let (visible_0, score_0) = visibility_right(&forest, 0, 0);
        assert!(!visible_0);
        assert_eq!(score_0, 1);

        let (visible_1, score_1) = visibility_right(&forest, 1, 1);
        assert!(visible_1);
        assert_eq!(score_1, 2);

        let (visible_2, score_2) = visibility_right(&forest, 2, 2);
        assert!(visible_2);
        assert_eq!(score_2, 1);

        let (visible_3, score_3) = visibility_right(&forest, 2, 0);
        assert!(visible_3);
        assert_eq!(score_3, 3);

        let (visible_4, score_4) = visibility_right(&forest, 0, 3);
        assert!(visible_4);
        assert_eq!(score_4, 0);
    }
}
