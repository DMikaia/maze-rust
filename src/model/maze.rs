use super::cell::Cell;
use crate::helpers::position::in_bounds;
use rand::{seq::SliceRandom, thread_rng};
use std::{cell::RefCell, rc::Rc};

pub struct Maze {
    pub grid: Vec<Rc<RefCell<Cell>>>,
    size: usize,
}

impl Maze {
    pub fn new(size: usize) -> Self {
        let grid: Vec<Rc<RefCell<Cell>>> = (0..size * size)
            .map(|i| {
                let x = i % size;
                let y = i / size;
                Cell::new(x, y)
            })
            .collect();

        Self { grid, size }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }

    fn get_all_neighbor_position(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
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

        for (c_x, c_y) in self.get_all_neighbor_position(x, y) {
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

    pub fn remove_wall(&self, current: &mut Cell, next: &mut Cell) {
        let x_diff = current.i as i32 - next.i as i32;
        if x_diff == 1 {
            current.remove_cell_wall(3);
            next.remove_cell_wall(1);
        } else if x_diff == -1 {
            current.remove_cell_wall(1);
            next.remove_cell_wall(3);
        }

        let y_diff = current.j as i32 - next.j as i32;
        if y_diff == 1 {
            current.remove_cell_wall(0);
            next.remove_cell_wall(2);
        } else if y_diff == -1 {
            current.remove_cell_wall(2);
            next.remove_cell_wall(0);
        }
    }
}
