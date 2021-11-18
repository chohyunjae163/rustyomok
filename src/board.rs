#[derive(Copy, Clone, PartialEq)]
pub enum CellState {
    Empty,
    OccupiedBlack,
    OccupiedWhite,
}

#[derive(PartialEq)]
pub enum StoneColor {
    White,
    Black,
}

pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

pub enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}