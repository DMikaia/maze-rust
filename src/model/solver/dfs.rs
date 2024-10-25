use crate::{
    helpers::position::in_bounds,
    model::maze::{neighbor, Maze},
    view::cell::Cell,
};
use std::{cell::RefCell, rc::Rc};

use super::traits::MazeSolver;

pub struct DfsSolver {
    pub path: Vec<Rc<RefCell<Cell>>>,
    pub visited: Vec<bool>,
}

impl DfsSolver {
    pub fn new(size: usize) -> Self {
        Self {
            path: Vec::new(),
            visited: vec![false; size * size],
        }
    }

    fn dfs(&mut self, current: Rc<RefCell<Cell>>, end: (usize, usize), maze: &Maze) -> bool {
        let (x, y) = (current.borrow().i, current.borrow().j);
        let current_index = maze.get_index(x, y);

        if self.visited[current_index] {
            return false;
        }

        self.visited[current_index] = true;
        self.path.push(current.clone());

        if (x, y) == (end.0, end.1) {
            return true;
        }

        for neighbor in maze.get_valid_neighbors(current.clone()) {
            if self.dfs(neighbor.clone(), end, maze) {
                return true;
            }
        }

        self.path.pop();
        false
    }
}

impl MazeSolver for DfsSolver {
    fn solve(&mut self, start: Rc<RefCell<Cell>>, end: (usize, usize), maze: &Maze) -> bool {
        self.dfs(start.clone(), end, maze)
    }

    fn get_path(&self) -> Vec<Rc<RefCell<Cell>>> {
        self.path.clone()
    }
}
