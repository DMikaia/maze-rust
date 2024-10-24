use super::canvas::GameCanvas;
use crate::{
    helpers::color::colors,
    model::{game, maze::Maze, state::GameState},
    utils::drawing_params::DrawingParams,
};
use sdl2::{pixels::Color, rect::Rect};
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

pub struct Renderer {
    base_color: Color,
    screen_area: Rect,
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

        // Calculate the maximum cell size that fits the screen dimensions
        let cell_width = screen_width / cell_count;
        let cell_height = screen_height / cell_count;

        let drawing_params: DrawingParams =
            DrawingParams::new((cell_width, cell_height), (screen_width, screen_height));

        Self {
            base_color,
            screen_area,
            drawing_params,
            game_canvas: Rc::clone(&game_canvas),
        }
    }

    fn render_generation(&self, game_canvas: &mut GameCanvas, maze: &Maze) {
        for cell in maze.grid.iter() {
            let cell_ref = cell.borrow();
            if let Some(current) = maze.stack.last() {
                if Rc::ptr_eq(&cell, current) {
                    cell_ref.highlight(game_canvas, &self.drawing_params, colors::HIGHLIGHT_COLOR);
                } else {
                    cell_ref.draw(game_canvas, &self.drawing_params, colors::WALL_COLOR);
                }
            } else {
                cell_ref.draw(game_canvas, &self.drawing_params, colors::WALL_COLOR);
            }
        }
    }

    pub fn render_maze(&self, maze: &Maze, state: &GameState) {
        let mut game_canvas: RefMut<'_, GameCanvas> = self.game_canvas.borrow_mut();

        game_canvas.canvas.set_draw_color(self.base_color);
        game_canvas.canvas.clear();

        match state {
            GameState::Generating => {
                self.render_generation(&mut game_canvas, maze);
            }
            _ => {}
        }

        game_canvas.canvas.present();
    }
}
