use crate::utils::{coordinates::Coordinates, drawing_params::DrawingParams};
use crate::view::board::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait LineDrawer {
    fn draw_line_between_points(
        &self,
        canvas: &mut Canvas<Window>,
        start: Coordinates,
        end: Coordinates,
    );

    fn draw_horizontal_line(&self, canvas: &mut Canvas<Window>, params: &DrawingParams, i: i32);

    fn draw_vertical_line(&self, canvas: &mut Canvas<Window>, params: &DrawingParams, i: i32);

    fn draw_lines(&self, canvas: &mut Canvas<Window>, params: &DrawingParams, cell: i32);
}

impl LineDrawer for Renderer {
    fn draw_line_between_points(
        &self,
        canvas: &mut Canvas<Window>,
        start: Coordinates,
        end: Coordinates,
    ) {
        canvas
            .draw_line(Point::new(start.x, start.y), Point::new(end.x, end.y))
            .ok()
            .unwrap_or_default();
    }

    fn draw_horizontal_line(&self, canvas: &mut Canvas<Window>, params: &DrawingParams, i: i32) {
        let (cell_width, cell_height) = params.cell;
        let (screen_width, _) = params.screen;

        // Start from (0, y) and end at (screen_width, y)
        let y = i * cell_height; // Adjust y by the row number
        self.draw_line_between_points(
            canvas,
            Coordinates::new(0, y),            // Start at x = 0
            Coordinates::new(screen_width, y), // End at x = screen_width
        );
    }

    fn draw_vertical_line(&self, canvas: &mut Canvas<Window>, params: &DrawingParams, i: i32) {
        let (cell_width, cell_height) = params.cell;
        let (_, screen_height) = params.screen;

        // Start from (x, 0) and end at (x, screen_height)
        let x = i * cell_width; // Adjust x by the column number
        self.draw_line_between_points(
            canvas,
            Coordinates::new(x, 0),             // Start at y = 0
            Coordinates::new(x, screen_height), // End at y = screen_height
        );
    }

    fn draw_lines(&self, canvas: &mut Canvas<Window>, params: &DrawingParams, cell: i32) {
        canvas.set_draw_color(Color::RGB(175, 155, 90));

        // Draw horizontal and vertical lines
        for i in 0..=cell {
            // Adjust loop to go from 0 to 5 to ensure borders are drawn
            self.draw_horizontal_line(canvas, params, i);
            self.draw_vertical_line(canvas, params, i);
        }
    }
}
