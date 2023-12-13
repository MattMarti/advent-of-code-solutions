use crate::load_file_lines;

use std::thread::sleep;
use std::time::Duration;

use minifb::{Key, Window, WindowOptions};
use raqote::{
    DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle,
};

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
}

enum NavState {
    none,
    active,
    visited,
}

struct MazeNavNode {
    pub coord: (usize, usize),
    pub visited: bool,
}

struct MazeNavigation {}

fn draw_navigation() {}

fn draw_maze(dt: &mut DrawTarget, maze: &PipeMaze) {
    let num_x = maze.nodes.len() as f32;
    let num_y = maze.nodes[0].len() as f32;
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
    //pb.rect(200.0, 100.0, 200.0, 100.0);
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
    let pipe_maze = PipeMaze::from_file_data(&lines);

    let width = 600;
    let height = 600;
    let mut pix_buff: Vec<u32> = vec![0; width * height];
    let mut window = Window::new(
        "Day 10, 2023 - Pipe Maze",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{e}");
    });

    let (win_width, win_height) = window.get_size();
    let mut dt = DrawTarget::new(win_width as i32, win_height as i32);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        dt.clear(SolidSource::from_unpremultiplied_argb(
            0x00, 0x00, 0x00, 0x00,
        ));

        draw_maze(&mut dt, &pipe_maze);

        window
            .update_with_buffer(dt.get_data(), win_width, win_height)
            .unwrap();

        sleep(Duration::from_millis(200));
    }
}
