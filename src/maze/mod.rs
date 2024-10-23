use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<bool>>,
    pub visited: Vec<Vec<bool>>,
    stack: Vec<(usize, usize)>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![false; width]; height],
            visited: vec![vec![false; width]; height],
            stack: Vec::new(),
        }
    }

    fn check_neighbors_pair(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let directions: [(isize, isize); 4] = [(0, 2), (0, -2), (2, 0), (-2, 0)];

        for (dx, dy) in directions.iter() {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if self.in_bounds(nx, ny) && !self.visited[ny][nx] {
                neighbors.push((nx, ny));
            }
        }

        neighbors.shuffle(&mut thread_rng());
        neighbors
    }

    pub fn generate_maze(&mut self, start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
        self.stack.push((start_x, start_y));
        self.visited[start_y][start_x] = true;
        self.grid[start_y][start_x] = true;

        while let Some((x, y)) = self.stack.last().copied() {
            let neighbors = self.check_neighbors_pair(x, y);
            if let Some(&(nx, ny)) = neighbors.first() {
                let mid_x = (x + nx) / 2;
                let mid_y = (y + ny) / 2;

                if self.in_bounds(mid_x, mid_y) {
                    self.grid[mid_y][mid_x] = true;
                }
                if self.in_bounds(ny, nx) {
                    self.grid[ny][nx] = true;
                    self.visited[ny][nx] = true;
                    self.stack.push((nx, ny));
                }
            } else {
                self.stack.pop();
            }
        }

        vec![(start_x, start_y), (self.width - 1, self.height - 1)]
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}
