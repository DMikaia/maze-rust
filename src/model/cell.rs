use std::{cell::RefCell, rc::Rc};

use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{utils::drawing_params::DrawingParams, view::canvas::GameCanvas};

pub struct Cell {
    pub i: usize,
    pub j: usize,
    pub walls: [bool; 4],
    pub game_canvas: Rc<RefCell<GameCanvas>>,
}

impl Cell {
    pub fn new(i: usize, j: usize, game_canvas: Rc<RefCell<GameCanvas>>) -> Self {
        Self {
            i,
            j,
            walls: [true; 4],
            game_canvas,
        }
    }

    pub fn highlight(&self, drawing_params: &DrawingParams, color: Color) {
        let mut game_canvas = self.game_canvas.borrow_mut();
        game_canvas.canvas.set_draw_color(color);

        let rect = Rect::new(
            self.i as i32,
            self.j as i32,
            (drawing_params.screen.0 * drawing_params.cell.0) as u32,
            (drawing_params.screen.1 * drawing_params.cell.1) as u32,
        );

        game_canvas.canvas.fill_rect(rect).ok().unwrap_or_default();
    }

    pub fn draw(&self, drawing_params: &DrawingParams, color: Color) {
        let mut game_canvas = self.game_canvas.borrow_mut();
        game_canvas.canvas.set_draw_color(color);

        let x = self.i as i32 * drawing_params.cell.0 as i32;
        let y = self.i as i32 * drawing_params.cell.1;

        if self.walls[0] {
            game_canvas
                .canvas
                .draw_line(Point::new(x, y), Point::new(x + drawing_params.cell.0, y))
                .ok()
                .unwrap();
        }
        if self.walls[1] {
            game_canvas
                .canvas
                .draw_line(
                    Point::new(x + drawing_params.cell.0, y),
                    Point::new(x + drawing_params.cell.0, y + drawing_params.cell.1),
                )
                .ok()
                .unwrap();
        }
        if self.walls[2] {
            game_canvas
                .canvas
                .draw_line(
                    Point::new(x + drawing_params.cell.0, y + drawing_params.cell.1),
                    Point::new(x, y + drawing_params.cell.1),
                )
                .ok()
                .unwrap();
        }
        if self.walls[3] {
            game_canvas
                .canvas
                .draw_line(Point::new(x, y + drawing_params.cell.1), Point::new(x, y))
                .ok()
                .unwrap();
        }
    }

    pub fn remove_cell_wall(&mut self, i: usize) {
        self.walls[i] = false;
    }
}
