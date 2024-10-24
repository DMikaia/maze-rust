use super::cell::Cell;
use crate::view::canvas::GameCanvas;
use rand::{seq::SliceRandom, thread_rng};
use std::{cell::RefCell, rc::Rc};

pub struct Maze {
    pub grid: Vec<Cell>,
    pub screen: (u32, u32),
    pub size: usize,
}

impl Maze {
    pub fn new(size: usize, screen: (u32, u32), canvas: Rc<RefCell<GameCanvas>>) -> Self {
        let grid: Vec<Cell> = (0..size * size)
            .map(|i| {
                let x = i % size;
                let y = i / size;
                Cell::new(x, y, Rc::clone(&canvas))
            })
            .collect();

        Self { grid, screen, size }
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.size && y < self.size
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
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
