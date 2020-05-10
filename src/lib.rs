mod utils;

use std::fmt;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn new_game(x: usize, y: usize) { State::new(x, y); }

// #[wasm_bindgen]
pub struct State {
    pub grid: Vec<Vec<bool>>,
    pub new_grid: Vec<bool>,
    pub width: usize,
    pub height: usize,
    pub generation: u32,
}


impl State {
    fn get_index(&self, x: usize, y: usize) -> usize {
        (x + self.width * y) as usize
    }

    fn get_coordinates(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    pub fn new(width: usize, height: usize) -> State {
        let mut grid = Vec::<Vec<bool>>::with_capacity(width);
        let mut new_grid = Vec::<bool>::with_capacity(width * height);
        for i in 0..width {
            let v = i == 2;
            grid.push((0..height).map(|_| v).collect());
        };
        for i in 0..width * height {
            let v = i % width == 2;
            new_grid.push(v);
        }
        return State {
            grid,
            new_grid,
            width,
            height,
            generation: 0,
        };
    }

    pub fn next(&mut self) {
        let new_grid: Vec<Vec<bool>> = (0..self.width).map(|x| {
            (0..self.height).map(|y| {
                let mut neighbors: u8 = 0;
                // add x - 1 neighbors (left neighbors)
                if x > 0 {
                    neighbors += if y > 0 && self.grid[x - 1][y - 1] { 1 } else { 0 };
                    neighbors += if self.grid[x - 1][y] { 1 } else { 0 };
                    neighbors += if y < (self.height - 1) && self.grid[x - 1][y + 1] { 1 } else { 0 };
                }

                // add x neighbors (top and bottom)
                neighbors += if y > 0 && self.grid[x][y - 1] { 1 } else { 0 };
                neighbors += if y < (self.height - 1) && self.grid[x][y + 1] { 1 } else { 0 };

                // add x + 1 neighbors (right neighbors)
                if x < self.width - 1 {
                    neighbors += if y > 0 && self.grid[x + 1][y - 1] { 1 } else { 0 };
                    neighbors += if self.grid[x + 1][y] { 1 } else { 0 };
                    neighbors += if y < (self.height - 1) && self.grid[x + 1][y + 1] { 1 } else { 0 };
                }

                let is_alive =
                    // Alive cells stay alive if 2 or 3 neighbors
                    (self.grid[x][y] && (neighbors == 2 || neighbors == 3)) ||
                        // Dead cells become alive if 3 neighbors
                        (!self.grid[x][y] && neighbors == 3);
                return is_alive;
            }).collect()
        }).collect();

        self.grid = new_grid;
        self.generation += 1;
    }

    pub fn new_next(&mut self) {
        let new_grid: Vec<bool> = (0..self.width * self.height).map(|idx| {
            let (x, y) = self.get_coordinates(idx);
            let mut neighbors: u8 = 0;
            // add x - 1 neighbors (left neighbors)
            if x > 0 {
                neighbors += if y > 0 && self.new_grid[self.get_index(x - 1, y - 1)] { 1 } else { 0 };
                neighbors += if self.new_grid[self.get_index(x - 1, y)] { 1 } else { 0 };
                neighbors += if y < (self.height - 1) && self.new_grid[self.get_index(x - 1, y + 1)] { 1 } else { 0 };
            }

            // add x neighbors (top and bottom)
            neighbors += if y > 0 && self.new_grid[self.get_index(x, y - 1)] { 1 } else { 0 };
            neighbors += if y < (self.height - 1) && self.new_grid[self.get_index(x, y + 1)] { 1 } else { 0 };

            // add x + 1 neighbors (right neighbors)
            if x < self.width - 1 {
                neighbors += if y > 0 && self.new_grid[self.get_index(x + 1, y - 1)] { 1 } else { 0 };
                neighbors += if self.new_grid[self.get_index(x + 1, y)] { 1 } else { 0 };
                neighbors += if y < (self.height - 1) && self.new_grid[self.get_index(x + 1, y + 1)] { 1 } else { 0 };
            }

            let is_alive =
                // Alive cells stay alive if 2 or 3 neighbors
                (self.new_grid[idx] && (neighbors == 2 || neighbors == 3)) ||
                    // Dead cells become alive if 3 neighbors
                    (!self.new_grid[idx] && neighbors == 3);
            return is_alive;
        }).collect();

        self.new_grid = new_grid;
        self.generation += 1;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}


const EMPTY_STR: char = ' ';

const CAR_RET: char = '\n';
const EMPTY_SQUARE: char = '◻';
const FILLED_SQUARE: char = '◼';

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (0..self.width * self.height).for_each(|i| {
            write!(f, "{}", if self.new_grid[i] { FILLED_SQUARE } else { EMPTY_SQUARE }).unwrap();
            if (i % self.width) == self.width - 1 {
                write!(f, "{}", CAR_RET).unwrap();
            } else {
                write!(f, "{}", EMPTY_STR).unwrap();
            }
        });
        // (0..self.height).for_each(|y| {
        //     (0..self.width).for_each(|x| {
        //         write!(f, "{}", if self.grid[x][y] { FILLED_SQUARE } else { EMPTY_SQUARE }).unwrap();
        //         if x < (self.width - 1) {
        //             write!(f, "{}", EMPTY_STR).unwrap();
        //         }
        //     });
        //     write!(f, "{}", CAR_RET).unwrap();
        // });
        Ok(())
    }
}