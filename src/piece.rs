use crate::coord::Coord;
use crate::player::Player;
use std::fmt::{self, Write};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Piece {
    pub kind: Kind,
    pub player: Player,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PieceCopy {
    pub coord: Coord,
    pub kind: Kind,
    pub player: Player,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.kind.character(self.player))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Kind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Kind {
    pub const PROMOTIONS: [Kind; 4] = [Kind::Queen, Kind::Rook, Kind::Bishop, Kind::Knight];

    pub const PAWN_VALUE: i32 = 100;
    pub const KNIGHT_VALUE: i32 = 300;
    pub const BISHOP_VALUE: i32 = 300;
    pub const ROOK_VALUE: i32 = 500;
    pub const QUEEN_VALUE: i32 = 900;

    fn character(&self, player: Player) -> char {
        match player {
            Player::White => match self {
                Kind::Pawn => '♙',
                Kind::Rook => '♖',
                Kind::Knight => '♘',
                Kind::Bishop => '♗',
                Kind::Queen => '♕',
                Kind::King => '♔',
            },
            Player::Black => match self {
                Kind::Pawn => '♟',
                Kind::Rook => '♜',
                Kind::Knight => '♞',
                Kind::Bishop => '♝',
                Kind::Queen => '♛',
                Kind::King => '♚',
            },
        }
    }
}
