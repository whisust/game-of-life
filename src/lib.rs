mod utils;

use std::fmt;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

macro_rules! log {
    ($($t: tt)* ) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct State {
    grid: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub generation: u32,
}


#[wasm_bindgen]
impl State {
    fn get_index(&self, x: usize, y: usize) -> usize {
        (x + self.width * y) as usize
    }

    fn get_coordinates(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    pub fn new(width: usize, height: usize) -> State {
        log!("Generating new state with dimensions {} x {}", width, height);
        utils::set_panic_hook(); // we have to plug this in a common code path
        let mut grid = Vec::<Cell>::with_capacity(width * height);
        for i in 0..width * height {
            let v = i % width == 2;
            grid.push(if v { Cell::Alive } else { Cell::Dead });
        }
        return State {
            grid,
            width,
            height,
            generation: 0,
        };
    }

    pub fn new_pristine() -> State {
        let grid = Vec::<Cell>::with_capacity(0);
        return State {
            grid,
            width: 0,
            height: 0,
            generation: 0,
        };
    }

    fn neighbor_count(&self, x: usize, y: usize) -> u8 {
        let mut neighbors: u8 = 0;
        // add x - 1 neighbors (left neighbors)
        if x > 0 {
            if y > 0 {
                neighbors += self.grid[self.get_index(x - 1, y - 1)] as u8;
            }
            neighbors += self.grid[self.get_index(x - 1, y)] as u8;
            if y < (self.height - 1) {
                neighbors += self.grid[self.get_index(x - 1, y + 1)] as u8
            }
        }

        // add x neighbors (top and bottom)
        if y > 0 {
            neighbors += self.grid[self.get_index(x, y - 1)] as u8;
        }
        if y < (self.height - 1) {
            neighbors += self.grid[self.get_index(x, y + 1)] as u8;
        }

        // add x + 1 neighbors (right neighbors)
        if x < self.width - 1 {
            if y > 0 {
                neighbors += self.grid[self.get_index(x + 1, y - 1)] as u8;
            }
            neighbors += self.grid[self.get_index(x + 1, y)] as u8;
            if y < (self.height - 1) {
                neighbors += self.grid[self.get_index(x + 1, y + 1)] as u8;
            }
        }
        neighbors
    }


    pub fn next(&mut self) {
        let new_grid: Vec<Cell> = (0..self.width * self.height).map(|idx| {
            let (x, y) = self.get_coordinates(idx);
            let neighbors = self.neighbor_count(x, y);

            return match (self.grid[idx], neighbors) {
                (Cell::Alive, 2) | (Cell::Alive, 3) | (Cell::Dead, 3) => Cell::Alive,
                _ => Cell::Dead
            };
        }).collect();

        self.grid = new_grid;
        self.generation += 1;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn cells(&self) -> *const Cell {
        self.grid.as_ptr()
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        self.grid = (0..width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        self.grid = (0..self.width * height).map(|_| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: usize, column: usize) {
        let idx = self.get_index(row, column);
        self.grid[idx].toggle();
    }
}

impl State {
    pub fn get_cells(&self) -> &[Cell] {
        &self.grid
    }

    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.grid[idx] = Cell::Alive;
        }
    }
}


const EMPTY_STR: char = ' ';

const CAR_RET: char = '\n';
const EMPTY_SQUARE: char = '◻';
const FILLED_SQUARE: char = '◼';

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (0..self.width * self.height).for_each(|i| {
            write!(f, "{}", if self.grid[i] == Cell::Alive { FILLED_SQUARE } else { EMPTY_SQUARE }).unwrap();
            if (i % self.width) == self.width - 1 {
                write!(f, "{}", CAR_RET).unwrap();
            } else {
                write!(f, "{}", EMPTY_STR).unwrap();
            }
        });
        Ok(())
    }
}