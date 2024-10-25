use crate::view::cell::Cell;
use std::{cell::RefCell, rc::Rc};

pub trait MazeSolver {
    fn solve(&mut self, tart: (usize, usize), end: (usize, usize)) -> Option<Rc<RefCell<Cell>>>;
}
