use crate::{model::maze::Maze, utils::drawing_params::DrawingParams};

use super::line::LineDrawer;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Renderer {
    pub base_color: Color,
    pub screen_area: Rect,
}

impl Renderer {
    pub fn new(base_color: Color, screen_area: Rect) -> Self {
        Self {
            base_color,
            screen_area,
        }
    }

    pub fn render_board(&self, canvas: &mut Canvas<Window>, cell_count: i32, maze: &Maze) {
        let screen_width = self.screen_area.width() as i32;
        let screen_height = self.screen_area.height() as i32;

        // Calculate the maximum cell size that fits the screen dimensions
        let cell_width = screen_width / cell_count;
        let cell_height = screen_height / cell_count;

        let params = DrawingParams::new((cell_width, cell_height), (screen_width, screen_height));

        self.draw_maze(canvas, &params, maze);
    }
}
