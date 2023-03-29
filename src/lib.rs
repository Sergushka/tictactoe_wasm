use wasm_bindgen::prelude::*;
mod game;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    alert("Hello, tictactoe-web!");
}
