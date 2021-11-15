use crate::board::{CellState, Coordinate, StoneColor};

pub struct GameEngine {
    board: [[CellState; 15]; 15],
    current_turn: StoneColor,
}

impl GameEngine {
    pub fn new() -> GameEngine {
        let engine = GameEngine {
            board: [[CellState::Empty; 15]; 15],
            current_turn: StoneColor::Black,
        };
        engine
    }

    pub fn place_stone(&mut self, stone_color: StoneColor, coord: Coordinate) {
        if self.board[coord.x][coord.y] != CellState::Empty {
            return;
        }

        if self.current_turn != stone_color {
            return;
        }

        match stone_color {
            StoneColor::White => self.board[coord.x][coord.y] = CellState::OccupiedWhite,
            StoneColor::Black => self.board[coord.x][coord.y] = CellState::OccupiedBlack,
        }

        match self.current_turn {
            StoneColor::White => self.current_turn = StoneColor::Black,
            StoneColor::Black => self.current_turn = StoneColor::White,
        }
    }
}
