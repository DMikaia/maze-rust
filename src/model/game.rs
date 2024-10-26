use super::{
    generator::traits::MazeGenerator,
    maze::Maze,
    solver::{dfs::DfsSolver, traits::MazeSolver},
    state::GameState,
};
use crate::{
    helpers::color::colors,
    view::{canvas::GameCanvas, renderer::Renderer},
};
use sdl2::{event::Event, rect::Rect, EventPump};
use std::{
    cell::RefCell,
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

pub struct Game {
    event_queue: EventPump,
    renderer: Renderer,
    maze: Maze,
    generator: Box<dyn MazeGenerator>,
    state: GameState,
    solver: Box<dyn MazeSolver>,
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
        maze: Maze,
        generator: Box<dyn MazeGenerator>,
        sdl_context: &sdl2::Sdl,
        screen_size: (u32, u32),
    ) -> Result<Self, String> {
        let (screen_width, screen_height) = adjust_screen_size(screen_size, cell);

        let event_queue = sdl_context.event_pump()?;

        let game_canvas = Rc::new(RefCell::new(GameCanvas::new(
            sdl_context,
            (screen_width, screen_height),
        )));
        let renderer = Renderer::new(
            colors::BACKGROUND_COLOR,
            Rect::new(0, 0, screen_width, screen_height),
            cell as i32,
            Rc::clone(&game_canvas),
        );

        let solver = Box::new(DfsSolver::new(cell as usize));

        Ok(Game {
            event_queue,
            renderer,
            generator,
            maze,
            state: GameState::Generating,
            solver,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut running = true;
        let mut next_state: Option<GameState> = None;
        let mut state_timer = Instant::now();

        while running {
            self.process_events(&mut running);

            match self.state {
                GameState::Generating => {
                    if !self.generator.generate(&self.maze) {
                        next_state = Some(GameState::Resolving);
                        self.state = GameState::Paused;
                        state_timer = Instant::now();
                    }
                }
                GameState::Resolving => {
                    if !self.solver.is_initialized() {
                        let start = self.maze.grid[self.maze.start].clone();
                        let end = (self.maze.end, self.maze.end);

                        self.solver.init(start, end);
                    }

                    if self.solver.solve(&self.maze) && self.solver.is_solved() {
                        if self.solver.is_solved() {
                            next_state = Some(GameState::Solved);
                            self.state = GameState::Paused;
                            state_timer = Instant::now();
                        }
                    }
                }
                GameState::Paused => {
                    if state_timer.elapsed() >= Duration::from_millis(100) {
                        if let Some(new_state) = next_state.take() {
                            self.state = new_state;
                        }
                    }
                }
                GameState::Solved => {}
            }

            self.renderer
                .render(&self.maze, &*self.generator, &*self.solver, &self.state);
            thread::sleep(Duration::from_millis(42));
        }

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
