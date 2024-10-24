use crate::Maze;
use std::{thread, time};

// impl Maze {
//     pub fn animate_solution(&self, path: Vec<(usize, usize)>) {
//         let sleep_duration = time::Duration::from_millis(500);
//         let mut current_position = (0, 0);

//         for &next_position in &path {
//             print!("{}[2J", 27 as char);
//             print!("{}[H", 27 as char);

//             self.print_with_bot(current_position);

//             thread::sleep(sleep_duration);

//             current_position = next_position;
//         }

//         print!("{}[2J", 27 as char);
//         self.print_with_bot(current_position);
//     }

//     pub fn print_with_bot(&self, bot_position: (usize, usize)) {
//         for y in 0..self.size {
//             for x in 0..self.size {
//                 if bot_position == (x, y) {
//                     print!("O");
//                 } else if self.grid[y][x] {
//                     print!(" ");
//                 } else {
//                     print!("#");
//                 }
//             }
//             println!();
//         }
//     }
// }
