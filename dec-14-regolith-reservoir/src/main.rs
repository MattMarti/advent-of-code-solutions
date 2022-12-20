use env_logger;
use log::{debug, info, trace};
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
    min_rock_x: usize,
    max_rock_x: usize,
    max_rock_y: usize,
}

impl World {
    pub fn from_verticies(vertices: &Vec<Vec<Point>>) -> Self {
        const NUM_ROWS: usize = 256;
        const NUM_COLS: usize = 1000;
        let mut tiles = vec![vec![Tile::Empty; NUM_COLS]; NUM_ROWS];
        let spawn_point = Point { x: 500, y: 0 };
        let mut min_x = NUM_COLS;
        let mut max_x = 0;
        let mut max_y = 0;
        for points in vertices.iter() {
            trace!("{:?}", points[0]);
            for i in 1..points.len() {
                trace!(" -> {:?}", points[i]);
                let start = points[i - 1];
                let end = points[i];
                if start.x < end.x {
                    for x in start.x..=end.x {
                        tiles[end.y][x] = Tile::Rock;
                    }
                } else if start.x > end.x {
                    for x in end.x..=start.x {
                        tiles[end.y][x] = Tile::Rock;
                    }
                } else if start.y < end.y {
                    for y in start.y..=end.y {
                        tiles[y][end.x] = Tile::Rock;
                    }
                } else if start.y > end.y {
                    for y in end.y..=start.y {
                        tiles[y][end.x] = Tile::Rock;
                    }
                } else {
                    tiles[end.y][end.x] = Tile::Rock;
                }
                if max_y < start.y || max_y < end.y {
                    max_y = std::cmp::max(start.y, end.y);
                }
                if max_x < start.x || max_x < end.x {
                    max_x = std::cmp::max(start.x, end.x);
                }
                if start.x < min_x || end.x < min_x {
                    min_x = std::cmp::min(start.x, end.x);
                }
            }
            trace!("\n");
        }
        Self {
            tiles: tiles,
            spawn: spawn_point,
            num_sand: 0,
            path: vec![spawn_point; 1],
            min_rock_x: min_x,
            max_rock_x: max_x,
            max_rock_y: max_y,
        }
    }

    fn add_floor(&mut self) {
        let floor_y = self.max_rock_y + 2;
        for j in 0..self.tiles[floor_y].len() {
            self.tiles[floor_y][j] = Tile::Rock;
        }
        self.path = vec![self.spawn; 1];
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
        for i in 0..=self.max_rock_y + 2 {
            write!(f, "{:>3} ", i);
            for j in std::cmp::max(0, self.min_rock_x - 2)
                ..=std::cmp::min(self.tiles[i].len(), self.max_rock_x + 2)
            {
                if (Point { x: j, y: i }) == self.spawn {
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
    env_logger::builder()
        .format_timestamp(None)
        .format_indent(None)
        .format_target(false)
        .format_level(false)
        .init();
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    info!("Filename: {}", fname);

    let vertices = load_vertices(&fname);
    debug!("{:?}", vertices);

    let mut world = World::from_verticies(&vertices);
    while world.can_drop() {
        world.drop_sand();
        trace!("{:?}", world);
        debug!("Last: {}", world.path.last().unwrap().y);
        trace!("\n");
        if world.path.last().unwrap().y >= world.tiles.len() - 2 {
            break;
        }
    }
    info!("Part 1: Number sand dropped: {}", world.num_sand - 1);

    world.add_floor();
    while world.can_drop() {
        world.drop_sand();
        trace!("{:?}", world);
        match world.path.last() {
            Some(y) => debug!("Last: {}", world.path.last().unwrap().y),
            None => debug!("Ran out of space to drop sand"),
        };
        trace!("\n");
    }
    info!("Part 2: Number sand dropped: {}", world.num_sand - 1);
}
