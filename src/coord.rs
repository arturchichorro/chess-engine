#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}

impl Coord {
    // Cardinal directions
    pub const R: Self = Self { row: 0, col: 1 };
    pub const D: Self = Self { row: -1, col: 0 };
    pub const U: Self = Self { row: 1, col: 0 };
    pub const L: Self = Self { row: 0, col: -1 };

    // Diagonal directions
    pub const UR: Self = Self { row: 1, col: 1 };
    pub const UL: Self = Self { row: 1, col: -1 };
    pub const DR: Self = Self { row: -1, col: 1 };
    pub const DL: Self = Self { row: -1, col: -1 };

    // Knight directions
    pub const UUR: Self = Self { row: 2, col: 1 };
    pub const URR: Self = Self { row: 1, col: 2 };
    pub const UUL: Self = Self { row: 2, col: -1 };
    pub const ULL: Self = Self { row: 1, col: -2 };
    pub const DDR: Self = Self { row: -2, col: 1 };
    pub const DRR: Self = Self { row: -1, col: 2 };
    pub const DDL: Self = Self { row: -2, col: -1 };
    pub const DLL: Self = Self { row: -1, col: -2 };

    pub fn is_valid(&self) -> bool {
        self.col <= 7 && self.row <= 7 && self.col >= 0 && self.row >= 0
    }
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coord {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl std::ops::Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Coord {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl std::ops::Mul<i32> for Coord {
    type Output = Self;

    fn mul(self, factor: i32) -> Self {
        Coord {
            row: self.row * factor,
            col: self.col * factor,
        }
    }
}

impl std::ops::Mul<Coord> for i32 {
    type Output = Coord;

    fn mul(self, coord: Coord) -> Coord {
        Coord {
            row: self * coord.row,
            col: self * coord.col,
        }
    }
}
