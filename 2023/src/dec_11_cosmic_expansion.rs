use crate::load_file_lines;

struct StarMap {
    coords: Vec<(usize, usize)>,
}

impl StarMap {
    pub fn from_file_data(file_data: &[String]) -> Self {
        let mut coords = Vec::new();
        for (row, line) in file_data.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    coords.push((col, row));
                }
            }
        }
        Self { coords }
    }

    fn expand_cols(&mut self) {
        self.coords.sort_by(|a, b| a.0.cmp(&b.0));
        for i in 1..self.coords.len() {
            if self.coords[i - 1].0 + 1 < self.coords[i].0 {
                let diff = self.coords[i].0 - self.coords[i - 1].0 - 1;
                for j in i..self.coords.len() {
                    self.coords[j].0 += diff;
                }
            }
        }
    }

    fn expand_rows(&mut self) {
        self.coords.sort_by(|a, b| a.1.cmp(&b.1));
        for i in 1..self.coords.len() {
            if self.coords[i - 1].1 + 1 < self.coords[i].1 {
                let diff = self.coords[i].1 - self.coords[i - 1].1 - 1;
                for j in i..self.coords.len() {
                    self.coords[j].1 += diff;
                }
            }
        }
    }

    pub fn expanded_from(map: &Self) -> Self {
        let mut next = Self {
            coords: map.coords.clone(),
        };
        next.expand_cols();
        next.expand_rows();
        next
    }

    pub fn sum_distances(&self) -> usize {
        let mut total = 0;
        for (i, a) in self.coords.iter().enumerate() {
            for b in self.coords.iter().skip(i) {
                total += calc_dist(a, b);
            }
        }
        total
    }
}

fn calc_dist(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let dist_col = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let dist_row = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
    dist_col + dist_row
}

pub fn run(args: &[String]) {
    let lines = load_file_lines(&args[0]).unwrap();
    let initial_map = StarMap::from_file_data(&lines);
    let expanded_map = StarMap::expanded_from(&initial_map);

    println!(
        "Distance after expansion (part 1): {}",
        expanded_map.sum_distances()
    );
}
