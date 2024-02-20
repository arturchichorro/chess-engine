use crate::{coord::Coord, player::Player};
use std::fmt::{self, Write};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Piece {
    pub piece: Type,
    pub player: Player,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.piece.character(self.player))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Type {
    pub const DIR_ROOK: [Coord; 4] = [Coord::U, Coord::D, Coord::R, Coord::L];
    pub const DIR_BISHOP: [Coord; 4] = [Coord::UR, Coord::UL, Coord::DR, Coord::DL];
    pub const DIR_QUEEN_KING: [Coord; 8] = [
        Coord::U,
        Coord::D,
        Coord::R,
        Coord::L,
        Coord::UR,
        Coord::UL,
        Coord::DR,
        Coord::DL,
    ];
    pub const DIR_KNIGHT: [Coord; 8] = [
        Coord::URR,
        Coord::UUR,
        Coord::ULL,
        Coord::UUL,
        Coord::DRR,
        Coord::DDR,
        Coord::DLL,
        Coord::DDL,
    ];
    pub const PROMOTIONS: [Type; 4] = [Type::Queen, Type::Rook, Type::Bishop, Type::Knight];

    fn character(&self, player: Player) -> char {
        match player {
            Player::White => match self {
                Type::Pawn => '♙',
                Type::Rook => '♖',
                Type::Knight => '♘',
                Type::Bishop => '♗',
                Type::Queen => '♕',
                Type::King => '♔',
            },
            Player::Black => match self {
                Type::Pawn => '♟',
                Type::Rook => '♜',
                Type::Knight => '♞',
                Type::Bishop => '♝',
                Type::Queen => '♛',
                Type::King => '♚',
            },
        }
    }
}
