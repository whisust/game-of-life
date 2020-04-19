extern crate clap;

use clap::{Arg, App, value_t_or_exit};
use std::io::{BufWriter, Write};
use std::io;

const EMPTY_STR: &[u8] = " ".as_bytes();

const CAR_RET: &[u8] = "\n".as_bytes();
const X: &[u8] = "x".as_bytes();
const DASH: &[u8] = "-".as_bytes();

struct GameState {
    grid: Vec<Vec<bool>>,
    x: usize,
    y: usize,
}

impl GameState {
    fn new(x: usize, y: usize) -> GameState {
        let mut grid = Vec::<Vec<bool>>::with_capacity(x);
        for i in 0..x {
            let v = i == 2;
            grid.push((0..y).map(|_| v).collect());
        };
        return GameState {
            grid,
            x,
            y,
        };
    }

    fn next(self: GameState) -> GameState {
        let new_grid: Vec<Vec<bool>> = (0..self.x).map(|x| {
            (0..self.y).map(|y| {
                let mut neighbors: u8 = 0;
                // add x - 1 neighbors (left neighbors)
                if x > 0 {
                    neighbors += if y > 0 && self.grid[x - 1][y - 1] { 1 } else { 0 };
                    neighbors += if self.grid[x - 1][y] { 1 } else { 0 };
                    neighbors += if y < (self.y - 1) && self.grid[x - 1][y + 1] { 1 } else { 0 };
                }

                // add x neighbors (top and bottom)
                neighbors += if y > 0 && self.grid[x][y - 1] { 1 } else { 0 };
                neighbors += if y < (self.y - 1) && self.grid[x][y + 1] { 1 } else { 0 };

                // add x + 1 neighbors (right neighbors)
                if x < self.x - 1 {
                    neighbors += if y > 0 && self.grid[x + 1][y - 1] { 1 } else { 0 };
                    neighbors += if self.grid[x + 1][y] { 1 } else { 0 };
                    neighbors += if y < (self.y - 1) && self.grid[x + 1][y + 1] { 1 } else { 0 };
                }

                return (self.grid[x][y] && (neighbors == 2 || neighbors == 3)) || (!self.grid[x][y] && neighbors == 3);
            }).collect()
        }).collect();

        return GameState {
            grid: new_grid,
            x: self.x,
            y: self.y,
        };
    }
}

fn main() {
    let matches = App::new("Conway's game of life")
        .version("1.0")
        .arg(Arg::with_name("x")
            .help("horizontal length of the board")
            .required(true)
            .default_value("20")
            .index(1))
        .arg(Arg::with_name("y")
            .help("vertical length of the board")
            .default_value("30")
            .index(2))
        .get_matches();
    let x = value_t_or_exit!(matches, "x", usize);
    let y = value_t_or_exit!(matches, "y", usize);

    let state = GameState::new(x, y);
    let mut stdout = BufWriter::new(io::stdout());

    println!("Render of a base state for x={0}, y={1}: ", x, y);
    render(&state, &mut stdout);
    println!("\nNext iteration:");
    render(&(state.next()), &mut stdout);
}


fn render(state: &GameState, writer: &mut dyn Write) {
    (0..state.y).for_each(|y| {
        (0..state.x).for_each(|x| {
            writer.write(if state.grid[x][y] { X } else { DASH }).unwrap();
            if x < (state.x - 1) {
                writer.write(EMPTY_STR).unwrap();
            }
        });
        writer.write(CAR_RET).unwrap();
    });
}
