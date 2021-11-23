use std::fmt;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum StoneColor {
    None,
    White,
    Black,
}

pub struct Coordinate {
    pub x: i8,
    pub y: i8,
}

//(0,0) is center
pub static DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1), //TOP LEFT
    (0, -1),  // TOP
    (1, -1),  // TOP RIGHT
    (-1, 0),  // LEFT
    (1, 0),   // RIGHT
    (-1, 1),  // BOTTOM ELEFT
    (0, 1),   // BOTTOM
    (1, 1),   // BOTTOM RIGHT
];
