use crate::{
    coord::Coord,
    piece::{Kind, Piece},
    player::Player,
    ply::Ply,
};

#[derive(Default, Debug, Copy, Clone)]
pub struct Board {
    pub turn: Player,

    board: [[Option<Piece>; 8]; 8],

    white_can_oo: bool,
    white_can_ooo: bool,
    black_can_oo: bool,
    black_can_ooo: bool,
    half_move_clock: u64, // Counts the number of moves in a row without pawn moves or capture
    en_passant_square: Option<Coord>,
}

impl Board {
    pub fn notation_conversion(v1: char, v2: i32) -> Option<Coord> {
        if (v1 as i32) >= ('a' as i32) && (v1 as i32) <= ('h' as i32) && v2 >= 1 && v2 <= 8 {
            Some(Coord {
                row: v2 - 1 as i32,
                col: (v1 as i32) - ('a' as i32),
            })
        } else {
            None
        }
    }

    pub fn new_from_fen(fen: &str) -> Board {
        let mut board: Board = Default::default();

        let board_from_fen = fen::BoardState::from_fen(fen).unwrap();

        board.turn = match board_from_fen.side_to_play {
            fen::Color::White => Player::White,
            fen::Color::Black => Player::Black,
        };

        board.white_can_oo = board_from_fen.white_can_oo;
        board.white_can_ooo = board_from_fen.white_can_ooo;
        board.black_can_oo = board_from_fen.black_can_oo;
        board.black_can_ooo = board_from_fen.black_can_ooo;
        board.half_move_clock = board_from_fen.halfmove_clock;

        board.en_passant_square = board_from_fen.en_passant_square.map(|x| Coord {
            row: (x / 8) as i32,
            col: (x % 8) as i32,
        });

        board_from_fen
            .pieces
            .into_iter()
            .enumerate()
            .map(|(i, p)| p.map(|x| (i, x)))
            .flatten()
            .for_each(|(i, p)| {
                board.square_set(
                    Coord {
                        row: (i / 8) as i32,
                        col: (i % 8) as i32,
                    },
                    Some(Piece {
                        player: match p.color {
                            fen::Color::White => Player::White,
                            fen::Color::Black => Player::Black,
                        },
                        kind: match p.kind {
                            fen::PieceKind::Pawn => Kind::Pawn,
                            fen::PieceKind::Knight => Kind::Knight,
                            fen::PieceKind::Bishop => Kind::Bishop,
                            fen::PieceKind::Rook => Kind::Rook,
                            fen::PieceKind::Queen => Kind::Queen,
                            fen::PieceKind::King => Kind::King,
                        },
                    }),
                );
            });

        board
    }

    pub fn print_board(&self, player_pov: Player) {
        for row_idx in 0..8 {
            let row = match player_pov {
                Player::White => self.board[7 - row_idx],
                Player::Black => self.board[row_idx],
            };

            for square in row {
                match square {
                    Some(piece) => print!(" {} ", piece),
                    None => print!(" . "),
                }
            }
            println!();
        }
    }

    fn square_get(&self, coord: Coord) -> &Option<Piece> {
        &self.board[coord.row as usize][coord.col as usize]
    }
    fn square_set(&mut self, coord: Coord, piece: Option<Piece>) {
        self.board[coord.row as usize][coord.col as usize] = piece;
    }
    fn is_square_occupied(&self, location: Coord) -> bool {
        self.square_get(location).is_some()
    }
    fn player_at_square(&self, location: Coord) -> Option<Player> {
        self.square_get(location).map(|piece| piece.player)
    }
    fn kind_at_square(&self, location: Coord) -> Option<Kind> {
        self.square_get(location).map(|piece| piece.kind)
    }

    fn get_pseudo_legal_moves(&self, coord: Coord) -> Vec<Ply> {
        if let Some(piece_in_square) = self.square_get(coord) {
            match piece_in_square.kind {
                Kind::Pawn => {
                    vec![self.get_pawn_moves(coord), self.get_pawn_captures(coord)].concat()
                }
                Kind::Knight => self.get_king_knight_moves(coord, &Coord::LIST_KNIGHT),
                Kind::Bishop => self.get_queen_rook_bishop_moves(coord, &Coord::LIST_DIAGONAL),
                Kind::Rook => self.get_queen_rook_bishop_moves(coord, &Coord::LIST_CARDINAL),
                Kind::Queen => {
                    self.get_queen_rook_bishop_moves(coord, &Coord::LIST_CARDINAL_DIAGONAL)
                }
                Kind::King => self.get_king_knight_moves(coord, &Coord::LIST_CARDINAL_DIAGONAL),
            }
        } else {
            vec![]
        }
    }

    fn get_king_knight_moves(&self, origin: Coord, directions: &[Coord]) -> Vec<Ply> {
        let player = self.player_at_square(origin).unwrap();

        directions
            .iter()
            .map(|&delta| origin + delta)
            .filter(|&pos| pos.is_valid() && self.player_at_square(pos) != Some(player))
            .map(|pos| Ply {
                origin,
                destination: pos,
                promotion: None,
            })
            .collect()
    }

