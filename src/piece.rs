use crate::player::Player;
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
