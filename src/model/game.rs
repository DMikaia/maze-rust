use std::{cell::RefCell, rc::Rc, thread, time::Duration};

use crate::view::{canvas::GameCanvas, renderer::Renderer};
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, video::Window, EventPump};

use super::maze::Maze;

pub struct Game {
    pub cell: u32,
    pub event_queue: EventPump,
    pub game_canvas: Rc<RefCell<GameCanvas>>,
    pub renderer: Renderer,
    pub maze: Maze,
}

fn adjust_screen_size(screen_size: (u32, u32), cell: u32) -> (u32, u32) {
    let mut screen_width = screen_size.0;
    let mut screen_height = screen_size.1;

    if screen_size.0 % cell != 0 {
        screen_width = (screen_size.0 / cell) * cell;
    }

    if screen_size.1 % cell != 0 {
        screen_height = (screen_size.1 / cell) * cell;
    }

    (screen_width, screen_height)
}

impl Game {
    pub fn new(
        cell: u32,
        sdl_context: &sdl2::Sdl,
        screen_size: (u32, u32),
    ) -> Result<Self, String> {
        let (screen_width, screen_height) = adjust_screen_size(screen_size, cell);

        let game_canvas = Rc::new(RefCell::new(GameCanvas::new(
            sdl_context,
            (screen_width, screen_height),
        )));

        let renderer = Renderer::new(
            Color::RGB(216, 200, 150),
            Rect::new(0, 0, screen_width, screen_height),
            cell as i32,
            Rc::clone(&game_canvas),
        );

        let event_queue = sdl_context.event_pump()?;

        let maze = Maze::new(
            cell as usize,
            (screen_width, screen_height),
            Rc::clone(&game_canvas),
        );

        Ok(Game {
            cell,
            event_queue,
            game_canvas,
            renderer,
            maze,
        })
    }

    // pub fn run(&mut self) -> Result<(), String> {
    //     let mut running = true;
    //     let mut maze_generated = false;

    //     self.maze.stack.push((0, 0));
    //     self.maze.visited[0][0] = true;
    //     self.maze.grid[0][0] = true;

    //     while running {
    //         self.process_events(&mut running);

    //         if !maze_generated {
    //             maze_generated = !self.maze.generate_maze_step();
    //         }

    //         self.renderer
    //             .render_board(&mut self.canvas, self.cell as i32, &self.maze);

    //         self.canvas.present();

    //         thread::sleep(Duration::from_millis(25));
    //     }

    //     Ok(())
    // }

    fn process_events(&mut self, running: &mut bool) {
        for event in self.event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => *running = false,
                _ => {}
            }
        }
    }
}
