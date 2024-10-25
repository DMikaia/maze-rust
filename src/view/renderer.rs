use super::{canvas::GameCanvas, cell::Cell};
use crate::{
    helpers::color::colors,
    model::{
        generator::traits::MazeGenerator, maze::Maze, solver::traits::MazeSolver, state::GameState,
    },
    utils::drawing_params::DrawingParams,
};
use sdl2::{pixels::Color, rect::Rect};
use std::{cell::RefCell, path, rc::Rc};

pub struct Renderer {
    base_color: Color,
    game_canvas: Rc<RefCell<GameCanvas>>,
    drawing_params: DrawingParams,
}

impl Renderer {
    pub fn new(
        base_color: Color,
        screen_area: Rect,
        cell_count: i32,
        game_canvas: Rc<RefCell<GameCanvas>>,
    ) -> Self {
        let screen_width = screen_area.width() as i32;
        let screen_height = screen_area.height() as i32;

        let cell_width = screen_width / cell_count;
        let cell_height = screen_height / cell_count;

        let drawing_params: DrawingParams =
            DrawingParams::new((cell_width, cell_height), (screen_width, screen_height));

        Self {
            base_color,
            drawing_params,
            game_canvas: Rc::clone(&game_canvas),
        }
    }

    pub fn render_generation(&self, maze: &Maze, generator: &dyn MazeGenerator) {
        let mut game_canvas = self.game_canvas.borrow_mut();

        game_canvas.canvas.set_draw_color(self.base_color);
        game_canvas.canvas.clear();

        for cell in &maze.grid {
            if let Ok(cell_ref) = cell.try_borrow() {
                let stroke = colors::FOREGROUND;
                let mut fill = colors::BACKGROUND_COLOR;

                if let Some(current) = generator.get_current_cell() {
                    if Rc::ptr_eq(&cell, &current) {
                        fill = colors::PRIMARY_COLOR;
                    }
                }

                cell_ref.draw(&mut game_canvas, &self.drawing_params, stroke, fill);
            }
        }

        game_canvas.canvas.present();
    }

    pub fn render_solution(&self, maze: &Maze, solver: &dyn MazeSolver) {
        let mut game_canvas = self.game_canvas.borrow_mut();

        game_canvas.canvas.set_draw_color(colors::BACKGROUND_COLOR);
        game_canvas.canvas.clear();

        let solution_path = solver.get_path();

        for cell in &maze.grid {
            if let Ok(cell_ref) = cell.try_borrow() {
                let stroke = colors::FOREGROUND;
                let mut fill = colors::BACKGROUND_COLOR;

                // Check if the cell is part of the solution path
                if solution_path.contains(cell) {
                    fill = colors::ACCENT;
                }

                cell_ref.draw(&mut game_canvas, &self.drawing_params, stroke, fill);
            }
        }

        game_canvas.canvas.present();
    }

    pub fn render(
        &self,
        maze: &Maze,
        generator: &dyn MazeGenerator,
        solver: &dyn MazeSolver,
        state: &GameState,
    ) {
        match state {
            GameState::Generating => self.render_generation(maze, generator),
            GameState::Solved => self.render_solution(maze, solver),
            _ => {}
        }
    }
}
