use crate::{model::maze::Maze, view::cell::Cell};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::traits::MazeSolver;

pub struct DfsSolver {
    pub path: Vec<Rc<RefCell<Cell>>>,
    pub visited: Vec<bool>,
    initialized: bool,
    stack: VecDeque<Rc<RefCell<Cell>>>,
    end: (usize, usize),
    solved: bool,
}

impl DfsSolver {
    pub fn new(size: usize) -> Self {
        Self {
            path: Vec::new(),
            visited: vec![false; size * size],
            initialized: false,
            stack: VecDeque::new(),
            end: (0, 0),
            solved: false,
        }
    }
}

impl MazeSolver for DfsSolver {
    fn init(&mut self, start: Rc<RefCell<Cell>>, end: (usize, usize)) {
        self.stack.push_back(start);
        self.end = end;
        self.solved = false;
        self.path.clear();
        self.visited.fill(false);
        self.initialized = true;
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn is_solved(&self) -> bool {
        self.solved
    }

    fn solve(&mut self, maze: &Maze) -> bool {
        if self.solved || self.stack.is_empty() {
            return true;
        }

        if let Some(current) = self.stack.pop_back() {
            let (x, y) = (current.borrow().i, current.borrow().j);
            let current_index = maze.get_index(x, y);

            if self.visited[current_index] {
                return false;
            }

            self.visited[current_index] = true;
            self.path.push(current.clone());

            if (x, y) == self.end {
                self.solved = true;
                return true;
            }

            for neighbor in maze.get_valid_neighbors(current.clone()) {
                let (nx, ny) = (neighbor.borrow().i, neighbor.borrow().j);
                if !self.visited[maze.get_index(nx, ny)] {
                    self.stack.push_back(neighbor);
                }
            }
        }

        false
    }

    fn get_path(&self) -> Vec<Rc<RefCell<Cell>>> {
        self.path.clone()
    }
}
