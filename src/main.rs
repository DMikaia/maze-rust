mod bot;
mod maze;
mod solver;

use maze::Maze;

fn main() {
    let mut maze: Maze = Maze::new(25, 25);

    println!("width: {}, height: {}", maze.width, maze.height);

    let coordinates = maze.generate_maze(0, 0);

    if let Some(path) = maze.dfs(coordinates[0], coordinates[1]) {
        maze.animate_solution(path);
    } else {
        print!("Error");
    }
}
