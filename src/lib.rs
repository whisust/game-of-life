mod utils;

use std::fmt;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;
extern crate fixedbitset;

use fixedbitset::FixedBitSet;

macro_rules! log {
    ($($t: tt)* ) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

#[wasm_bindgen]
pub struct State {
    grid: FixedBitSet,
    pub width: usize,
    pub height: usize,
    pub generation: u32,
}

trait PatternIndexes {
    fn cells(pattern: Pattern, x: usize, y: usize) -> Vec<(usize, usize, bool)>;
}

#[wasm_bindgen]
pub enum Pattern {
    Glider,
    Pulsar,
}

impl PatternIndexes for Pattern {
    fn cells(pattern: Pattern, x: usize, y: usize) -> Vec<(usize, usize, bool)> {
        match pattern {
            Pattern::Glider => [
                (x - 1, y - 1, false), (x, y - 1, true), (x + 1, y - 1, false),
                (x - 1, y, false), (x, y, false), (x + 1, y, true),
                (x - 1, y + 1, true), (x, y + 1, true), (x + 1, y + 1, true),
            ].to_vec(),
            _ => Vec::<(usize, usize, bool)>::new()
        }
    }
}

// impl PatternIndexes for Pattern::Pulsar {
//     fn cells(x: usize, y: usize) -> *[(usize, usize, bool)] {
//         return *[
//             (x - 1, y + 1, false), (x, y + 1, true), (x + 1, y + 1, true),
//             (x - 1, y, false), (x, y, false), (x + 1, y, true),
//             (x - 1, y - 1, true), (x, y - 1, true), (x + 1, y - 1, true),
//         ];
//     }
// }


#[wasm_bindgen]
impl State {
    fn get_index(&self, x: usize, y: usize) -> usize {
        (x + self.width * y) as usize
    }

    fn get_coordinates(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn cell_at(&self, x: usize, y: usize) -> bool {
        self.grid[self.get_index(x, y)]
    }

    pub fn new(width: usize, height: usize) -> State {
        log!("Generating new state with dimensions {} x {}", width, height);
        utils::set_panic_hook(); // we have to plug this in a common code path
        let grid = FixedBitSet::with_capacity(width * height);
        return State {
            grid,
            width,
            height,
            generation: 0,
        };
    }

    fn neighbor_count(&self, x: usize, y: usize) -> u8 {
        let mut neighbors: u8 = 0;
        // add x - 1 neighbors (left neighbors)
        if x > 0 {
            if y > 0 {
                neighbors += self.cell_at(x - 1, y - 1) as u8;
            }
            neighbors += self.cell_at(x - 1, y) as u8;
            if y < (self.height - 1) {
                neighbors += self.cell_at(x - 1, y + 1) as u8
            }
        }

        // add x neighbors (top and bottom)
        if y > 0 {
            neighbors += self.cell_at(x, y - 1) as u8;
        }
        if y < (self.height - 1) {
            neighbors += self.cell_at(x, y + 1) as u8;
        }

        // add x + 1 neighbors (right neighbors)
        if x < self.width - 1 {
            if y > 0 {
                neighbors += self.cell_at(x + 1, y - 1) as u8;
            }
            neighbors += self.cell_at(x + 1, y) as u8;
            if y < (self.height - 1) {
                neighbors += self.cell_at(x + 1, y + 1) as u8;
            }
        }
        neighbors
    }


    pub fn next(&mut self) {
        let mut new_grid = FixedBitSet::with_capacity(self.width * self.height);
        for idx in 0..self.width * self.height {
            let (x, y) = self.get_coordinates(idx);
            let neighbors = self.neighbor_count(x, y);

            new_grid.set(idx, match (self.grid[idx], neighbors) {
                (true, 2) | (true, 3) | (false, 3) => true,
                _ => false
            });
        };

        self.grid = new_grid;
        self.generation += 1;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn cells(&self) -> *const u32 {
        self.grid.as_slice().as_ptr()
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        self.grid = FixedBitSet::with_capacity(width * self.height);
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        self.grid = FixedBitSet::with_capacity(self.width * height);
    }

    pub fn toggle_cell(&mut self, row: usize, column: usize) {
        let idx = self.get_index(row, column);
        self.grid.toggle(idx);
    }

    pub fn add_pattern(&mut self, row: usize, column: usize, pattern: Pattern) {
        self.set_pattern(Pattern::cells(pattern, row, column))
    }
}

impl State {
    pub fn get_cells(&self) -> &[u32] {
        &self.grid.as_slice()
    }

    pub fn set_cells(&mut self, cells: &[(usize, usize)], enabled: bool) {
        for (row, col) in cells.iter().cloned() {
            if row <= self.width && col <= self.height {
                let idx = self.get_index(row, col);
                self.grid.set(idx, enabled);
            }
        }
    }

    pub fn set_pattern(&mut self, cells: Vec<(usize, usize, bool)>) {
        for (row, col, enabled) in cells.iter().cloned() {
            if row <= self.width && col <= self.height {
                let idx = self.get_index(row, col);
                self.grid.set(idx, enabled);
            }
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
            write!(f, "{}", if self.grid[i] { FILLED_SQUARE } else { EMPTY_SQUARE }).unwrap();
            if (i % self.width) == self.width - 1 {
                write!(f, "{}", CAR_RET).unwrap();
            } else {
                write!(f, "{}", EMPTY_STR).unwrap();
            }
        });
        Ok(())
    }
}