extern crate clap;

use clap::{Arg, App, value_t_or_exit};
use std::io::{BufWriter, Write};
use std::io;
use game_of_life::State;


fn main() {
    let matches = App::new("Conway's game of life")
        .version("1.0")
        .arg(Arg::with_name("w")
            .help("horizontal length of the board")
            .required(true)
            .default_value("20")
            .index(1))
        .arg(Arg::with_name("h")
            .help("vertical length of the board")
            .default_value("30")
            .index(2))
        .get_matches();
    let width = value_t_or_exit!(matches, "w", usize);
    let height = value_t_or_exit!(matches, "h", usize);

    let mut state = State::new(width, height);
    let mut stdout = BufWriter::new(io::stdout());

    println!("Render of a base state for width={0}, height={1}: ", width, height);
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
    writer.write(state.render().as_bytes()).unwrap();
    writer.flush().unwrap();
}
