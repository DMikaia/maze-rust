use super::traits::MazeGenerator;
use crate::model::maze::Maze;
use crate::view::cell::Cell;
use std::{cell::RefCell, rc::Rc};

pub struct DfsGenerator {
    pub stack: Vec<Rc<RefCell<Cell>>>,
}

impl DfsGenerator {
    pub fn new(starting: Rc<RefCell<Cell>>) -> Self {
        Self {
            stack: vec![Rc::clone(&starting)],
        }
    }
}

impl MazeGenerator for DfsGenerator {
    fn generate(&mut self, maze: &Maze) -> bool {
        while !self.stack.is_empty() {
            let current = self.stack.pop().unwrap();

            let mut current_cell = current.borrow_mut();
            current_cell.set_visited();

            if let Some(next) = maze.get_random_neighbor(current_cell.i, current_cell.j) {
                let mut next_cell = next.borrow_mut();
                next_cell.set_visited();

                self.stack.push(current.clone());

                maze.remove_wall(&mut current_cell, &mut next_cell);

                self.stack.push(next.clone());
            } else {
                self.stack.pop();
            }

            return true;
        }

        false
    }

    fn get_current_cell(&self) -> Option<Rc<RefCell<Cell>>> {
        self.stack.last().cloned()
    }
}
