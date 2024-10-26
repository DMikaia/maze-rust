use crate::model::{
    generator::{dfs::DfsGenerator, kruskal::KruskalGenerator, traits::MazeGenerator},
    maze::Maze,
};
use std::io::stdin;

pub fn cell_number() -> u32 {
    println!("Enter the number of cell: ");
    let mut input: String = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    let cell: u32 = input
        .trim()
        .parse()
        .expect("Failed to parse input into u32");

    cell
}

pub fn get_generator(maze: &Maze) -> Box<dyn MazeGenerator> {
    println!("Choose your generator (1: DFS or 2: Kruskal): ");

    let mut input: String = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    let choice: usize = input
        .trim()
        .parse()
        .expect("Failed to parse input into u32");

    if choice == 1 {
        Box::new(DfsGenerator::new(maze))
    } else {
        Box::new(KruskalGenerator::new(maze))
    }
}
