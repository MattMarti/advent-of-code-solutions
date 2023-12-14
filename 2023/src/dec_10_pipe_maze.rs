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
                    start = (j, i);
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
            let next_coord = (col + 1, row);
            pathable_nodes.push(next_coord);
        }
        if curr_node.up_ok && row > 0 {
            let next_coord = (col, row - 1);
            pathable_nodes.push(next_coord);
        }
        if curr_node.left_ok && col > 0 {
            let next_coord = (col - 1, row);
            pathable_nodes.push(next_coord);
        }
        if curr_node.down_ok && row + 1 < self.num_rows() {
            let next_coord = (col, row + 1);
            pathable_nodes.push(next_coord);
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum NodeStatus {
    None,
    MazeVisited,
    EmptyVisited,
    InnerInferred,
}

// The active_nodes is shared state between the maze search and
// opening search. Just don't start doing the other task before
// the first finishes
struct MazeNavigation<'m> {
    maze: &'m PipeMaze,
    active_nodes: Vec<(usize, usize)>,
    node_statuses: Vec<NodeStatus>,
    maze_steps_taken: usize,
}

impl<'m> MazeNavigation<'m> {
    pub fn new(maze: &'m PipeMaze) -> Self {
        let mut obj = Self {
            maze,
            active_nodes: Vec::new(),
            node_statuses: vec![NodeStatus::None; maze.num_rows() * maze.num_cols()],
            maze_steps_taken: 1,
        };
        let start_idx = obj.to_flat_idx(&maze.start);
        obj.node_statuses[start_idx] = NodeStatus::MazeVisited;

        let col = maze.start.0;
        let row = maze.start.1;
        if col + 1 < maze.num_cols() && maze.nodes[row][col + 1].left_ok {
            obj.set_coord_active(&(col + 1, row));
        }
        if row > 0 && maze.nodes[row - 1][col].down_ok {
            obj.set_coord_active(&(col, row - 1));
        }
        if col > 0 && maze.nodes[row][col - 1].right_ok {
            obj.set_coord_active(&(col - 1, row));
        }
        if row + 1 < maze.num_rows() && maze.nodes[row + 1][col].up_ok {
            obj.set_coord_active(&(col, row + 1));
        }
        obj
    }

    fn set_coord_active(&mut self, coord: &(usize, usize)) {
        let idx = self.to_flat_idx(coord);
        self.active_nodes.push(*coord);
        self.node_statuses[idx] = NodeStatus::MazeVisited;
    }

    fn to_flat_idx(&self, coord: &(usize, usize)) -> usize {
        let row = coord.1;
        let col = coord.0;
        self.maze.num_cols() * row + col
    }

    pub fn advance_maze_nav(&mut self) -> usize {
        let mut nodes_advanced = 0;
        let mut next_active_nodes = Vec::new();
        for coord in self.active_nodes.iter() {
            let pathable_coords = self.maze.get_pathable_coords(coord);
            for pc in pathable_coords.iter() {
                let idx = self.to_flat_idx(pc);
                if self.node_statuses[idx] == NodeStatus::MazeVisited {
                    continue;
                }
                next_active_nodes.push(*pc);
                self.node_statuses[idx] = NodeStatus::MazeVisited;
                nodes_advanced += 1;
            }
        }
        self.active_nodes = next_active_nodes;
        if self.active_nodes.is_empty() {
            return 0;
        }
        self.maze_steps_taken += 1;
        nodes_advanced
    }

    pub fn reset_active_nodes(&mut self) {
        self.active_nodes = Vec::new();
    }

