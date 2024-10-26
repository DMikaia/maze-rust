mod choice;
mod helpers;
mod model;
mod view;

use choice::{cell_number, get_generator};
use model::{game::Game, maze::Maze};
use sdl2::init;

fn main() -> Result<(), String> {
    let cell = cell_number();
    let maze = Maze::new(cell as usize);
    let generator = get_generator(&maze);

    let screen_size: (u32, u32) = (800, 600);
    let sdl_context = init()?;

    let mut game: Game = Game::new(cell, maze, generator, &sdl_context, screen_size)?;

    game.run()
}
