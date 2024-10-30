use connect4_board_library::Bitboard;
use wasm_bindgen::prelude::*;

// JS functions that rust can call
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

// Rust function that JS can call
// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

#[wasm_bindgen]
pub struct State {
    bitboard: Bitboard,
}

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let bitboard = Bitboard::new();
        return Self { bitboard };
    }

    pub fn place(&mut self, column: usize) {
        self.bitboard.drop_piece(column);

        if self.bitboard.check_win() {
            alert(&format!(
                "Player {} won",
                (self.bitboard.move_counter & 1) + 1
            ));
        }
    }
}
