use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Clone, Copy, Debug, Eq, Hash, Default, PartialEq)]
struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct HeadMotion {
    pub dir: Direction,
    pub delta: u32,
}

impl HeadMotion {
    fn from_str(line: &str) -> Self {
        use Direction::*;
        let line_iter: Vec<&str> = line.split(' ').collect();
        Self {
            dir: match line_iter[0] {
                "U" => Up,
                "D" => Down,
                "R" => Right,
                "L" => Left,
                _ => panic!("Unsupported movement direction: {}", line_iter[0]),
            },
            delta: line_iter[1].to_string().parse::<u32>().unwrap(),
        }
    }
}

#[derive(Default)]
struct Rope {
    pub knots: Vec<Coord>,
    pub visited: HashMap<Coord, usize>,
}

impl Rope {
    pub fn default() -> Self {
        Self {
            knots: vec![Coord::default(); 2],
            ..Default::default()
        }
    }

    pub fn new(knots: usize) -> Self {
        const MIN_KNOTS: usize = 2;
        assert!(MIN_KNOTS <= knots, "Rope must be created with at least {} knots but was {}", MIN_KNOTS, knots);
        Self {
            knots: vec![Coord::default(); knots],
            ..Default::default()
        }
    }

    pub fn add_motion(&mut self, motion: &HeadMotion) {
        for _ in 0..motion.delta {
            self.move_head_one(&motion.dir);
        }
    }

    fn move_head_one(&mut self, dir: &Direction) {
        use Direction::*;
        match dir {
            Up => self.knots[0].y += 1,
            Down => self.knots[0].y -= 1,
            Right => self.knots[0].x += 1,
            Left => self.knots[0].x -= 1,
        };
        self.move_tail();
        self.visited
            .entry(*self.knots.last().unwrap())
            .and_modify(|num| *num += 1)
            .or_insert(1);
    }

    fn move_tail(&mut self) {
        for i in 1..self.knots.len() {
            let new = Self::move_follower(&self.knots[i - 1], &self.knots[i]);
            self.knots[i] = new;
        }
    }

