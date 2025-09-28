use crate::load_file_lines;

use std::thread::sleep;
use std::time::{Duration, Instant};

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
    PipeNotVisited,
    PipeVisited,
    EmptyVisited,
}

// The active_nodes is shared state between the maze search and
// opening search. Just don't start doing the other task before
// the first finishes
struct MazeNavigation {
    active_nodes: Vec<(usize, usize)>,
    node_statuses: Vec<NodeStatus>,
    num_cols: usize,
    num_rows: usize,
}

impl MazeNavigation {
    pub fn new(maze: &PipeMaze) -> Self {
        let mut obj = Self {
            active_nodes: Vec::new(),
            node_statuses: vec![NodeStatus::None; 9 * maze.num_rows() * maze.num_cols()],
            num_cols: 3 * maze.num_cols(),
            num_rows: 3 * maze.num_rows(),
        };

        // TODO This can be initialized with data directly from the file
        // Construct pipe maze
        for (maze_i, row) in maze.nodes.iter().enumerate() {
            let nav_i = 3 * maze_i;
            for (maze_j, node) in row.iter().enumerate() {
                let nav_j = 3 * maze_j;
                let mut pipe_indices = Vec::with_capacity(5);
                if node.right_ok || node.left_ok || node.down_ok || node.up_ok {
                    pipe_indices.push(obj.to_flat_idx(&(nav_j + 1, nav_i + 1)));
                }
                if node.right_ok {
                    pipe_indices.push(obj.to_flat_idx(&(nav_j + 2, nav_i + 1)));
                }
                if node.up_ok {
                    pipe_indices.push(obj.to_flat_idx(&(nav_j + 1, nav_i)));
                }
                if node.left_ok {
                    pipe_indices.push(obj.to_flat_idx(&(nav_j, nav_i + 1)));
                }
                if node.down_ok {
                    pipe_indices.push(obj.to_flat_idx(&(nav_j + 1, nav_i + 2)));
                }
                for idx in pipe_indices.iter() {
                    obj.node_statuses[*idx] = NodeStatus::PipeNotVisited;
                }
            }
        }

        // Set starting node
        let start_coord = (3 * maze.start.0 + 1, 3 * maze.start.1 + 1);
        let start_idx = obj.to_flat_idx(&start_coord);
        obj.node_statuses[start_idx] = NodeStatus::PipeVisited;
        obj.active_nodes.push(start_coord);

        obj
    }

    fn to_flat_idx(&self, coord: &(usize, usize)) -> usize {
        let col = coord.0;
        let row = coord.1;
        self.num_cols * row + col
    }

    fn get_adjacent_matching(
        &self,
        coord: &(usize, usize),
        node_type: NodeStatus,
        diagonal_ok: bool,
    ) -> Vec<(usize, usize)> {
        let col = coord.0;
        let row = coord.1;

        let at_left_edge = col == 0;
        let at_right_edge = col + 1 == self.num_cols;
        let at_top_edge = row == 0;
        let at_bottom_edge = row + 1 == self.num_rows;

        let mut check_indices = Vec::with_capacity(8);
        if diagonal_ok && !at_top_edge && !at_left_edge {
            check_indices.push((col - 1, row - 1));
        }
        if !at_top_edge {
            check_indices.push((col, row - 1));
        }
        if diagonal_ok && !at_top_edge && !at_right_edge {
            check_indices.push((col + 1, row - 1));
        }
        if !at_left_edge {
            check_indices.push((col - 1, row));
        }
        if !at_right_edge {
            check_indices.push((col + 1, row));
        }
        if diagonal_ok && !at_bottom_edge && !at_left_edge {
            check_indices.push((col - 1, row + 1));
        }
        if !at_bottom_edge {
            check_indices.push((col, row + 1));
        }
        if diagonal_ok && !at_bottom_edge && !at_right_edge {
            check_indices.push((col + 1, row + 1));
        }
        check_indices
            .iter()
            .filter(|c| self.node_statuses[self.to_flat_idx(c)] == node_type)
            .cloned()
            .collect()
    }

