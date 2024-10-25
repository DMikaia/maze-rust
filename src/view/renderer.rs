use super::canvas::GameCanvas;
use crate::{
    helpers::color::colors,
    model::{maze::Maze, state::GameState},
    utils::drawing_params::DrawingParams,
};
use sdl2::{pixels::Color, rect::Rect};
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
    thread,
    time::Duration,
};

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

    pub fn render_generation(&self, maze: &Maze) {
        let mut game_canvas = self.game_canvas.borrow_mut();

        game_canvas.canvas.set_draw_color(colors::BACKGROUND_COLOR);
        game_canvas.canvas.clear();

        for cell in &maze.grid {
            if let Ok(cell_ref) = cell.try_borrow() {
                let stroke = colors::FOREGROUND;
                let mut fill = colors::BACKGROUND_COLOR;

                if let Some(current) = maze.stack.last() {
                    if Rc::ptr_eq(&cell, current) {
                        fill = colors::PRIMARY_COLOR;
                    }
                }

                cell_ref.draw(&mut game_canvas, &self.drawing_params, stroke, fill);
            }
        }

        game_canvas.canvas.present();
    }

    pub fn render(&self, maze: &Maze, state: &GameState) {
        match state {
            GameState::Generating => self.render_generation(maze),
            _ => {}
        }
    }
}
