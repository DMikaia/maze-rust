use crate::{model::maze::Maze, view::cell::Cell};
use std::{cell::RefCell, rc::Rc};

pub trait MazeGenerator {
    fn generate(&mut self, maze: &Maze) -> bool;

    fn get_current_cell(&self) -> Option<Rc<RefCell<Cell>>>;
}
