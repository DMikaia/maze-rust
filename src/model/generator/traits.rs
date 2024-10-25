use crate::model::maze::Maze;

pub trait MazeGenerator {
    fn generate(&mut self, maze: &Maze) -> bool;
}
