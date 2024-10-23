mod bot;
mod maze;
mod model;
mod solver;

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

    let screen_size: (u32, u32) = (800, 600);
    let sdl_context = init()?;

    let mut game: Result<Game, String> = Game::new(&sdl_context, screen_size);

    Ok(())
}
