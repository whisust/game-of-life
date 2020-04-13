extern crate clap;

use clap::{Arg, App, value_t};

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

fn render(state: &Vec<Vec<bool>>, x: usize, y: usize) {
    let mut _formatted_state: String = "".to_string();
    for _y in 0..y {
        for _x in 0..x {
            _formatted_state.push_str(if state[_x][_y] { "x" } else { "-" });
            if _x < (x-1) {
                _formatted_state.push_str(" ");
            }
        }
        _formatted_state.push_str("\n")
    }
    println!("{}", _formatted_state);
}

fn main() {
    let parser = App::new("Conway's game of life")
        .version("1.0")
        .arg(Arg::with_name("x")
            .help("horizontal length of the board")
            .required(true)
            .index(1))
        .arg(Arg::with_name("y")
            .help("vertical length of the board")
            .index(2))
        .get_matches();
    let x = value_t!(parser.value_of("x"), usize).unwrap_or(30);
    let y = value_t!(parser.value_of("y"), usize).unwrap_or(30);

    let state = new_state(x, y);

    render(&state, x, y);

    println!("Render of a base state for x={0}, y={1}", x, y);
}
