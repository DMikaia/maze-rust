mod bot;
mod maze;
mod model;
mod solver;
mod view;

use std::io::stdin;

use maze::Maze;
use model::game::Game;
use sdl2::init;

fn main() -> Result<(), String> {
    // let mut maze: Maze = Maze::new(25, 25);

    // println!("width: {}, height: {}", maze.width, maze.height);

    // let coordinates = maze.generate_maze(0, 0);

    // if let Some(path) = maze.solve_maze(coordinates[0], coordinates[1]) {
    //     maze.animate_solution(path);
    // } else {
    //     print!("Error");
    // }

    print!("Enter the number of  cells: ");
    let mut input: String = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read line !");

    let cell: u32 = input.trim().parse().expect("The value must be a number !");

    let screen_size: (u32, u32) = (800, 600);
    let sdl_context = init()?;

    let mut game: Result<Game, String> = Game::new(cell, &sdl_context, screen_size);

    Ok(())
}
