use crate::{model::maze::Maze, view::cell::Cell};
use std::{cell::RefCell, rc::Rc};

pub trait MazeSolver {
    fn init(&mut self, start: Rc<RefCell<Cell>>, end: (usize, usize));

    fn is_initialized(&self) -> bool;

    fn is_solved(&self) -> bool;

    fn solve(&mut self, maze: &Maze) -> bool;

    fn get_path(&self) -> Vec<Rc<RefCell<Cell>>>;
}
