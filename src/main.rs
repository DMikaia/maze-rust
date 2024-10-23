mod bot;
mod model;
mod solver;
mod utils;
mod view;

use model::{game::Game, maze::Maze};
use sdl2::init;
use std::io::stdin;

fn main() -> Result<(), String> {
    // let mut maze: Maze = Maze::new(25, 25);

    // println!("width: {}, height: {}", maze.width, maze.height);

    // let coordinates = maze.generate_maze(0, 0);

    // if let Some(path) = maze.solve_maze(coordinates[0], coordinates[1]) {
    //     maze.animate_solution(path);
    // } else {
    //     print!("Error");
    // }

    println!("Enter the number of  cells: ");
    let mut input: String = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read line !");

    let cell: u32 = input.trim().parse().expect("The value must be a number !");

    let screen_size: (u32, u32) = (800, 600);
    let sdl_context = init()?;

    let mut game: Game = Game::new(cell, &sdl_context, screen_size)?;

    game.run()
}
