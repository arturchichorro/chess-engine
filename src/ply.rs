use crate::{coord::Coord, piece::Type};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ply {
    pub origin: Coord,
    pub destination: Coord,
    pub promotion: Option<Type>,
}
