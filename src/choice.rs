use crate::model::{
    generator::{
        dfs::DfsGenerator, kruskal::KruskalGenerator, prim::PrimGenerator, traits::MazeGenerator,
    },
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
    println!("Choose your generator (1: DFS || 2: Kruskal || 3: Prim): ");

    let mut input: String = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    let choice: usize = input
        .trim()
        .parse()
        .expect("Failed to parse input into u32");

    if choice == 1 {
        Box::new(DfsGenerator::new(maze))
    } else if choice == 2 {
        Box::new(KruskalGenerator::new(maze))
    } else {
        Box::new(PrimGenerator::new(maze))
    }
}