    fn move_follower(lead: &Coord, follow: &Coord) -> Coord {
        let dist_x = (lead.x - follow.x).abs();
        let dist_y = (lead.y - follow.y).abs();
        let mut new_pos: Coord = *follow;
        if dist_x < 2 && dist_y < 2 {
            return new_pos;
        } else if lead.x == follow.x {
            if follow.y < lead.y - 1 {
                new_pos.y += 1;
            } else if follow.y > lead.y + 1 {
                new_pos.y -= 1;
            }
        } else if lead.y == follow.y {
            if follow.x < lead.x - 1 {
                new_pos.x += 1;
            } else if follow.x > lead.x + 1 {
                new_pos.x -= 1;
            }
        } else {
            if lead.x > follow.x {
                new_pos.x += 1;
            } else if lead.x < follow.x {
                new_pos.x -= 1;
            }
            if lead.y > follow.y {
                new_pos.y += 1;
            } else if lead.y < follow.y {
                new_pos.y -= 1;
            }
        }
        new_pos
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut rope = Rope::default();
    let mut long_rope = Rope::new(10);
    for read_line in reader.lines() {
        let line = read_line?;
        let motion = HeadMotion::from_str(&line);
        rope.add_motion(&motion);
        long_rope.add_motion(&motion);
    }

    let max_visited: usize = rope.visited.len();
    println!("Rope had {} visits.", max_visited);

    let max_visited_long: usize = long_rope.visited.len();
    println!("Long rope had {} visits.", max_visited_long);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    impl Coord {
        fn new(x: i32, y: i32) -> Self {
            Self { x: x, y: y }
        }
    }

    #[test]
    fn equality_for_coords() {
        assert_eq!(Coord::new(1, 2), Coord::new(1, 2));
        assert!(Coord::new(2, 2) != Coord::new(1, 2));
        assert!(Coord::new(2, 2) != Coord::new(2, 0));
    }

    #[test]
    fn make_head_motion() {
        let motion = HeadMotion::from_str("D 5");
        assert!(matches!(motion.dir, Direction::Down));
        assert_eq!(motion.delta, 5);
    }

    #[test]
    fn adding_head_motion_moves_head() {
        let mut rope = Rope::default();
        rope.add_motion(&HeadMotion::from_str("R 1"));
        assert_eq!(rope.head, Coord::new(1, 0));
        rope.add_motion(&HeadMotion::from_str("R 2"));
        assert_eq!(rope.head, Coord::new(3, 0));
        rope.add_motion(&HeadMotion::from_str("U 1"));
        assert_eq!(rope.head, Coord::new(3, 1));
        rope.add_motion(&HeadMotion::from_str("D 3"));
        assert_eq!(rope.head, Coord::new(3, -2));
        rope.add_motion(&HeadMotion::from_str("L 3"));
        assert_eq!(rope.head, Coord::new(0, -2));
    }

    #[test]
    fn tail_moves_cross_r() {
        let mut rope = Rope::default();
        rope.head = Coord::new(1, 0);
        rope.add_motion(&HeadMotion::from_str("R 1"));
        assert_eq!(rope.head, Coord::new(2, 0));
        assert_eq!(rope.tail, Coord::new(1, 0));
        rope.add_motion(&HeadMotion::from_str("R 3"));
        assert_eq!(rope.head, Coord::new(5, 0));
        assert_eq!(rope.tail, Coord::new(4, 0));
    }

    #[test]
    fn tail_moves_cross_l() {
        let mut rope = Rope::default();
        rope.head = Coord::new(-1, 0);
        rope.add_motion(&HeadMotion::from_str("L 1"));
        assert_eq!(rope.head, Coord::new(-2, 0));
        assert_eq!(rope.tail, Coord::new(-1, 0));
        rope.add_motion(&HeadMotion::from_str("L 3"));
        assert_eq!(rope.head, Coord::new(-5, 0));
        assert_eq!(rope.tail, Coord::new(-4, 0));
    }

    #[test]
    fn tail_moves_cross_u() {
        let mut rope = Rope::default();
        rope.head = Coord::new(0, 1);
        rope.add_motion(&HeadMotion::from_str("U 1"));
        assert_eq!(rope.head, Coord::new(0, 2));
        assert_eq!(rope.tail, Coord::new(0, 1));
        rope.add_motion(&HeadMotion::from_str("U 3"));
        assert_eq!(rope.head, Coord::new(0, 5));
        assert_eq!(rope.tail, Coord::new(0, 4));
    }

    #[test]
    fn tail_moves_cross_d() {
        let mut rope = Rope::default();
        rope.head = Coord::new(0, -1);
        rope.add_motion(&HeadMotion::from_str("D 1"));
        assert_eq!(rope.head, Coord::new(0, -2));
        assert_eq!(rope.tail, Coord::new(0, -1));
        rope.add_motion(&HeadMotion::from_str("D 3"));
        assert_eq!(rope.head, Coord::new(0, -5));
        assert_eq!(rope.tail, Coord::new(0, -4));
    }

    #[test]
    fn tail_moves_diag_ru() {
        let cases: Vec<(Coord, HeadMotion)> = vec![
            (Coord::new(1, 1), HeadMotion::from_str("U 1")),
            (Coord::new(1, 1), HeadMotion::from_str("R 1")),
        ];
        for (head_start, motion) in cases {
            let mut rope = Rope::default();
            rope.head = head_start;
            rope.add_motion(&motion);
            assert_eq!(rope.tail, Coord::new(1, 1));
        }
    }

    #[test]
    fn tail_moves_diag_lu() {
        let cases: Vec<(Coord, HeadMotion)> = vec![
            (Coord::new(-1, 1), HeadMotion::from_str("L 1")),
            (Coord::new(-1, 1), HeadMotion::from_str("U 1")),
        ];
        for (head_start, motion) in cases {
            let mut rope = Rope::default();
            rope.head = head_start;
            rope.add_motion(&motion);
            assert_eq!(rope.tail, Coord::new(-1, 1));
        }
    }

    #[test]
    fn tail_moves_diag_ld() {
        let cases: Vec<(Coord, HeadMotion)> = vec![
            (Coord::new(-1, -1), HeadMotion::from_str("L 1")),
            (Coord::new(-1, -1), HeadMotion::from_str("D 1")),
        ];
        for (head_start, motion) in cases {
            let mut rope = Rope::default();
            rope.head = head_start;
            rope.add_motion(&motion);
            assert_eq!(rope.tail, Coord::new(-1, -1));
        }
    }

    #[test]
    fn tail_moves_diag_rd() {
        let cases: Vec<(Coord, HeadMotion)> = vec![
            (Coord::new(1, -1), HeadMotion::from_str("R 1")),
            (Coord::new(1, -1), HeadMotion::from_str("D 1")),
        ];
        for (head_start, motion) in cases {
            let mut rope = Rope::default();
            rope.head = head_start;
            rope.add_motion(&motion);
            assert_eq!(rope.tail, Coord::new(1, -1));
        }
    }

    #[test]
    fn close_head_motion_results_in_no_tail_motion() {
        let cases: Vec<(Coord, HeadMotion)> = vec![
            // Start on top
            (Coord::new(0, 0), HeadMotion::from_str("R 1")),
            (Coord::new(0, 0), HeadMotion::from_str("L 1")),
            (Coord::new(0, 0), HeadMotion::from_str("D 1")),
            (Coord::new(0, 0), HeadMotion::from_str("U 1")),
            // Start at sides
            (Coord::new(1, 0), HeadMotion::from_str("U 1")),
            (Coord::new(1, 0), HeadMotion::from_str("D 1")),
            (Coord::new(0, 1), HeadMotion::from_str("L 1")),
            (Coord::new(0, 1), HeadMotion::from_str("R 1")),
            (Coord::new(0, -1), HeadMotion::from_str("L 1")),
            (Coord::new(0, -1), HeadMotion::from_str("R 1")),
            (Coord::new(-1, 0), HeadMotion::from_str("D 1")),
            (Coord::new(-1, 0), HeadMotion::from_str("U 1")),
            // Move at diagonals
            (Coord::new(-1, 0), HeadMotion::from_str("R 1")),
            (Coord::new(1, 0), HeadMotion::from_str("L 1")),
            (Coord::new(0, 1), HeadMotion::from_str("D 1")),
            (Coord::new(0, -1), HeadMotion::from_str("U 1")),
            (Coord::new(1, 1), HeadMotion::from_str("D 1")),
            (Coord::new(1, 1), HeadMotion::from_str("L 1")),
            (Coord::new(-1, 1), HeadMotion::from_str("D 1")),
            (Coord::new(-1, 1), HeadMotion::from_str("R 1")),
            (Coord::new(-1, -1), HeadMotion::from_str("U 1")),
            (Coord::new(-1, -1), HeadMotion::from_str("R 1")),
            (Coord::new(1, -1), HeadMotion::from_str("U 1")),
            (Coord::new(1, -1), HeadMotion::from_str("L 1")),
        ];
        for (i, (head_start, motion)) in cases.iter().enumerate() {
            let mut rope = Rope::default();
            rope.head = *head_start;
            rope.add_motion(motion);
            assert_eq!(rope.tail, Coord::new(0, 0), "At index {}", i);
        }
    }
}