    fn get_queen_rook_bishop_moves(&self, origin: Coord, directions: &[Coord]) -> Vec<Ply> {
        let player = self.player_at_square(origin).unwrap();

        // directions
        //     .iter()
        //     .map(|dir| {
        //         let mut plys = vec![];

        //         for i in 1.. {
        //             let pos = origin + (*dir) * i;

        //             if !pos.is_valid() {
        //                 break;
        //             }

        //             if self.player_at_square(pos) == Some(player) {
        //                 break;
        //             }

        //             plys.push(Ply {
        //                 origin,
        //                 destination: pos,
        //                 promotion: None,
        //             });

        //             if self.player_at_square(pos) == Some(player.opponent()) {
        //                 break;
        //             }
        //         }

        //         plys.into_iter()
        //     })
        //     .flatten()
        //     .collect()

        directions
            .iter()
            .map(|&dir| {
                (1..)
                    .map(move |i| origin + dir * i)
                    .take_while(|&c| c.is_valid())
                    .take_while(|&c| self.player_at_square(c) != Some(player))
                    .take_while(move |&c| self.player_at_square(c - dir) != Some(player.opponent()))
                    .map(|c| Ply {
                        origin,
                        destination: c,
                        promotion: None,
                    })
            })
            .flatten()
            .collect()
    }

    pub fn make_move(&self, ply: Ply) -> Board {
        let mut new_game_state = self.clone();

        let piece = self.square_get(ply.origin).unwrap();
        let dir = piece.player.advancing_direction();

        // Detect if move was en passant abd remove the captured pawn
        if piece.kind == Kind::Pawn
            && ply.origin.col != ply.destination.col
            && self.square_get(ply.destination).is_none()
        {
            new_game_state.square_set(ply.destination - dir, None);
        }

        // Update en_passant_square
        new_game_state.en_passant_square =
            if piece.kind == Kind::Pawn && ply.destination == ply.origin + 2 * dir {
                Some(ply.origin + dir)
            } else {
                None
            };

        // Set piece (including promotions) on new square
        new_game_state.square_set(
            ply.destination,
            Some(Piece {
                kind: ply.promotion.unwrap_or(piece.kind),
                player: piece.player,
            }),
        );

        // Remove piece from old square
        new_game_state.square_set(ply.origin, None);

        println!(
            "{:?}",
            self.is_square_attacked(ply.destination, Player::White)
        );
        // Change turn and return new game state
        // new_game_state.turn = new_game_state.turn.opponent();
        new_game_state
    }

    // -----------------

    fn get_pawn_moves(&self, origin: Coord) -> Vec<Ply> {
        let player = self.player_at_square(origin).unwrap();

        let dir = player.advancing_direction();

        let mut results: Vec<Ply> = vec![];

        if !self.is_square_occupied(origin + dir) {
            if origin.row == 0 || origin.row == 7 {
                for promo in Kind::PROMOTIONS {
                    results.push(Ply {
                        origin,
                        destination: origin + dir,
                        promotion: Some(promo),
                    })
                }
            } else {
                results.push(Ply {
                    origin,
                    destination: origin + dir,
                    promotion: None,
                })
            }
        } else {
            return results;
        }

        if origin.row % 5 == 1 {
            if !self.is_square_occupied(origin + 2 * dir) {
                results.push(Ply {
                    origin,
                    destination: origin + 2 * dir,
                    promotion: None,
                })
            }
        }

        results
    }

    fn get_pawn_captures(&self, origin: Coord) -> Vec<Ply> {
        let player = self.player_at_square(origin).unwrap();
        let dir = player.advancing_direction();

        let mut results: Vec<Ply> = [Coord::L, Coord::R]
            .iter()
            .map(|&delta| origin + dir + delta)
            .filter(|&pos| pos.is_valid() && self.player_at_square(pos) == Some(player.opponent()))
            .map(|pos| {
                if pos.row == 7 || pos.row == 0 {
                    Box::new(Kind::PROMOTIONS.iter().map(move |&promo| Ply {
                        origin,
                        destination: pos,
                        promotion: Some(promo),
                    })) as Box<dyn Iterator<Item = Ply>>
                } else {
                    Box::new(std::iter::once(Ply {
                        origin,
                        destination: pos,
                        promotion: None,
                    })) as Box<dyn Iterator<Item = Ply>>
                }
            })
            .flatten()
            .collect();

        // En passant
        if self.en_passant_square.is_some() {
            let ep_square = self.en_passant_square.unwrap(); // TODO
            if (origin + dir).row == ep_square.row {
                if (origin.col + 1 == ep_square.col) || (origin.col - 1 == ep_square.col) {
                    results.push(Ply {
                        origin,
                        destination: ep_square,
                        promotion: None,
                    })
                }
            }
        }

        results
    }

