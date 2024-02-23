use crate::coord::Coord;
use crate::player::Player;
use std::fmt::{self, Write};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PieceLoc {
    pub kind: Kind,
    pub loc: Coord,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Piece {
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
