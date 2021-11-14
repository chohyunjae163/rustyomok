#[derive(Copy, Clone)]
enum CellState {
    Empty,
    OccupiedBlack,
    OccupiedWhite,
}

pub enum PieceColor {
    White,
    Black,
}

pub struct Coordinate {
    x : usize,
    y : usize,
}

pub struct Board {
    cells : [[CellState;15];15]
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells : [[CellState::Empty;15];15]
        }
    }

    pub fn place_stone(&mut self, color : PieceColor, coord : Coordinate ) {
        match color {
            White => self.cells[coord.x][coord.y] = CellState::OccupiedWhite,
            Black => self.cells[coord.x][coord.y] = CellState::OccupiedBlack 
        }
    }
}
