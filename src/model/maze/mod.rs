pub mod cell;
pub mod neighbor;

use crate::view::cell::Cell;
use std::{cell::RefCell, rc::Rc};

pub struct Maze {
    pub grid: Vec<Rc<RefCell<Cell>>>,
    pub size: usize,
    pub start: usize,
    pub end: usize,
}

impl Maze {
    pub fn new(size: usize) -> Self {
        let grid: Vec<Rc<RefCell<Cell>>> = (0..size * size)
            .map(|i| {
                let x = i % size;
                let y = i / size;
                Cell::new(x, y)
            })
            .collect();

        Self {
            grid,
            size,
            start: 0,
            end: size - 1,
        }
    }
}
