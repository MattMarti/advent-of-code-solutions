use crate::load_file_lines;

use std::thread::sleep;
use std::time::Duration;

use minifb::{Key, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

struct MazeNode {
    up_ok: bool,
    down_ok: bool,
    left_ok: bool,
    right_ok: bool,
}

impl MazeNode {
    pub fn from_char(c: char) -> Self {
        match c {
            'S' => Self {
                up_ok: true,
                down_ok: true,
                left_ok: true,
                right_ok: true,
            },
            '|' => Self {
                up_ok: true,
                down_ok: true,
                left_ok: false,
                right_ok: false,
            },
            '-' => Self {
                up_ok: false,
                down_ok: false,
                left_ok: true,
                right_ok: true,
            },
            'L' => Self {
                up_ok: true,
                down_ok: false,
                left_ok: false,
                right_ok: true,
            },
            'J' => Self {
                up_ok: true,
                down_ok: false,
                left_ok: true,
                right_ok: false,
            },
            '7' => Self {
                up_ok: false,
                down_ok: true,
                left_ok: true,
                right_ok: false,
            },
            'F' => Self {
                up_ok: false,
                down_ok: true,
                left_ok: false,
                right_ok: true,
            },
            '.' => Self {
                up_ok: false,
                down_ok: false,
                left_ok: false,
                right_ok: false,
            },
            _ => panic!("Bad character: {c}"),
        }
    }
}

struct PipeMaze {
    pub start: (usize, usize),
    pub nodes: Vec<Vec<MazeNode>>,
}

impl PipeMaze {
    pub fn from_file_data(lines: &[String]) -> Self {
        let mut nodes: Vec<Vec<MazeNode>> = Vec::with_capacity(lines.len());
        let mut start = (0, 0);
        for (i, line) in lines.iter().enumerate() {
            let mut row: Vec<MazeNode> = Vec::with_capacity(line.chars().count());
            for (j, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = (i, j);
                }
                row.push(MazeNode::from_char(c));
            }
            nodes.push(row);
        }
        Self { start, nodes }
    }

    pub fn get_pathable_coords(&self, coord: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut pathable_nodes = Vec::with_capacity(2);
        let row = coord.1;
        let col = coord.0;
        let curr_node = &self.nodes[row][col];
        if curr_node.right_ok && col + 1 < self.num_cols() {
            pathable_nodes.push((col + 1, row));
        }
        if curr_node.up_ok && row > 0 {
            pathable_nodes.push((col, row - 1));
        }
        if curr_node.left_ok && col > 0 {
            pathable_nodes.push((col - 1, row));
        }
        if curr_node.down_ok && row + 1 < self.num_rows() {
            pathable_nodes.push((col, row + 1));
        }
        pathable_nodes
    }

    pub fn num_rows(&self) -> usize {
        self.nodes.len()
    }

    pub fn num_cols(&self) -> usize {
        self.nodes[0].len()
    }
}

struct MazeNavigation<'m> {
    maze: &'m PipeMaze,
    active_nodes: Vec<(usize, usize)>,
    visited_nodes: Vec<bool>,
    steps_taken: usize,
}

impl<'m> MazeNavigation<'m> {
    pub fn new(maze: &'m PipeMaze) -> Self {
        let mut obj = Self {
            maze,
            active_nodes: Vec::new(),
            visited_nodes: vec![false; maze.num_rows() * maze.num_cols()],
            steps_taken: 1,
        };
        let start_idx = obj.get_visited_node_index(&maze.start);
        obj.visited_nodes[start_idx] = true;

        let row = maze.start.0;
        let col = maze.start.1;
        if col + 1 < maze.num_cols() && maze.nodes[row][col + 1].left_ok {
            let coord = (row, col + 1);
            let idx = obj.get_visited_node_index(&coord);
            obj.active_nodes.push(coord);
            obj.visited_nodes[idx] = true;
        }
        if row > 0 && maze.nodes[row - 1][col].down_ok {
            let coord = (row - 1, col);
            let idx = obj.get_visited_node_index(&coord);
            obj.active_nodes.push(coord);
            obj.visited_nodes[idx] = true;
        }
        if col > 0 && maze.nodes[row][col - 1].right_ok {
            let coord = (row - 1, col);
            let idx = obj.get_visited_node_index(&coord);
            obj.active_nodes.push(coord);
            obj.visited_nodes[idx] = true;
        }
        if row + 1 < maze.num_rows() && maze.nodes[row + 1][col].up_ok {
            let coord = (row + 1, col);
            let idx = obj.get_visited_node_index(&coord);
            obj.active_nodes.push(coord);
            obj.visited_nodes[idx] = true;
        }
        obj
    }

    fn get_visited_node_index(&self, coord: &(usize, usize)) -> usize {
        let row = coord.0;
        let col = coord.1;
        self.maze.num_cols() * row + col
    }

