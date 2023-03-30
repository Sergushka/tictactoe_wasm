use game::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
mod game;

#[wasm_bindgen]
extern "C" {}

thread_local! {
    static GAME: RefCell<Game>  = RefCell::new(Game::new());
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    GAME.with(|game| game.borrow().get_state())
}

#[wasm_bindgen(js_name = makeTurn)]
pub fn make_turn(i: usize) {
    GAME.with(|game| game.borrow_mut().make_turn(i));
}

#[wasm_bindgen(js_name = isGameOver)]
pub fn is_game_over() -> bool {
    GAME.with(|game| game.borrow().is_finished())
}

#[wasm_bindgen(js_name = getLastPlayed)]
pub fn get_winner() -> String {
    GAME.with(|game| game.borrow().get_last_opposite())
}

#[wasm_bindgen(js_name = restart)]
pub fn restart() {
    GAME.with(|game| game.borrow_mut().restart());
}
