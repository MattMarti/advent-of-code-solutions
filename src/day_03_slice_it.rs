use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const NUM_SQAURES: usize = 1000;

struct Claim {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Claim {
    pub fn new(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^#(?P<id>\d+) @ (?P<x>\d+),( )?(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)$"
            )
            .unwrap();
        };
        RE.captures_iter(s).next().map(|caps| Self {
            x: caps["x"].parse::<usize>().unwrap(),
            y: caps["y"].parse::<usize>().unwrap(),
            width: caps["width"].parse::<usize>().unwrap(),
            height: caps["height"].parse::<usize>().unwrap(),
        })
    }
}

fn load_claims(path: &str) -> io::Result<Vec<Claim>> {
    let mut ids = Vec::<Claim>::new();
    println!("Opening {}", path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line_input in reader.lines() {
        let line = line_input?;
        if line.is_empty() {
            continue;
        }
        ids.push(Claim::new(&line).unwrap());
    }
    Ok(ids)
}

fn get_num_overlapped(claims: &[Claim]) -> usize {
    let mut tiles = Vec::<Vec::<usize>>::new();
    for _ in 0..NUM_SQAURES {
        tiles.push(vec![0; NUM_SQAURES]);
    }
    let mut num_overlaps = 0;
    for c in claims {
        for i in c.x..c.x + c.width {
            for j in c.y..c.y + c.height {
                tiles[i][j] += 1;
                if tiles[i][j] == 2 {
                    num_overlaps += 1;
                }
            }
        }
    }
    num_overlaps
}

pub fn run(args: &[String]) {
    let claims = load_claims(&args[0]).unwrap();
    println!("Found {} claims", claims.len());


    let num_overlaps = get_num_overlapped(&claims);
    println!("Number of overlaps: {}", num_overlaps);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_claim_from_str() {
        let input = "#100 @ 123, 456: 789x369";
        let claim = Claim::new(input).unwrap();
        assert_eq!(claim.id, 100);
        assert_eq!(claim.x, 123);
        assert_eq!(claim.y, 456);
        assert_eq!(claim.width, 789);
        assert_eq!(claim.height, 369);
    }

    #[test]
    fn test_claim_from_bad_str() {
        let input = "asdf";
        assert!(Claim::new(input).is_none());
    }
}
