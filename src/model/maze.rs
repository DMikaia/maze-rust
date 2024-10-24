use super::cell::Cell;
use crate::view::canvas::GameCanvas;
use rand::{seq::SliceRandom, thread_rng};
use std::{cell::RefCell, rc::Rc};

pub struct Maze {
    pub grid: Vec<Rc<RefCell<Cell>>>,
    pub screen: (u32, u32),
    pub size: usize,
    pub stack: Vec<Rc<RefCell<Cell>>>,
}

impl Maze {
    pub fn new(size: usize, screen: (u32, u32)) -> Self {
        let grid: Vec<Rc<RefCell<Cell>>> = (0..size * size)
            .map(|i| {
                let x = i % size;
                let y = i / size;
                Cell::new(x, y)
            })
            .collect();

        Self {
            grid,
            screen,
            size,
            stack: Vec::with_capacity(size),
        }
    }

    pub fn generate(&mut self) {}

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.size && y < self.size
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }

    pub fn check_neighbors(&self, x: usize, y: usize) -> Vec<Rc<RefCell<Cell>>> {
        let mut neighbors: Vec<Rc<RefCell<Cell>>> = Vec::new();

        if self.in_bounds(x, y - 1) && y - 1 > 0 {
            let top = self.grid[self.get_index(x, y - 1)].clone();
            if !top.borrow().visited {
                neighbors.push(top);
            }
        }
        if self.in_bounds(x + 1, y) {
            let right = self.grid[self.get_index(x + 1, y)].clone();
            if !right.borrow().visited {
                neighbors.push(right);
            }
        }
        if self.in_bounds(x, y + 1) {
            let down = self.grid[self.get_index(x, y + 1)].clone();
            if !down.borrow().visited {
                neighbors.push(down);
            }
        }
        if self.in_bounds(x - 1, y) && x - 1 > 0 {
            let left = self.grid[self.get_index(x - 1, y)].clone();
            if !left.borrow().visited {
                neighbors.push(left);
            }
        }

        neighbors.shuffle(&mut thread_rng());
        neighbors
    }

    pub fn remove_wall(current: &mut Cell, next: &mut Cell) {
        let x_diff = (current.i - next.i) as isize;
        if x_diff == 1 {
            current.remove_cell_wall(3);
            next.remove_cell_wall(1);
        } else if x_diff == -1 {
            current.remove_cell_wall(1);
            next.remove_cell_wall(3);
        }

        let y_diff = (current.j - next.j) as isize;
        if y_diff == 1 {
            current.remove_cell_wall(0);
            next.remove_cell_wall(2);
        } else if y_diff == -1 {
            current.remove_cell_wall(2);
            next.remove_cell_wall(0);
        }
    }
}
