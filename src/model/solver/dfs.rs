use crate::{model::maze::Maze, view::cell::Cell};
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
            visited: Vec::with_capacity(size),
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

        let neighbors = maze.get_all_neighbor_position(x, y);
        for (new_x, new_y) in neighbors {
            let new_index = maze.get_index(new_x, new_y);
            let new_neighbor = maze.grid[new_index].clone();

            if self.dfs(new_neighbor, end, maze) {
                return true;
            }
        }

        self.path.pop();

        false
    }
}

impl MazeSolver for DfsSolver {
    fn solve(
        &mut self,
        start: Rc<RefCell<Cell>>,
        end: (usize, usize),
        maze: &Maze,
    ) -> Option<Vec<Rc<RefCell<Cell>>>> {
        if self.dfs(start.clone(), end, maze) {
            Some(self.path.clone())
        } else {
            None
        }
    }
}
