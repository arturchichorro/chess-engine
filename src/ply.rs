use crate::{coord::Coord, piece::Kind};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ply {
    pub origin: Coord,
    pub destination: Coord,
    pub promotion: Option<Kind>,
}
