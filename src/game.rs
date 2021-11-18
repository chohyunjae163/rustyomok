use crate::board::{CellState, Coordinate, Direction, StoneColor};

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

        //TODO:: need to know whether the game is over.
        self.check_gameend(coord);
    }

    fn check_gameend(&self, coord: Coordinate) {
        //TODO:check five stones in a row for every direction. NEEDS WORK HERE!!
        // for i in 0..5 {
        //     let xy = GameEngine::get_direction(Direction::TopLeft);
        //     let neighbor = coord.x + xy.0 as usize;
        // }
    }

    fn get_direction(direction: Direction) -> (i8, i8) {
        return match direction {
            Direction::TopLeft => (-1, -1),
            Direction::Top => (0, -1),
            Direction::TopRight => (1, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::BottomLeft => (-1, 1),
            Direction::Bottom => (0, 1),
            Direction::BottomRight => (1, 1),
        };
    }

    pub fn reset_board(&mut self) {
        for row in self.board.iter_mut() {
            for col in row.iter_mut() {
                *col = CellState::Empty;
            }
        }
    }
}
