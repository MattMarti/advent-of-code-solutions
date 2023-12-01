use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const NUM_SQAURES: usize = 1000;

struct Claim {
    id: usize,
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
            id: caps["id"].parse::<usize>().unwrap(),
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

struct Panel {
    tiles: Vec<Vec<Vec<usize>>>,
}

impl Panel {
    pub fn from_claims(claims: &[Claim]) -> Self {
        let mut tiles = Vec::<Vec<Vec<usize>>>::new();
        for _ in 0..NUM_SQAURES {
            tiles.push(vec![Vec::<usize>::new(); NUM_SQAURES]);
        }
        let mut panel = Self { tiles };
        for c in claims {
            for i in c.x..c.x + c.width {
                for j in c.y..c.y + c.height {
                    panel.tiles[i][j].push(c.id);
                }
            }
        }
        panel
    }

    fn get_num_overlapped(&self) -> usize {
        let mut num_overlaps = 0;
        for row in self.tiles.iter() {
            for ids in row.iter() {
                if ids.len() > 1 {
                    num_overlaps += 1;
                }
            }
        }
        num_overlaps
    }

    fn get_nonoverlapped_id(&self) -> Option<usize> {
        let mut pure_ids = HashSet::<usize>::new();
        for row in self.tiles.iter() {
            for tile in row.iter() {
                for id in tile {
                    pure_ids.insert(*id);
                }
            }
        }
        for row in self.tiles.iter() {
            for tile in row.iter() {
                if tile.len() > 1 {
                    for id in tile.iter() {
                        pure_ids.remove(id);
                    }
                }
            }
        }
        pure_ids.iter().next().copied()
    }
}

pub fn run(args: &[String]) {
    let claims = load_claims(&args[0]).unwrap();
    println!("Found {} claims", claims.len());

    let panel = Panel::from_claims(&claims);

    let num_overlaps = panel.get_num_overlapped();
    println!("Number of overlaps: {}", num_overlaps);

    let pure_id = panel.get_nonoverlapped_id().unwrap();
    println!("Not overlapped id: {}", pure_id);
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
