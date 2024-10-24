use super::cell::Cell;
use rand::{seq::SliceRandom, thread_rng};
use std::{cell::RefCell, rc::Rc};

pub struct Maze {
    pub grid: Vec<Rc<RefCell<Cell>>>,
    pub stack: Vec<Rc<RefCell<Cell>>>,
    screen: (u32, u32),
    size: usize,
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
            stack: Vec::with_capacity(size),
            screen,
            size,
        }
    }

    pub fn generate(&mut self) -> bool {
        if let Some(current) = self.stack.pop() {
            let mut current_cell = current.borrow_mut();
            current_cell.set_visited();

            if let Some(next) = self.get_random_neighbor(current_cell.i, current_cell.j) {
                let mut next_cell = next.borrow_mut();
                next_cell.set_visited();

                self.stack.push(current.clone());
                self.remove_wall(&mut current_cell, &mut next_cell);

                self.stack.push(next.clone());

                return true;
            }
        }

        false
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.size && y < self.size
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }

    fn get_random_neighbor(&self, x: usize, y: usize) -> Option<Rc<RefCell<Cell>>> {
        let mut neighbors: Vec<Rc<RefCell<Cell>>> = Vec::new();

        if y as i32 - 1 >= 0 {
            if self.in_bounds(x, y - 1) {
                let top = self.grid[self.get_index(x, y - 1)].clone();
                if !top.borrow().visited {
                    neighbors.push(top);
                }
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

        if x as i32 - 1 >= 0 {
            if self.in_bounds(x - 1, y) {
                let left = self.grid[self.get_index(x - 1, y)].clone();
                if !left.borrow().visited {
                    neighbors.push(left);
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

    fn remove_wall(&self, current: &mut Cell, next: &mut Cell) {
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
