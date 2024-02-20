#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}

impl Coord {
    pub const R: Self = Coord::new(0, 1);
    pub const D: Self = Coord::new(-1, 0);
    pub const U: Self = Coord::new(1, 0);
    pub const L: Self = Coord::new(0, -1);
    pub const UR: Self = Coord::new(1, 1);
    pub const UL: Self = Coord::new(1, -1);
    pub const DR: Self = Coord::new(-1, 1);
    pub const DL: Self = Coord::new(-1, -1);

    // Knight directions
    pub const UUR: Self = Coord::new(2, 1);
    pub const URR: Self = Coord::new(1, 2);
    pub const UUL: Self = Coord::new(2, -1);
    pub const ULL: Self = Coord::new(1, -2);
    pub const DDR: Self = Coord::new(-2, 1);
    pub const DRR: Self = Coord::new(-1, 2);
    pub const DDL: Self = Coord::new(-2, -1);
    pub const DLL: Self = Coord::new(-1, -2);

    pub const fn new(col: i32, row: i32) -> Coord {
        Coord { row: col, col: row } // TODO: Fix this
    }

    pub fn is_valid_location(&self) -> bool {
        !(self.col > 7 || self.row > 7 || self.col < 0 || self.row < 0)
    }

    pub fn add(&self, delta: &Coord) -> Coord {
        Coord {
            row: self.row + delta.row,
            col: self.col + delta.col,
        }
    }

    pub fn mult(&self, factor: i32) -> Coord {
        Coord {
            row: factor * self.row,
            col: factor * self.col,
        }
    }
}
