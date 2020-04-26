extern crate clap;

use clap::{Arg, App, value_t_or_exit};
use std::io::{BufWriter, Write};
use std::io;

mod game;
use game::State;

const EMPTY_STR: &[u8] = " ".as_bytes();

const CAR_RET: &[u8] = "\n".as_bytes();
const X: &[u8] = "x".as_bytes();
const DASH: &[u8] = "-".as_bytes();


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

    let mut state = State::new(x, y);
    let mut stdout = BufWriter::new(io::stdout());

    println!("Render of a base state for x={0}, y={1}: ", x, y);
    render(&state, &mut stdout);

    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.contains("end") {
                    println!("Ciao!");
                    break;
                } else {
                    state.next();
                    render(&state, &mut stdout);
                }
            }
            Err(_) => println!("Error")
        };
    }
}


fn render(state: &State, writer: &mut dyn Write) {
    writer.write(format!("\nGeneration {}:\n", state.generation).as_bytes()).unwrap();
    (0..state.y).for_each(|y| {
        (0..state.x).for_each(|x| {
            writer.write(if state.grid[x][y] { X } else { DASH }).unwrap();
            if x < (state.x - 1) {
                writer.write(EMPTY_STR).unwrap();
            }
        });
        writer.write(CAR_RET).unwrap();
    });
    writer.flush();
}
