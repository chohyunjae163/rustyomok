
use board::{Board};

pub extern "C" fn place_stone() {
    let board = Board::new();
}

mod board;