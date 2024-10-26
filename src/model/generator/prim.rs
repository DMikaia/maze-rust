use super::traits::MazeGenerator;
use crate::{model::maze::Maze, view::cell::Cell};
use rand::{thread_rng, Rng};
use std::{cell::RefCell, rc::Rc};

pub struct PrimGenerator {
    current: Option<Rc<RefCell<Cell>>>,
    walls: Vec<((usize, usize), (usize, usize))>,
}

fn add_walls(walls: &mut Vec<((usize, usize), (usize, usize))>, i: usize, j: usize, size: usize) {
    if j as i32 - 1 >= 0 {
        walls.push(((i, j), (i, j - 1)));
    }

    if i + 1 < size {
        walls.push(((i, j), (i + 1, j)));
    }

    if j + 1 < size {
        walls.push(((i, j), (i, j + 1)));
    }

    if i as i32 - 1 >= 0 {
        walls.push(((i, j), (i - 1, j)));
    }
}

impl PrimGenerator {
    pub fn new(maze: &Maze) -> Self {
        let mut start = maze.grid[maze.start].borrow_mut();
        let (i, j) = (start.i, start.j);

        start.set_visited();

        let mut walls = Vec::new();
        add_walls(&mut walls, i, j, maze.size);

        Self {
            current: Some(maze.grid[maze.start].clone()),
            walls,
        }
    }
}

impl MazeGenerator for PrimGenerator {
    fn generate(&mut self, maze: &Maze) -> bool {
        while !self.walls.is_empty() {
            let mut rng = thread_rng();

            let index = rng.gen_range(0..self.walls.len());

            let ((i1, j1), (i2, j2)) = self.walls[index];

            let mut cell_1 = maze.grid[maze.get_index(i1, j1)].borrow_mut();
            let mut cell_2 = maze.grid[maze.get_index(i2, j2)].borrow_mut();

            if cell_1.is_visited() != cell_2.is_visited() {
                maze.remove_wall(&mut cell_1, &mut cell_2);

                if !cell_1.is_visited() {
                    cell_1.set_visited();
                    add_walls(&mut self.walls, i1, j1, maze.size);
                }

                if !cell_2.is_visited() {
                    cell_2.set_visited();
                    add_walls(&mut self.walls, i2, j2, maze.size);
                }
            }

            self.current = Some(maze.grid[maze.get_index(i1, j1)].clone());
            self.walls.remove(index);

            return true;
        }

        false
    }

    fn get_current_cell(&self) -> Option<Rc<RefCell<Cell>>> {
        self.current.clone()
    }
}