    pub fn advance_maze_nav(&mut self) -> usize {
        let mut nodes_advanced = 0;
        let mut next_active_nodes = Vec::with_capacity(self.active_nodes.capacity());
        for coord in self.active_nodes.iter() {
            let pathable_coords =
                self.get_adjacent_matching(coord, NodeStatus::PipeNotVisited, false);
            for pc in pathable_coords.iter() {
                let idx = self.to_flat_idx(pc);
                next_active_nodes.push(*pc);
                self.node_statuses[idx] = NodeStatus::PipeVisited;
                nodes_advanced += 1;
            }
        }
        self.active_nodes = next_active_nodes;
        if self.active_nodes.is_empty() {
            return 0;
        }
        nodes_advanced
    }

    pub fn reset_active_nodes(&mut self) {
        self.active_nodes = Vec::new();
    }

    fn get_next_outer(&self) -> Option<(usize, usize)> {
        let mut row = 0;
        let mut col = 0;
        while col + 1 < self.num_cols {
            if self.node_statuses[self.to_flat_idx(&(col, row))] == NodeStatus::None {
                return Some((col, row));
            }
            col += 1;
        }
        while row + 1 < self.num_rows {
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
        let mut next_active_nodes = Vec::with_capacity(self.active_nodes.capacity());
        for coord in self.active_nodes.iter() {
            let pathable_coords = self.get_adjacent_matching(coord, NodeStatus::None, true);
            for pc in pathable_coords.iter() {
                next_active_nodes.push(*pc);
                let idx = self.to_flat_idx(pc);
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

    pub fn num_maze_nodes(&self) -> usize {
        self.node_statuses.len() / 9
    }

    pub fn count_traversed_maze_nodes(&self) -> usize {
        let mut total = 0;
        for i in 0..self.num_rows / 3 {
            let row = 3 * i + 1;
            for j in 0..self.num_cols / 3 {
                let col = 3 * j + 1;
                let num_visited = self
                    .get_adjacent_matching(&(col, row), NodeStatus::PipeVisited, false)
                    .len();
                if num_visited >= 2 {
                    total += 1;
                }
            }
        }
        total
    }

    pub fn count_empty_outer_nodes(&self) -> usize {
        let mut total = 0;
        for i in 0..self.num_rows / 3 {
            let row = 3 * i + 1;
            for j in 0..self.num_cols / 3 {
                let col = 3 * j + 1;
                let num_visited_pipes = self
                    .get_adjacent_matching(&(col, row), NodeStatus::PipeVisited, true)
                    .len();
                if num_visited_pipes >= 2 {
                    continue;
                }
                let num_empty = self
                    .get_adjacent_matching(&(col, row), NodeStatus::EmptyVisited, true)
                    .len();
                if num_empty > 1 {
                    total += 1;
                }
            }
        }
        total
    }
}

fn draw_node_status(
    dt: &mut DrawTarget,
    nav: &MazeNavigation,
    node_status: NodeStatus,
    red: u8,
    green: u8,
    blue: u8,
) {
    let num_x = nav.num_cols as f32;
    let num_y = nav.num_rows as f32;
    let grid_width = dt.width() as f32 / num_x;
    let grid_height = dt.height() as f32 / num_y;

    let mut pb = PathBuilder::new();
    for (idx, status) in nav.node_statuses.iter().enumerate() {
        if *status == node_status {
            let row = idx / nav.num_cols;
            let col = idx % nav.num_cols;
            let x = grid_width * col as f32;
            let y = grid_height * row as f32;
            pb.rect(x, y, grid_width, grid_height);
        }
    }
    let path = pb.finish();
    dt.fill(
        &path,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(
            0xa0, red, green, blue,
        )),
        &DrawOptions::new(),
    );
}

fn draw_navigation(dt: &mut DrawTarget, nav: &MazeNavigation) {
    draw_node_status(dt, nav, NodeStatus::PipeNotVisited, 0x00, 0x00, 0xff);
    draw_node_status(dt, nav, NodeStatus::PipeVisited, 0x00, 0xff, 0x00);
    draw_node_status(dt, nav, NodeStatus::EmptyVisited, 0x00, 0xff, 0xff);

    // Draw active nodes
    let num_x = nav.num_cols as f32;
    let num_y = nav.num_rows as f32;
    let grid_width = dt.width() as f32 / num_x;
    let grid_height = dt.height() as f32 / num_y;

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

#[derive(Debug, Default)]
struct PerformanceTimer
{
    pub clear: u128,
    pub p1_advance: u128,
    pub p1_draw: u128,
    pub p2_advance: u128,
    pub p2_draw: u128,
    pub win_update: u128,
}

pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();
    let maze_refresh_ms: u64 = args[1].parse().unwrap();
    let pipe_maze = PipeMaze::from_file_data(&lines);
    let mut navigation = MazeNavigation::new(&pipe_maze);

    let do_print_time = args.contains(&"-t".to_owned()) || args.contains(&"--time".to_owned());

    let default_window_refresh_ms = 20;
    let window_refresh_ms = if maze_refresh_ms < default_window_refresh_ms {
        maze_refresh_ms
    } else {
        default_window_refresh_ms
    };
    let window_refresh_us = 1000 * window_refresh_ms;
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
    window
        .update_with_buffer(dt.get_data(), win_width, win_height)
        .unwrap();
    sleep(Duration::from_millis(window_refresh_ms));

    let mut iter = 1;
    let mut maze_steps_taken = 0;
    let mut part_1_solved = false;
    let mut part_2_solved = false;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let iter_start = Instant::now();
        let mut perf_timer = PerformanceTimer::default();

        let should_refresh =
            !(part_1_solved && part_2_solved) && iter % (maze_refresh_ms / window_refresh_ms) == 0;

        if should_refresh {
            let start_clear = Instant::now();
            dt.clear(SolidSource::from_unpremultiplied_argb(
                0x00, 0x00, 0x00, 0x00,
            ));
            perf_timer.clear = start_clear.elapsed().as_micros();

            if !part_1_solved {
                let start_nav = Instant::now();
                let num_updated = navigation.advance_maze_nav();
                perf_timer.p1_advance = start_nav.elapsed().as_micros();
                if num_updated == 0 {
                    println!("Steps in longest loop (part 1): {}", (maze_steps_taken / 3));
                    part_1_solved = true;
                    navigation.reset_active_nodes();
                } else {
                    maze_steps_taken += 1;
                }
                let start_draw = Instant::now();
                draw_navigation(&mut dt, &navigation);
                perf_timer.p1_draw = start_draw.elapsed().as_micros();
            }
            if part_1_solved && !part_2_solved {
                let start_nav = Instant::now();
                if navigation.advance_outer_nav().is_none() {
                    let num_enclosed = navigation.num_maze_nodes()
                        - navigation.count_traversed_maze_nodes()
                        - navigation.count_empty_outer_nodes();
                    println!("Spaces enclosed (part 2): {}", num_enclosed);
                    part_2_solved = true;
                }
                perf_timer.p2_advance = start_nav.elapsed().as_micros();
                let start_draw = Instant::now();
                draw_navigation(&mut dt, &navigation);
                perf_timer.p2_draw = start_draw.elapsed().as_micros();
            }

            let start_update = Instant::now();
            window
                .update_with_buffer(dt.get_data(), win_width, win_height)
                .unwrap();
            perf_timer.win_update = start_update.elapsed().as_micros();
        }

        if do_print_time {
            print!("\r{:?}", perf_timer);
        }

        let iter_duration = iter_start.elapsed().as_micros() as u64;
        let wait_time_us = if iter_duration < window_refresh_us {
            window_refresh_us - iter_duration
        }
        else {
            10
        };
        sleep(Duration::from_micros(wait_time_us));
        iter += 1;
    }
}