    fn get_next_outer(&self) -> Option<(usize, usize)> {
        let mut row = 0;
        let mut col = 0;
        while col + 1 < self.maze.num_cols() {
            if self.node_statuses[self.to_flat_idx(&(col, row))] == NodeStatus::None {
                return Some((col, row));
            }
            col += 1;
        }
        while row + 1 < self.maze.num_rows() {
            if self.node_statuses[self.to_flat_idx(&(col, row))] == NodeStatus::None {
                return Some((col, row));
            }
            row += 1;
        }
        while col > 0 {
            col -= 1;
            if self.node_statuses[self.to_flat_idx(&(col, row))] == NodeStatus::None {
                return Some((col, row));
            }
        }
        while row > 0 {
            row -= 1;
            if self.node_statuses[self.to_flat_idx(&(col, row))] == NodeStatus::None {
                return Some((col, row));
            }
        }
        None
    }

    fn get_outer_neighbors(&self, coord: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut res = Vec::with_capacity(4);
        let col = coord.0;
        let row = coord.1;
        let num_cols = self.maze.num_cols();
        let num_rows = self.maze.num_rows();
        if col + 1 < num_cols {
            let idx = self.to_flat_idx(&(col + 1, row));
            if self.node_statuses[idx] == NodeStatus::None {
                res.push((col + 1, row));
            }
        }
        if row > 0 {
            let idx = self.to_flat_idx(&(col, row - 1));
            if self.node_statuses[idx] == NodeStatus::None {
                res.push((col, row - 1));
            }
        }
        if col > 0 {
            let idx = self.to_flat_idx(&(col - 1, row));
            if self.node_statuses[idx] == NodeStatus::None {
                res.push((col - 1, row));
            }
        }
        if row + 1 < num_rows {
            let idx = self.to_flat_idx(&(col, row + 1));
            if self.node_statuses[idx] == NodeStatus::None {
                res.push((col, row + 1));
            }
        }
        res
    }

    pub fn advance_outer_nav(&mut self) -> Option<usize> {
        if self.active_nodes.is_empty() {
            let next_coord = self.get_next_outer();
            if let Some(coord) = next_coord {
                self.active_nodes.push(coord);
                return Some(1);
            } else {
                return None;
            }
        }
        let mut nodes_advanced = 0;
        let mut next_active_nodes = Vec::new();
        for coord in self.active_nodes.iter() {
            let pathable_coords = self.get_outer_neighbors(coord);
            for pc in pathable_coords.iter() {
                let idx = self.to_flat_idx(pc);
                if self.node_statuses[idx] != NodeStatus::None {
                    continue;
                }
                next_active_nodes.push(*pc);
                self.node_statuses[idx] = NodeStatus::EmptyVisited;
                nodes_advanced += 1;
            }
        }
        self.active_nodes = next_active_nodes;
        if self.active_nodes.is_empty() {
            return Some(0);
        }
        Some(nodes_advanced)
    }
}

