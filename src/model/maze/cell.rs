use super::Maze;
use crate::view::cell::Cell;

impl Maze {
    pub fn remove_wall(&self, current: &mut Cell, next: &mut Cell) {
        let x_diff = current.i as i32 - next.i as i32;
        if x_diff == 1 {
            current.remove_cell_wall(3);
            next.remove_cell_wall(1);
        } else if x_diff == -1 {
            current.remove_cell_wall(1);
            next.remove_cell_wall(3);
        }

        let y_diff = current.j as i32 - next.j as i32;
        if y_diff == 1 {
            current.remove_cell_wall(0);
            next.remove_cell_wall(2);
        } else if y_diff == -1 {
            current.remove_cell_wall(2);
            next.remove_cell_wall(0);
        }
    }
}
