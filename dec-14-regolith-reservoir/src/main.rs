use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Sand,
    Rock,
    Empty,
}

struct World {
    tiles: Vec<Vec<Tile>>,
    spawn: Point,
    num_sand: usize,
    path: Vec<Point>,
}

impl World {
    pub fn from_verticies(vertices: &Vec<Vec<Point>>) -> Self {
        let spawn = Point { x: 500, y: 0 };
        let mut this = Self {
            tiles: vec![vec![Tile::Empty; 1000]; 256],
            spawn: spawn,
            num_sand: 0,
            path: vec![spawn; 1],
        };
        for points in vertices.iter() {
            print!("{:?}", points[0]);
            for i in 1..points.len() {
                print!(" -> {:?}", points[i]);
                let start = points[i - 1];
                let end = points[i];
                if start.x < end.x {
                    for x in start.x..=end.x {
                        this.tiles[end.y][x] = Tile::Rock;
                    }
                } else if start.x > end.x {
                    for x in end.x..=start.x {
                        this.tiles[end.y][x] = Tile::Rock;
                    }
                } else if start.y < end.y {
                    for y in start.y..=end.y {
                        this.tiles[y][end.x] = Tile::Rock;
                    }
                } else if start.y > end.y {
                    for y in end.y..=start.y {
                        this.tiles[y][end.x] = Tile::Rock;
                    }
                } else {
                    this.tiles[end.y][end.x] = Tile::Rock;
                }
            }
            println!();
        }
        this
    }

    pub fn can_drop(&self) -> bool {
        self.path.len() > 0
    }

    pub fn drop_sand(&mut self) {
        self.extend_path();
        let end = self.path.pop().unwrap();
        self.tiles[end.y][end.x] = Tile::Sand;
        self.num_sand += 1;
    }

    fn extend_path(&mut self) {
        while true {
            let end_point = self.path.last().unwrap();
            if end_point.y > self.tiles.len() {
                break;
            }
            match self.get_next_path_point(&end_point) {
                Some(point) => self.path.push(point),
                None => break,
            };
        }
    }

    fn get_next_path_point(&self, point: &Point) -> Option<Point> {
        let x = point.x;
        let y = point.y;
        if y + 1 == self.tiles.len() {
            return None;
        }
        if self.tiles[y + 1][x] == Tile::Empty {
            return Some(Point { x: x, y: y + 1 });
        }
        if self.tiles[y + 1][x - 1] == Tile::Empty {
            return Some(Point { x: x - 1, y: y + 1 });
        }
        if self.tiles[y + 1][x + 1] == Tile::Empty {
            return Some(Point { x: x + 1, y: y + 1 });
        }
        None
    }
}

fn load_vertices(fname: &str) -> Vec<Vec<Point>> {
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);
    let mut all_vertices = Vec::<Vec<Point>>::new();
    for read_line in reader.lines() {
        let line = read_line.unwrap();
        if line.is_empty() {
            continue;
        }

        // Iterate over point
        let mut vertices = Vec::<Point>::new();
        for points in line.split(" -> ").collect::<Vec<&str>>() {
            let coordstr = points.split(",").collect::<Vec<&str>>();
            vertices.push(Point {
                x: coordstr[0].parse::<usize>().unwrap(),
                y: coordstr[1].parse::<usize>().unwrap(),
            });
        }
        all_vertices.push(vertices);
    }
    all_vertices
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut min_x = self.tiles[0].len();
        let mut max_x = 0;
        let mut max_y = 0;
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[i].len() {
                if self.tiles[i][j] == Tile::Rock {
                    if j < min_x {
                        min_x = j;
                    }
                    if max_x < j {
                        max_x = j;
                    }
                    if max_y < i {
                        max_y = i;
                    }
                }
            }
        }
        for i in 0..=max_y {
            write!(f, "{:>3} ", i);
            for j in std::cmp::max(0, min_x - 2)..=std::cmp::min(self.tiles[i].len(), max_x + 2) {
                if (Point{x: j, y: i}) == self.spawn {
                    write!(f, "+");
                    continue;
                }
                match self.tiles[i][j] {
                    Tile::Rock => write!(f, "#"),
                    Tile::Sand => write!(f, "o"),
                    Tile::Empty => write!(f, "."),
                };
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);

    let vertices = load_vertices(&fname);
    println!("{:?}", vertices);

    let mut world = World::from_verticies(&vertices);
    while world.can_drop() {
        world.drop_sand();
        println!("{:?}", world);
        println!("Last: {}", world.path.last().unwrap().y);
        println!();
        if world.path.last().unwrap().y >= world.tiles.len() - 2 {
            println!("Last: {}", world.path.last().unwrap().y);
            println!();
            break;
        }
    }
    println!("Number sand dropped: {}", world.num_sand - 1);
}
