//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

extern crate game_of_life;

use game_of_life::State;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn input_spaceship() -> State {
    let mut state = State::new(6, 6);
    state.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    state
}

#[cfg(test)]
pub fn expected_spaceship() -> State {
    let mut state = State::new(6, 6);
    state.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    state
}

#[wasm_bindgen_test]
pub fn test_next() {
    let mut state = input_spaceship();
    let expected_state = expected_spaceship();

    state.next();
    assert_eq!(&state.get_cells(), &expected_state.get_cells());
}