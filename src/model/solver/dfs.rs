use crate::view::cell::Cell;
use std::{cell::RefCell, rc::Rc};

pub struct DfsSolver {
    pub path: Vec<Rc<RefCell<Cell>>>,
    pub visited: Vec<bool>,
}

impl DfsSolver {
    pub fn new(size: usize) -> Self {
        Self {
            path: Vec::new(),
            visited: Vec::with_capacity(size),
        }
    }
}

// impl Maze {
//     fn check_solver_cell(&self, x: usize, y: usize) -> bool {
//         x < self.size && y < self.size && self.grid[y][x]
//     }

//     fn check_neighbor(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
//         let mut neighbors = Vec::new();
//         let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

//         for (dx, dy) in directions.iter() {
//             let nx = (x as isize + dx) as usize;
//             let ny = (y as isize + dy) as usize;
//             if self.in_bounds(nx, ny) && self.check_solver_cell(nx, ny) {
//                 neighbors.push((nx, ny));
//             }
//         }

//         neighbors
//     }

//     fn dfs(
//         &mut self,
//         x: usize,
//         y: usize,
//         end_x: usize,
//         end_y: usize,
//         path: &mut Vec<(usize, usize)>,
//     ) -> bool {
//         if !self.check_solver_cell(x, y) || self.visited[y][x] {
//             return false;
//         }

//         self.visited[y][x] = true;
//         path.push((x, y));

//         if (x, y) == (end_x, end_y) {
//             return true;
//         }

//         let neighbors = self.check_neighbor(x, y);
//         for (new_x, new_y) in neighbors {
//             if self.dfs(new_x, new_y, end_x, end_y, path) {
//                 return true;
//             }
//         }

//         path.pop();
//         false
//     }

//     pub fn solve_maze(
//         &mut self,
//         start: (usize, usize),
//         end: (usize, usize),
//     ) -> Option<Vec<(usize, usize)>> {
//         self.visited = vec![vec![false; self.size]; self.size];
//         let mut path = Vec::new();

//         if self.dfs(start.0, start.1, end.0, end.1, &mut path) {
//             Some(path)
//         } else {
//             None
//         }
//     }
// }
