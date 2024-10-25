use crate::{utils::drawing_params::DrawingParams, view::canvas::GameCanvas};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, PartialEq, Eq)]
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

    pub fn draw(
        &self,
        game_canvas: &mut GameCanvas,
        drawing_params: &DrawingParams,
        stroke: Color,
        fill: Color,
    ) {
        let rect = Rect::new(
            (self.i as i32 * drawing_params.cell.0 as i32) as i32,
            (self.j as i32 * drawing_params.cell.1 as i32) as i32,
            drawing_params.cell.0 as u32,
            drawing_params.cell.1 as u32,
        );

        // Set the fill color and fill the rectangle for the cell
        game_canvas.canvas.set_draw_color(fill);
        game_canvas.canvas.fill_rect(rect).ok().unwrap_or_default();

        // Set the stroke color for the walls
        game_canvas.canvas.set_draw_color(stroke);
        let x = self.i as i32 * drawing_params.cell.0 as i32;
        let y = self.j as i32 * drawing_params.cell.1 as i32;

        // Create a vector to hold the wall lines
        let mut wall_lines: Vec<(Point, Point)> = Vec::new();

        // Check walls and add the corresponding lines to the vector
        if self.walls[0] && self.j > 0 {
            // Top wall
            wall_lines.push((
                Point::new(x, y),
                Point::new(x + drawing_params.cell.0 as i32, y),
            ));
        }

        if self.walls[1] {
            // Right wall
            wall_lines.push((
                Point::new(x + drawing_params.cell.0 as i32, y),
                Point::new(
                    x + drawing_params.cell.0 as i32,
                    y + drawing_params.cell.1 as i32,
                ),
            ));
        }

        if self.walls[2] {
            // Bottom wall
            wall_lines.push((
                Point::new(
                    x + drawing_params.cell.0 as i32,
                    y + drawing_params.cell.1 as i32,
                ),
                Point::new(x, y + drawing_params.cell.1 as i32),
            ));
        }

        if self.walls[3] && self.i > 0 {
            // Left wall
            wall_lines.push((
                Point::new(x, y + drawing_params.cell.1 as i32),
                Point::new(x, y),
            ));
        }

        // Draw all wall lines in a single pass
        for (start, end) in wall_lines {
            game_canvas.canvas.draw_line(start, end).ok().unwrap();
        }
    }

    pub fn remove_cell_wall(&mut self, i: usize) {
        self.walls[i] = false;
    }

    pub fn set_visited(&mut self) {
        self.visited = true;
    }

    pub fn get_walls(&self) -> [bool; 4] {
        self.walls.clone()
    }
}
