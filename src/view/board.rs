use crate::utils::drawing_params::DrawingParams;

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

    pub fn render(&self, canvas: &mut Canvas<Window>, cell: i32) {
        // Set the background color and fill the canvas
        canvas.set_draw_color(self.base_color);
        canvas.fill_rect(self.screen_area).ok().unwrap_or_default();

        let params = DrawingParams::new(
            (self.screen_area.w / cell, self.screen_area.h / cell),
            (self.screen_area.w, self.screen_area.h),
        );

        // Draw the lines (via the LineDrawer trait implementation)
        self.draw_lines(canvas, &params, cell);
    }
}