    fn get_all_moves(&self) -> Vec<Ply> {
        let mut moves: Vec<Ply> = vec![];

        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.square_get(Coord { row, col }) {
                    if self.turn == piece.player {
                        moves.extend(self.get_pseudo_legal_moves(Coord { row, col }).iter())
                    }
                }
            }
        }

        todo!()
    }

    fn is_square_attacked(&self, origin: Coord, player: Player) -> bool {
        // Verify diagonals
        let diagonal_attack = Coord::LIST_DIAGONAL.iter().any(|&dir| {
            (1..)
                .map(move |i| origin + dir * i)
                .take_while(|&c| c.is_valid())
                .take_while(|&c| self.player_at_square(c) != Some(player))
                .take_while(move |&c| self.player_at_square(c - dir) != Some(player.opponent()))
                .any(|c| {
                    self.player_at_square(c - dir) == Some(player.opponent())
                        && (self.kind_at_square(c - dir) == Some(Kind::Bishop)
                            || self.kind_at_square(c - dir) == Some(Kind::Queen))
                })
        });

        // Verify horizontal and vertical lines
        let cardinal_attack = Coord::LIST_CARDINAL.iter().any(|&dir| {
            (1..)
                .map(move |i| origin + dir * i)
                .take_while(|&c| c.is_valid())
                .take_while(|&c| self.player_at_square(c) != Some(player))
                .take_while(move |&c| self.player_at_square(c - dir) != Some(player.opponent()))
                .any(|c| {
                    self.player_at_square(c - dir) == Some(player.opponent())
                        && (self.kind_at_square(c - dir) == Some(Kind::Rook)
                            || self.kind_at_square(c - dir) == Some(Kind::Queen))
                })
        });

        // Verify pawns
        let pawn_attack = [Coord::L, Coord::R]
            .iter()
            .map(|c| origin + *c - player.opponent().advancing_direction())
            .any(|pos| {
                self.player_at_square(pos) == Some(player.opponent())
                    && self.kind_at_square(pos) == Some(Kind::Pawn)
            });

        // Verify Knights
        let knight_attack = Coord::LIST_KNIGHT.iter().any(|x| {
            let pos = origin + *x;
            self.player_at_square(pos) == Some(player.opponent())
                && self.kind_at_square(pos) == Some(Kind::Knight)
        });

        dbg!(diagonal_attack || knight_attack || cardinal_attack || pawn_attack)
    }

    fn get_castling_moves(&self) -> Vec<Ply> {
        let player = self.turn;
        let mut results: Vec<Ply> = vec![];

        // Como reduzir estes dois bocados de código quase idênticos?
        match player {
            Player::White => {
                if self.white_can_oo
                    && !self.is_square_attacked(Coord { row: 0, col: 5 }, player)
                    && !self.is_square_attacked(Coord { row: 0, col: 6 }, player)
                    && !self.is_square_occupied(Coord { row: 0, col: 5 })
                    && !self.is_square_occupied(Coord { row: 0, col: 6 })
                {
                    results.push(Ply {
                        origin: Coord { row: 0, col: 4 },
                        destination: Coord { row: 0, col: 6 },
                        promotion: None,
                    })
                }
                if self.white_can_ooo
                    && !self.is_square_attacked(Coord { row: 0, col: 2 }, player)
                    && !self.is_square_attacked(Coord { row: 0, col: 3 }, player)
                    && !self.is_square_occupied(Coord { row: 0, col: 1 })
                    && !self.is_square_occupied(Coord { row: 0, col: 2 })
                    && !self.is_square_occupied(Coord { row: 0, col: 3 })
                {
                    results.push(Ply {
                        origin: Coord { row: 0, col: 4 },
                        destination: Coord { row: 0, col: 2 },
                        promotion: None,
                    })
                }
            }
            Player::Black => {
                if self.black_can_oo
                    && !self.is_square_attacked(Coord { row: 7, col: 5 }, player)
                    && !self.is_square_attacked(Coord { row: 7, col: 6 }, player)
                    && !self.is_square_occupied(Coord { row: 7, col: 5 })
                    && !self.is_square_occupied(Coord { row: 7, col: 6 })
                {
                    results.push(Ply {
                        origin: Coord { row: 7, col: 4 },
                        destination: Coord { row: 7, col: 6 },
                        promotion: None,
                    })
                }
                if self.black_can_ooo
                    && !self.is_square_attacked(Coord { row: 7, col: 2 }, player)
                    && !self.is_square_attacked(Coord { row: 7, col: 3 }, player)
                    && !self.is_square_occupied(Coord { row: 7, col: 1 })
                    && !self.is_square_occupied(Coord { row: 7, col: 2 })
                    && !self.is_square_occupied(Coord { row: 7, col: 3 })
                {
                    results.push(Ply {
                        origin: Coord { row: 7, col: 4 },
                        destination: Coord { row: 7, col: 2 },
                        promotion: None,
                    })
                }
            }
        }

        results
    }

    pub fn arbiter(&self, ply: &Ply) -> bool {
        let legal_moves = self.get_pseudo_legal_moves(ply.origin);

        legal_moves.contains(ply)
    }
}
