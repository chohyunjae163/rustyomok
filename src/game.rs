use crate::board::{Coordinate, StoneColor, DIRECTIONS};
use std::convert::TryFrom;
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
        let row = coord.x as usize;
        let column = coord.y as usize;
        if self.board[row][column] != StoneColor::None {
            return false;
        }

        if self.current_turn != stone_color {
            return false;
        }

        match stone_color {
            StoneColor::White => self.board[row][column] = StoneColor::White,
            StoneColor::Black => self.board[row][column] = StoneColor::Black,
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
                //FIXME: there is a bug!!!
                let x_dir = dir.0;
                let y_dir = dir.1;
                let x = coord.x + x_dir * i;
                let y = coord.y + y_dir * i;
                let row = x as usize;
                let col = y as usize;
                if self.board[row][col] == StoneColor::Black {
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

#[cfg(test)]
mod tests {
    use super::super::board::Coordinate;
    use super::super::board::StoneColor;
    use super::GameEngine;
    #[test]
    fn test_board_Empty() {
        let engine = GameEngine::new();
        for row in engine.board.iter() {
            for col in row.iter() {
                assert_eq!(*col, StoneColor::None);
            }
        }
    }
    #[test]
    fn test_place_stone() {
        let mut engine = GameEngine::new();
        engine.place_stone(StoneColor::Black, Coordinate { x: 8, y: 5 });
        assert_eq!(engine.board[8][5], StoneColor::Black);
    }

    #[test]
    fn test_gameend_check() {
        let mut engine = GameEngine::new();
        engine.place_stone(StoneColor::Black, Coordinate { x: 4, y: 5 });
        engine.place_stone(StoneColor::Black, Coordinate { x: 5, y: 5 });
        engine.place_stone(StoneColor::Black, Coordinate { x: 6, y: 5 });
        engine.place_stone(StoneColor::Black, Coordinate { x: 7, y: 5 });
        engine.place_stone(StoneColor::Black, Coordinate { x: 8, y: 5 });
        let game_end = engine.check_gameend(Coordinate { x: 8, y: 5 });
        assert_eq!(game_end, true);
    }
}
