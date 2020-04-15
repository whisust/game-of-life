extern crate clap;

use clap::{Arg, App, value_t_or_exit};
use std::io::{BufWriter, Write};
use std::io;

fn new_state(x: usize, y: usize) -> Vec<Vec<bool>> {
    let mut xs = Vec::<Vec<bool>>::with_capacity(x);
    // let mut is_even = true;
    for i in 0..x {
        let v = i == 2;
        xs.push((0..y).map(|_| v).collect());
        // is_even = !is_even;
    }
    return xs;
}

const EMPTY_STR : &[u8] = " ".as_bytes();
const CAR_RET : &[u8] = "\n".as_bytes();
const X : &[u8] = "x".as_bytes();
const DASH : &[u8] = "-".as_bytes();

fn render(state: &Vec<Vec<bool>>, max_x: usize, max_y: usize, writer: &mut dyn Write) {
    (0..max_y).for_each(|y| {
        (0..max_x).for_each(|x| {
            writer.write(if state[x][y] { X } else { DASH });
            if x < (max_x - 1) {
                writer.write(EMPTY_STR);
            }
        });
        writer.write(CAR_RET);
    });
}

fn main() {
    let matches = App::new("Conway's game of life")
        .version("1.0")
        .arg(Arg::with_name("x")
            .help("horizontal length of the board")
            .required(true)
            .default_value("30")
            .index(1))
        .arg(Arg::with_name("y")
            .help("vertical length of the board")
            .default_value("30")
            .index(2))
        .get_matches();
    let x = value_t_or_exit!(matches, "x", usize);
    let y = value_t_or_exit!(matches, "y", usize);

    let state = new_state(x, y);

    println!("Render of a base state for x={0}, y={1}: ", x, y);
    let mut stdout = BufWriter::new(io::stdout());

    render(&state, x, y, &mut stdout);
}
