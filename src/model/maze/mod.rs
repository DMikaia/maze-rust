pub mod cell;
pub mod neighbor;

use rand::{thread_rng, Rng};

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

        let mut rng = thread_rng();

        let (start_i, start_j) = (
            rng.gen_range(0..(size / 2) - 1),
            rng.gen_range(0..(size / 2) - 1),
        );
        let (end_i, end_j) = (rng.gen_range(size / 2..size), rng.gen_range(size / 2..size));

        let start = start_i + start_j * size;
        let end = end_i + end_j * size;

        println!("start = {}", start);
        println!("end = {}", end);

        Self {
            grid,
            size,
            start,
            end,
        }
    }
}
