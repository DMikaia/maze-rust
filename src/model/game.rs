use std::{thread, time::Duration};

use crate::view::board::Renderer;
use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, video::Window, EventPump};

use super::maze::Maze;

pub struct Game {
    pub canvas: Canvas<Window>,
    pub cell: u32,
    pub event_queue: EventPump,
    pub renderer: Renderer,
    pub maze: Maze,
}

impl Game {
    pub fn new(
        cell: u32,
        sdl_context: &sdl2::Sdl,
        screen_size: (u32, u32),
    ) -> Result<Self, String> {
        let video_subsystem = sdl_context.video()?;

        // Calculate the adjusted screen dimensions based on the number of cells
        let adjusted_width = (screen_size.0 / cell) * cell;
        let adjusted_height = (screen_size.1 / cell) * cell;

        let window = video_subsystem
            .window("Maze", adjusted_width, adjusted_height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        let renderer = Renderer::new(
            Color::RGB(216, 200, 150),
            Rect::new(0, 0, adjusted_width, adjusted_height),
        );

        let event_queue = sdl_context.event_pump()?;

        let maze = Maze::new(cell as usize);

        Ok(Game {
            cell,
            canvas,
            event_queue,
            renderer,
            maze,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut running = true;
        let mut maze_generated = false;

        self.maze.stack.push((0, 0));
        self.maze.visited[0][0] = true;
        self.maze.grid[0][0] = true;

        while running {
            self.process_events(&mut running);

            if !maze_generated {
                maze_generated = !self.maze.generate_maze_step();
            }

            self.renderer
                .render_board(&mut self.canvas, self.cell as i32, &self.maze);

            self.canvas.present();

            thread::sleep(Duration::from_millis(50));
        }

        self.maze.display_maze();
        Ok(())
    }

    fn process_events(&mut self, running: &mut bool) {
        for event in self.event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => *running = false,
                _ => {}
            }
        }
    }
}
