#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
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

    // Lists
    pub const LIST_CARDINAL: [Self; 4] = [Self::R, Self::D, Self::U, Self::L];
    pub const LIST_DIAGONAL: [Self; 4] = [Self::UR, Self::UL, Self::DR, Self::DL];
    pub const LIST_CARDINAL_DIAGONAL: [Self; 8] = [
        Self::R,
        Self::D,
        Self::U,
        Self::L,
        Self::UR,
        Self::UL,
        Self::DR,
        Self::DL,
    ];
    pub const LIST_KNIGHT: [Self; 8] = [
        Self::UUR,
        Self::URR,
        Self::UUL,
        Self::ULL,
        Self::DDR,
        Self::DRR,
        Self::DDL,
        Self::DLL,
    ];

    pub fn is_valid(&self) -> bool {
        self.col <= 7 && self.row <= 7 && self.col >= 0 && self.row >= 0
    }

    pub fn find_dir_between_coords(pos_one: Coord, pos_two: Coord) -> Coord {
        let row_delta = pos_one.row - pos_two.row;
        let col_delta = pos_one.col - pos_two.col;
        match (row_delta, col_delta) {
            (0, _) => {
                if col_delta > 0 {
                    Coord::R
                } else {
                    Coord::L
                }
            }
            (_, 0) => {
                if row_delta > 0 {
                    Coord::U
                } else {
                    Coord::D
                }
            }
            (_, _) => {
                if row_delta > 0 {
                    if col_delta > 0 {
                        Coord::UR
                    } else {
                        Coord::UL
                    }
                } else {
                    if col_delta > 0 {
                        Coord::DR
                    } else {
                        Coord::DL
                    }
                }
            }
        }
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
