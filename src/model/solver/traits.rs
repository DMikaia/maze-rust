use crate::{model::maze::Maze, view::cell::Cell};
use std::{cell::RefCell, rc::Rc};

pub trait MazeSolver {
    fn solve(
        &mut self,
        start: Rc<RefCell<Cell>>,
        end: (usize, usize),
        maze: &Maze,
    ) -> Option<Vec<Rc<RefCell<Cell>>>>;
}
