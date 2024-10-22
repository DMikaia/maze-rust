use crate::Maze;

impl Maze {
    pub fn check_solver_cell(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && self.grid[y][x]
    }

    pub fn solve_maze(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; self.width]; self.height];
        let mut path: Vec<(usize, usize)> = Vec::new();

        stack.push(start);

        while let Some((x, y)) = stack.pop() {
            if (x, y) == end {
                path.push((x, y));
                return Some(path);
            }

            if !visited[y][x] {
                visited[y][x] = true;
                path.push((x, y));

                let mut valid_neighbors = Vec::new();
                for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let new_x = (x as isize + dx) as usize;
                    let new_y = (y as isize + dy) as usize;

                    if self.in_bounds(new_x, new_y)
                        && self.check_solver_cell(new_x, new_y)
                        && !visited[new_y][new_x]
                    {
                        valid_neighbors.push((new_x, new_y));
                    }
                }

                valid_neighbors.sort_by_key(|&(nx, ny)| {
                    (nx as isize - end.0 as isize).abs() + (ny as isize - end.1 as isize).abs()
                });

                for neighbor in valid_neighbors.into_iter().rev() {
                    stack.push(neighbor);
                }
            } else {
                path.pop();
            }
        }

        None
    }
}
