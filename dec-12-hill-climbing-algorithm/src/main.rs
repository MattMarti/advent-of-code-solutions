use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, Eq, Hash, Default, PartialEq)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn dist(&self, other: &Point) -> f32 {
        let x_dist = self.x as f32 - other.x as f32;
        let y_dist = self.y as f32 - other.y as f32;
        (x_dist * x_dist + y_dist * y_dist).sqrt()
    }
}

#[derive(Clone, Copy)]
struct Node {
    pub visited: bool,
    pub global_goal: f32,
    pub local_goal: f32,
    pub position: Point,
    pub parent: Point,
}

impl Node {
    pub fn from_point(position: &Point) -> Self {
        Self {
            visited: false,
            global_goal: f32::INFINITY,
            local_goal: f32::INFINITY,
            position: *position,
            parent: Point { x: 0, y: 0 }, // TODO See if you can store a reference here
        }
    }

    pub fn from_parent(position: &Point, parent: &Point) -> Self {
        Self {
            visited: false,
            global_goal: f32::INFINITY,
            local_goal: f32::INFINITY,
            position: *position,
            parent: Point {
                x: parent.x,
                y: parent.y,
            },
        }
    }

    pub fn from_starting_point(start: &Point, target: &Point) -> Self {
        Self {
            visited: false,
            global_goal: start.dist(target),
            local_goal: 0.0,
            position: *start,
            parent: *start,
        }
    }

    pub fn get_neighbors(&self, map: &[Vec<u8>]) -> Vec<Point> {
        let mut neighbors = Vec::<Point>::new();
        let loc = &self.position;
        let curr = map[loc.x][loc.y];
        if loc.x > 0 && map[loc.x - 1][loc.y] + 1 >= curr {
            neighbors.push(Point {
                x: loc.x - 1,
                y: loc.y,
            });
        }
        if loc.x < map.len() - 1 && map[loc.x + 1][loc.y] + 1 >= curr {
            neighbors.push(Point {
                x: loc.x + 1,
                y: loc.y,
            });
        }
        if loc.y > 0 && map[loc.x][loc.y - 1] + 1 >= curr {
            neighbors.push(Point {
                x: loc.x,
                y: loc.y - 1,
            });
        }
        if loc.y < map[loc.x].len() - 1 && map[loc.x][loc.y + 1] + 1 >= curr {
            neighbors.push(Point {
                x: loc.x,
                y: loc.y + 1,
            });
        }
        neighbors
    }

    fn calc_heuristic(&self, end: &Point) -> f32 {
        self.position.dist(&end)
    }
}

fn get_path_astar(map: &[Vec<u8>], start: &Point, end: &Point) -> Result<Vec<Point>, String> {
    // Starting conditions
    let mut all_nodes = Vec::<Vec<Node>>::new(); // TODO use a map instead
    for (i, row) in map.iter().enumerate() {
        all_nodes.push(Vec::<Node>::new());
        for (j, col) in row.iter().enumerate() {
            all_nodes
                .last_mut()
                .unwrap()
                .push(Node::from_point(&Point { x: i, y: j }));
        }
    }
    let mut checks = Vec::<Node>::new();
    checks.push(Node::from_starting_point(end, start));

    // Visit every node found in the algorithm
    while checks.len() > 0 {
        // Sort by global goal, with highest first, lowest last
        checks.sort_by(|a, b| b.global_goal.partial_cmp(&a.global_goal).unwrap());
        let node = checks.pop().unwrap();

        // Check all the neighbors
        for neighbor_point in node.get_neighbors(&map).iter_mut() {
            let mut neighbor = all_nodes[neighbor_point.x][neighbor_point.y];

            // If the heuristic is better, then replace the parent of the neighbor
            if node.local_goal < neighbor.local_goal + 1.0 {
                neighbor.parent = node.position;
                neighbor.local_goal = node.local_goal + 1.0; // All distances are 1
                neighbor.global_goal = neighbor.calc_heuristic(start) + neighbor.local_goal;
            }

            // If not already discovered, add neightbors to the list
            if !neighbor.visited {
                neighbor.visited = true;
                checks.push(neighbor);
            }

            all_nodes[neighbor.position.x][neighbor.position.y] = neighbor;
        }
    }
    if !all_nodes[start.x][start.y].visited {
        return Err("No path found".to_string());
    }

    // Follow the parents of the nodes back to the start
    let mut path = Vec::<Point>::new();
    path.push(all_nodes[start.x][start.y].position);
    while path.last().unwrap() != end {
        let pos = path.last().unwrap();
        let node = all_nodes[pos.x][pos.y];
        path.push(node.parent);
    }
    Ok(path)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut terrain = Vec::<Vec<u8>>::new();
    let mut start_point = Point { x: 0, y: 0 };
    let mut end_point = Point { x: 0, y: 0 };
    for (row, read_line) in reader.lines().enumerate() {
        let line = read_line?;
        terrain.push(Vec::<u8>::new());
        for (col, letter) in line.chars().enumerate() {
            let last = terrain.last_mut().unwrap();
            let height = match letter {
                'S' => {
                    start_point = Point { x: row, y: col };
                    'a' as u8
                }
                'E' => {
                    end_point = Point { x: row, y: col };
                    'z' as u8
                }
                _ => letter as u8,
            };
            last.push(height);
        }
    }

    let path_result = get_path_astar(&terrain, &start_point, &end_point);
    match &path_result {
        Ok(path) => println!("Length of path is {}", path.len() - 1),
        Err(msg) => println!("Failed to find path: {}", msg),
    }

    // Now find the shortest path from any square of 'a'
    let mut shortest_path = path_result.unwrap().len();
    for (i, row) in terrain.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == ('a' as u8) {
                match get_path_astar(&terrain, &Point { x: i, y: j }, &end_point) {
                    Ok(path) => {
                        if path.len() - 1 < shortest_path {
                            shortest_path = path.len() - 1
                        }
                    }
                    Err(_) => (),
                }
            }
        }
    }
    println!(
        "The shortest possible path starting at 'a' is {}",
        shortest_path
    );

    Ok(())
}
