use crate::{
    helpers::color::colors, utils::drawing_params::DrawingParams, view::canvas::GameCanvas,
};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct Cell {
    pub i: usize,
    pub j: usize,
    pub walls: [bool; 4],
    pub visited: bool,
}

impl Cell {
    pub fn new(i: usize, j: usize) -> Rc<RefCell<Cell>> {
        Rc::new(RefCell::new(Cell {
            i,
            j,
            walls: [true; 4],
            visited: false,
        }))
    }

    pub fn highlight(
        &self,
        game_canvas: &mut GameCanvas,
        drawing_params: &DrawingParams,
        color: Color,
    ) {
        game_canvas.canvas.set_draw_color(color);

        let rect = Rect::new(
            (self.i as i32 * drawing_params.cell.0 as i32) as i32,
            (self.j as i32 * drawing_params.cell.1 as i32) as i32,
            drawing_params.cell.0 as u32,
            drawing_params.cell.1 as u32,
        );

        game_canvas.canvas.fill_rect(rect).ok().unwrap_or_default();
        self.draw(game_canvas, drawing_params, colors::WALL_COLOR);
    }

    pub fn draw(&self, game_canvas: &mut GameCanvas, drawing_params: &DrawingParams, color: Color) {
        game_canvas.canvas.set_draw_color(color);

        let x = self.i as i32 * drawing_params.cell.0 as i32;
        let y = self.j as i32 * drawing_params.cell.1 as i32;

        // Top wall
        if self.walls[0] && self.j > 0 && self.i > 0 {
            game_canvas
                .canvas
                .draw_line(
                    Point::new(x, y),
                    Point::new(x + drawing_params.cell.0 as i32, y),
                )
                .ok()
                .unwrap();
        }

        // Right wall
        if self.walls[1] {
            game_canvas
                .canvas
                .draw_line(
                    Point::new(x + drawing_params.cell.0 as i32, y),
                    Point::new(
                        x + drawing_params.cell.0 as i32,
                        y + drawing_params.cell.1 as i32,
                    ),
                )
                .ok()
                .unwrap();
        }

        // Bottom wall
        if self.walls[2] {
            game_canvas
                .canvas
                .draw_line(
                    Point::new(
                        x + drawing_params.cell.0 as i32,
                        y + drawing_params.cell.1 as i32,
                    ),
                    Point::new(x, y + drawing_params.cell.1 as i32),
                )
                .ok()
                .unwrap();
        }

        // Left wall
        if self.walls[3] && self.j > 0 && self.i > 0 {
            game_canvas
                .canvas
                .draw_line(
                    Point::new(x, y + drawing_params.cell.1 as i32),
                    Point::new(x, y),
                )
                .ok()
                .unwrap();
        }
    }

    pub fn remove_cell_wall(&mut self, i: usize) {
        self.walls[i] = false;
    }

    pub fn set_visited(&mut self) {
        self.visited = true;
    }
}
