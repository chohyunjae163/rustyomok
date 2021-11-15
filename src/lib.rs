#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use game::GameEngine;

lazy_static! {
    static ref GAME_ENGINE: Mutex<GameEngine> = Mutex::new(GameEngine::new());
}

#[no_mangle]
pub extern "C" fn place_stone() {
    let engine = GAME_ENGINE.lock().unwrap();
}

mod board;
mod game;
