use super::Maze;
use crate::{helpers::position::in_bounds, view::cell::Cell};
use rand::{seq::SliceRandom, thread_rng};
use std::{cell::RefCell, rc::Rc};

impl Maze {
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }

    pub fn get_neighbors_position(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut points: Vec<(usize, usize)> = vec![];

        // Top neighbor
        if y as i32 - 1 >= 0 {
            points.push((x, y - 1));
        }

        // Right neighbor
        points.push((x + 1, y));

        // Bottom neighbor
        points.push((x, y + 1));

        // Left neighbor
        if x as i32 - 1 >= 0 {
            points.push((x - 1, y));
        }

        points
    }

    pub fn get_random_neighbor(&self, x: usize, y: usize) -> Option<Rc<RefCell<Cell>>> {
        let mut neighbors: Vec<Rc<RefCell<Cell>>> = Vec::new();

        for (c_x, c_y) in self.get_neighbors_position(x, y) {
            if in_bounds(self.size, (c_x, c_y)) {
                let current = self.grid[self.get_index(c_x, c_y)].clone();

                if !current.borrow().visited {
                    neighbors.push(current);
                }
            }
        }

        if !neighbors.is_empty() {
            neighbors.shuffle(&mut thread_rng());
            Some(neighbors[0].clone())
        } else {
            None
        }
    }

    pub fn get_valid_neighbors(&self, cell: Rc<RefCell<Cell>>) -> Vec<Rc<RefCell<Cell>>> {
        let (x, y) = (cell.borrow().i, cell.borrow().j);
        let walls: [bool; 4] = cell.borrow().get_walls();
        let mut neighbors: Vec<Rc<RefCell<Cell>>> = Vec::new();

        if !walls[0] && in_bounds(self.size, (x, y - 1)) {
            neighbors.push(self.grid[self.get_index(x, y - 1)].clone());
        }
        if !walls[1] && in_bounds(self.size, (x + 1, y)) {
            neighbors.push(self.grid[self.get_index(x + 1, y)].clone());
        }
        if !walls[2] && in_bounds(self.size, (x, y + 1)) {
            neighbors.push(self.grid[self.get_index(x, y + 1)].clone());
        }
        if !walls[3] && in_bounds(self.size, (x - 1, y)) {
            neighbors.push(self.grid[self.get_index(x - 1, y)].clone());
        }

        neighbors
    }
}
