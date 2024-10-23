use crate::model::maze::Maze;
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

    fn draw_maze(&self, canvas: &mut Canvas<Window>, params: &DrawingParams, maze: &Maze);
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
        let (_, cell_height) = params.cell;
        let (screen_width, _) = params.screen;

        let y = i * cell_height;
        self.draw_line_between_points(
            canvas,
            Coordinates::new(0, y),
            Coordinates::new(screen_width, y),
        );
    }

    fn draw_vertical_line(&self, canvas: &mut Canvas<Window>, params: &DrawingParams, i: i32) {
        let (cell_width, _) = params.cell;
        let (_, screen_height) = params.screen;

        let x = i * cell_width;
        self.draw_line_between_points(
            canvas,
            Coordinates::new(x, 0),
            Coordinates::new(x, screen_height),
        );
    }

    fn draw_maze(&self, canvas: &mut Canvas<Window>, params: &DrawingParams, maze: &Maze) {
        let (cell_width, cell_height) = params.cell;

        canvas.set_draw_color(Color::RGB(216, 200, 150));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(175, 155, 90));

        for i in 0..=maze.size {
            self.draw_horizontal_line(canvas, params, i as i32);
            self.draw_vertical_line(canvas, params, i as i32);
        }

        for y in 0..maze.size {
            for x in 0..maze.size {
                let start_x = (x as i32) * cell_width;
                let start_y = (y as i32) * cell_height;

                let rect =
                    sdl2::rect::Rect::new(start_x, start_y, cell_width as u32, cell_height as u32);

                if !maze.grid[y][x] {
                    if x == 0 || !maze.grid[y][x - 1] {
                        canvas
                            .draw_line(
                                Point::new(start_x, start_y),
                                Point::new(start_x, start_y + cell_height),
                            )
                            .unwrap();
                    }

                    if y > 0 && !maze.grid[y - 1][x] {
                        canvas
                            .draw_line(
                                Point::new(start_x, start_y),
                                Point::new(start_x + cell_width, start_y),
                            )
                            .unwrap();
                    }
                } else {
                    canvas.set_draw_color(Color::RGB(7, 6, 5));
                    canvas.fill_rect(rect).unwrap();
                }
            }
        }
    }
}