fn draw_navigation(dt: &mut DrawTarget, nav: &MazeNavigation) {
    let num_x = nav.maze.num_cols() as f32;
    let num_y = nav.maze.num_rows() as f32;
    let grid_width = dt.width() as f32 / num_x;
    let grid_height = dt.height() as f32 / num_y;

    // Draw visited nodes
    let mut pb = PathBuilder::new();
    for (idx, status) in nav.node_statuses.iter().enumerate() {
        if *status == NodeStatus::MazeVisited {
            let row = idx / nav.maze.num_cols();
            let col = idx % nav.maze.num_cols();
            let x = grid_width * col as f32;
            let y = grid_height * row as f32;
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

    // Draw outer nodes
    let mut pb = PathBuilder::new();
    for (idx, status) in nav.node_statuses.iter().enumerate() {
        if *status == NodeStatus::EmptyVisited {
            let row = idx / nav.maze.num_cols();
            let col = idx % nav.maze.num_cols();
            let x = grid_width * col as f32;
            let y = grid_height * row as f32;
            pb.rect(x, y, grid_width, grid_height);
        }
    }
    let path = pb.finish();
    dt.fill(
        &path,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(
            0xa0, 0xff, 0x00, 0xff,
        )),
        &DrawOptions::new(),
    );

    // Draw enclosed nodes
    let mut pb = PathBuilder::new();
    for (idx, status) in nav.node_statuses.iter().enumerate() {
        if *status == NodeStatus::EmptyVisited {
            let row = idx / nav.maze.num_cols();
            let col = idx % nav.maze.num_cols();
            let x = grid_width * col as f32;
            let y = grid_height * row as f32;
            pb.rect(x, y, grid_width, grid_height);
        }
    }
    let path = pb.finish();
    dt.fill(
        &path,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(
            0xa0, 0x00, 0xff, 0xff,
        )),
        &DrawOptions::new(),
    );

    // Draw active nodes
    let mut pb = PathBuilder::new();
    for coord in nav.active_nodes.iter() {
        let x = grid_width * coord.0 as f32;
        let y = grid_height * coord.1 as f32;
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
    let num_x = maze.num_cols() as f32;
    let num_y = maze.num_rows() as f32;
    let idx_to_px_x = dt.width() as f32 / num_x;
    let idx_to_px_y = dt.height() as f32 / num_y;
    let grid_width = idx_to_px_x / 3.0;
    let grid_height = idx_to_px_y / 3.0;
    let mut pb = PathBuilder::new();
    for (i, row) in maze.nodes.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            let x = j as f32 * idx_to_px_x + grid_height;
            let y = i as f32 * idx_to_px_y + grid_width;
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
    let maze_refresh_ms: u64 = args[1].parse().unwrap();
    let pipe_maze = PipeMaze::from_file_data(&lines);
    let mut navigation = MazeNavigation::new(&pipe_maze);

    let default_window_refresh_ms = 20;
    let window_refresh_ms = if maze_refresh_ms < default_window_refresh_ms {
        maze_refresh_ms
    } else {
        default_window_refresh_ms
    };
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

    draw_navigation(&mut dt, &navigation);
    draw_maze(&mut dt, &pipe_maze);
    window
        .update_with_buffer(dt.get_data(), win_width, win_height)
        .unwrap();
    sleep(Duration::from_millis(window_refresh_ms));

    let mut iter = 1;
    let mut part_1_solved = false;
    let mut part_2_solved = false;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let should_refresh =
            !(part_1_solved && part_2_solved) && iter % (maze_refresh_ms / window_refresh_ms) == 0;

        if should_refresh {
            if !(part_1_solved || part_2_solved) {
                dt.clear(SolidSource::from_unpremultiplied_argb(
                    0x00, 0x00, 0x00, 0x00,
                ));
            }

            if !part_1_solved {
                let num_updated = navigation.advance_maze_nav();
                if num_updated == 0 {
                    println!(
                        "Steps in longest loop (part 1): {}",
                        navigation.maze_steps_taken
                    );
                    part_1_solved = true;
                    navigation.reset_active_nodes();
                }
                draw_navigation(&mut dt, &navigation);
                draw_maze(&mut dt, &pipe_maze);
            }
            if part_1_solved && !part_2_solved {
                if navigation.advance_outer_nav().is_none() {
                    navigation
                        .node_statuses
                        .iter_mut()
                        .filter(|s| **s == NodeStatus::None)
                        .for_each(|s| *s = NodeStatus::InnerInferred);
                    let num_enclosed = navigation
                        .node_statuses
                        .iter()
                        .filter(|s| **s == NodeStatus::InnerInferred)
                        .count();
                    println!("Spaces enclosed (part 2): {}", num_enclosed);
                    part_2_solved = true;
                }
                draw_navigation(&mut dt, &navigation);
                draw_maze(&mut dt, &pipe_maze);
            }
        }
        window
            .update_with_buffer(dt.get_data(), win_width, win_height)
            .unwrap();
        sleep(Duration::from_millis(window_refresh_ms));
        iter += 1;
    }
    window
        .update_with_buffer(dt.get_data(), win_width, win_height)
        .unwrap();
}
