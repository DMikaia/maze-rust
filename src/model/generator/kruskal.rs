use super::traits::MazeGenerator;
use crate::helpers::union_find::UnionFind;
use crate::{model::maze::Maze, view::cell::Cell};
use rand::{seq::SliceRandom, thread_rng};
use std::{cell::RefCell, rc::Rc};

pub struct KruskalGenerator {
    edges: Vec<((usize, usize), (usize, usize))>,
    uf: UnionFind,
    current_edge_index: usize,
    current_index: Option<Rc<RefCell<Cell>>>,
}

fn generate_edges(size: usize) -> Vec<((usize, usize), (usize, usize))> {
    let mut edges: Vec<((usize, usize), (usize, usize))> = Vec::new();

    for i in 0..size {
        for j in 0..size {
            if j + 1 < size {
                edges.push(((i, j), (i, j + 1)));
            }
            if i + 1 < size {
                edges.push(((i, j), (i + 1, j)));
            }
            if j > 0 {
                edges.push(((i, j), (i, j - 1)));
            }
            if i > 0 {
                edges.push(((i, j), (i - 1, j)));
            }
        }
    }

    edges
}

impl KruskalGenerator {
    pub fn new(maze: &Maze) -> Self {
        let mut edges: Vec<((usize, usize), (usize, usize))> = generate_edges(maze.size);
        let mut rng = thread_rng();
        edges.shuffle(&mut rng);

        let uf: UnionFind = UnionFind::new(maze.size * maze.size);

        Self {
            edges,
            uf,
            current_edge_index: 0,
            current_index: None,
        }
    }
}

impl MazeGenerator for KruskalGenerator {
    fn generate(&mut self, maze: &Maze) -> bool {
        if self.uf.is_fully_connected() || self.current_edge_index >= self.edges.len() {
            return false;
        }

        let ((i1, j1), (i2, j2)) = self.edges[self.current_edge_index];
        let index_1 = maze.get_index(i1, j1);
        let index_2 = maze.get_index(i2, j2);

        self.current_index = Some(maze.grid[index_1].clone());

        if self.uf.find(index_1) != self.uf.find(index_2) {
            let mut cell_1 = maze.grid[index_1].borrow_mut();
            let mut cell_2 = maze.grid[index_2].borrow_mut();

            maze.remove_wall(&mut cell_1, &mut cell_2);
            self.uf.union(index_1, index_2);
        }

        self.current_edge_index += 1;

        true
    }

    fn get_current_cell(&self) -> Option<Rc<RefCell<Cell>>> {
        self.current_index.clone()
    }
}
