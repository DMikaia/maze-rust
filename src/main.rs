mod bot;
mod helpers;
mod model;
mod solver;
mod utils;
mod view;

use model::{game::Game, maze::Maze};
use sdl2::init;
use std::io::stdin;

fn main() -> Result<(), String> {
    println!("Enter the number of cell: ");
    let mut input: String = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    let cell: u32 = input
        .trim()
        .parse()
        .expect("Failed to parse input into u32");

    let screen_size: (u32, u32) = (800, 600);
    let sdl_context = init()?;

    let mut game: Game = Game::new(cell, &sdl_context, screen_size)?;

    game.run()
}
