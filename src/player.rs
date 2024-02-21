use std::fmt;

use crate::coord::Coord;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn opponent(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    pub fn advancing_direction(&self) -> Coord {
        match self {
            Player::White => Coord::U,
            Player::Black => Coord::D,
        }
    }

    pub fn home_row(&self) -> i32 {
        match self {
            Player::White => 0,
            Player::Black => 7,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::White
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::Black => write!(f, "Black"),
            Player::White => write!(f, "White"),
        }
    }
}
