use std::{cell::RefCell, rc::Rc};

use crate::model::{cell::Cell, maze::Maze};

pub trait MazeGenerator {
    fn generate(&mut self, maze: &Maze) -> bool;

    fn get_current_cell(&self) -> Option<Rc<RefCell<Cell>>>;
}
