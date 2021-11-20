use crate::board::{Coordinate, StoneColor, DIRECTIONS};

pub struct GameEngine {
    board: [[StoneColor; 15]; 15],
    current_turn: StoneColor,
}

impl GameEngine {
    pub fn new() -> GameEngine {
        let engine = GameEngine {
            board: [[StoneColor::None; 15]; 15],
            current_turn: StoneColor::Black,
        };
        engine
    }

    pub fn place_stone(&mut self, stone_color: StoneColor, coord: Coordinate) -> bool {
        if self.board[coord.x][coord.y] != StoneColor::None {
            return false;
        }

        if self.current_turn != stone_color {
            return false;
        }

        match stone_color {
            StoneColor::White => self.board[coord.x][coord.y] = StoneColor::White,
            StoneColor::Black => self.board[coord.x][coord.y] = StoneColor::Black,
            StoneColor::None => return false,
        }

        match self.current_turn {
            StoneColor::White => self.current_turn = StoneColor::Black,
            StoneColor::Black => self.current_turn = StoneColor::White,
            StoneColor::None => return false,
        }

        true
    }

    pub fn check_gameend(&self, coord: Coordinate) -> bool {
        for dir in DIRECTIONS {
            let mut inarow = 0;
            for i in 0..5 {
                //TODO 예외사항처리
                let x = coord.x + dir.0 as usize + i;
                let y = coord.y + dir.1 as usize + i;
                if self.board[x][y] == self.current_turn {
                    inarow += 1;
                }
            }
            if inarow == 5 {
                return true;
            }
        }

        false
    }

    pub fn reset_board(&mut self) {
        for row in self.board.iter_mut() {
            for col in row.iter_mut() {
                *col = StoneColor::None;
            }
        }
    }
}