    pub fn advance_nodes(&mut self) -> usize {
        let mut nodes_advanced = 0;
        let mut next_active_nodes = Vec::new();
        for coord in self.active_nodes.iter() {
            let pathable_coords = self.maze.get_pathable_coords(coord);
            for pc in pathable_coords.iter() {
                let idx = self.get_visited_node_index(pc);
                if self.visited_nodes[idx] {
                    continue;
                }
                next_active_nodes.push(*pc);
                self.visited_nodes[idx] = true;
                nodes_advanced += 1;
            }
        }
        self.active_nodes = next_active_nodes;
        if self.active_nodes.is_empty() {
            return 0;
        }
        self.steps_taken += 1;
        nodes_advanced
    }
}

fn draw_navigation(dt: &mut DrawTarget, nav: &MazeNavigation, map_width: usize, map_height: usize) {
    let num_x = map_width as f32;
    let num_y = map_height as f32;
    let idx_to_px_x = dt.width() as f32 / num_x;
    let idx_to_px_y = dt.height() as f32 / num_y;
    let grid_width = idx_to_px_x;
    let grid_height = idx_to_px_y;

    // Draw visited nodes
    let mut pb = PathBuilder::new();
    for (idx, visited) in nav.visited_nodes.iter().enumerate() {
        if *visited {
            let row = idx / nav.maze.num_cols();
            let col = idx % nav.maze.num_cols();
            let x = idx_to_px_x * col as f32;
            let y = idx_to_px_y * row as f32;
            pb.rect(x, y, grid_width, grid_height);
        }
    }
    let path = pb.finish();
    dt.fill(
        &path,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(
            0xa0, 0x00, 0xff, 0x00,
        )),
        &DrawOptions::new(),
    );

    // Draw active nodes
    let mut pb = PathBuilder::new();
    for coord in nav.active_nodes.iter() {
        let x = idx_to_px_x * coord.1 as f32;
        let y = idx_to_px_y * coord.0 as f32;
        pb.rect(x, y, grid_width, grid_height);
    }
    let path = pb.finish();
    dt.fill(
        &path,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(
            0xff, 0xff, 0x00, 0x00,
        )),
        &DrawOptions::new(),
    );
}

fn draw_maze(dt: &mut DrawTarget, maze: &PipeMaze) {
    let num_x = maze.num_rows() as f32;
    let num_y = maze.num_cols() as f32;
    let idx_to_px_x = dt.width() as f32 / num_x;
    let idx_to_px_y = dt.height() as f32 / num_y;
    let grid_width = idx_to_px_x / 3.0;
    let grid_height = idx_to_px_y / 3.0;
    let mut pb = PathBuilder::new();
    for (i, row) in maze.nodes.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            let y = i as f32 * idx_to_px_x + grid_width;
            let x = j as f32 * idx_to_px_y + grid_height;
            if node.right_ok || node.up_ok || node.left_ok || node.down_ok {
                pb.rect(x, y, grid_width, grid_height);
            }
            if node.right_ok {
                pb.rect(x + grid_width, y, grid_width, grid_height);
            }
            if node.up_ok {
                pb.rect(x, y - grid_height, grid_width, grid_height);
            }
            if node.left_ok {
                pb.rect(x - grid_width, y, grid_width, grid_height);
            }
            if node.down_ok {
                pb.rect(x, y + grid_height, grid_width, grid_height);
            }
        }
    }
    let path = pb.finish();
    dt.fill(
        &path,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(
            0xff, 0x00, 0x00, 0xff,
        )),
        &DrawOptions::new(),
    );
}

pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();
    let refresh_ms = args[1].parse().unwrap();
    let pipe_maze = PipeMaze::from_file_data(&lines);
    let mut navigation = MazeNavigation::new(&pipe_maze);

    let width = 750;
    let height = 750;
    let mut window = Window::new(
        "AoC 2023 - Day 10 - Pipe Maze",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{e}");
    });

    let (win_width, win_height) = window.get_size();
    let mut dt = DrawTarget::new(win_width as i32, win_height as i32);
    draw_navigation(
        &mut dt,
        &navigation,
        pipe_maze.num_rows(),
        pipe_maze.num_cols(),
    );
    draw_maze(&mut dt, &pipe_maze);
    window
        .update_with_buffer(dt.get_data(), win_width, win_height)
        .unwrap();
    sleep(Duration::from_millis(refresh_ms));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        dt.clear(SolidSource::from_unpremultiplied_argb(
            0x00, 0x00, 0x00, 0x00,
        ));

        let num_updated = navigation.advance_nodes();
        draw_navigation(
            &mut dt,
            &navigation,
            pipe_maze.num_rows(),
            pipe_maze.num_cols(),
        );
        draw_maze(&mut dt, &pipe_maze);

        window
            .update_with_buffer(dt.get_data(), win_width, win_height)
            .unwrap();

        if num_updated == 0 {
            println!("Steps in longest loop (part 1): {}", navigation.steps_taken);
            break;
        }

        sleep(Duration::from_millis(refresh_ms));
    }
}
