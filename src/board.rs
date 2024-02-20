use crate::{
    coord::Coord,
    piece::{Piece, Type},
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
                        piece: match p.kind {
                            fen::PieceKind::Pawn => Type::Pawn,
                            fen::PieceKind::Knight => Type::Knight,
                            fen::PieceKind::Bishop => Type::Bishop,
                            fen::PieceKind::Rook => Type::Rook,
                            fen::PieceKind::Queen => Type::Queen,
                            fen::PieceKind::King => Type::King,
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
    fn square_is_occupied(&self, location: Coord) -> bool {
        self.square_get(location).is_some()
    }
    fn player_at_square(&self, location: Coord) -> Option<Player> {
        self.square_get(location).map(|piece| piece.player)
    }

    fn get_pseudo_legal_moves(&self, coord: Coord) -> Vec<Ply> {
        if let Some(piece_in_square) = self.square_get(coord) {
            match piece_in_square.piece {
                Type::Pawn => {
                    vec![self.get_pawn_moves(coord), self.get_pawn_captures(coord)].concat()
                }
                Type::Knight => self.get_king_knight_moves(coord, &Coord::LIST_KNIGHT),
                Type::Bishop => self.get_queen_rook_bishop_moves(coord, &Coord::LIST_DIAGONAL),
                Type::Rook => self.get_queen_rook_bishop_moves(coord, &Coord::LIST_CARDINAL),
                Type::Queen => {
                    self.get_queen_rook_bishop_moves(coord, &Coord::LIST_CARDINAL_DIAGONAL)
                }
                Type::King => self.get_king_knight_moves(coord, &Coord::LIST_CARDINAL_DIAGONAL),
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

        directions
            .iter()
            .map(|dir| {
                let mut plys = vec![];

                for i in 1.. {
                    let pos = origin + (*dir) * i;

                    if !pos.is_valid() {
                        break;
                    }

                    if self.player_at_square(pos) == Some(player) {
                        break;
                    }

                    plys.push(Ply {
                        origin,
                        destination: pos,
                        promotion: None,
                    });

                    if self.player_at_square(pos) == Some(player.opponent()) {
                        break;
                    }
                }

                plys.into_iter()
            })
            .flatten()
            .collect()
    }

    // -----------------

    fn get_pawn_moves(&self, origin: Coord) -> Vec<Ply> {
        let player = self.player_at_square(origin).unwrap();

        let dir = player.advancing_direction();

        let mut results: Vec<Ply> = vec![];

        if !self.square_is_occupied(origin + dir) {
            if origin.row == 0 || origin.row == 7 {
                for promo in Type::PROMOTIONS {
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
            if !self.square_is_occupied(origin + 2 * dir) {
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
        let left_capture = origin + dir + Coord::L;
        let right_capture = origin + dir + Coord::R;
        let mut results: Vec<Ply> = vec![];

        // Capture to the left
        if left_capture.is_valid() && self.player_at_square(left_capture) == Some(player.opponent())
        {
            if left_capture.row == 7 || left_capture.row == 0 {
                for promo in Type::PROMOTIONS {
                    results.push(Ply {
                        origin,
                        destination: left_capture,
                        promotion: Some(promo),
                    })
                }
            } else {
                results.push(Ply {
                    origin,
                    destination: left_capture,
                    promotion: None,
                })
            }
        }
        // Capture to the right
        if left_capture.is_valid() && self.player_at_square(left_capture) == Some(player.opponent())
        {
            if right_capture.row == 7 || right_capture.row == 0 {
                for promo in Type::PROMOTIONS {
                    results.push(Ply {
                        origin,
                        destination: right_capture,
                        promotion: Some(promo),
                    })
                }
            } else {
                results.push(Ply {
                    origin,
                    destination: right_capture,
                    promotion: None,
                })
            }
        }

        // En passant
        if self.en_passant_square.is_some() {
            let ep_square = self.en_passant_square.unwrap();
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

    fn get_castling_moves(&self, origin: Coord) -> Vec<Ply> {
        todo!()
    }

    pub fn arbiter(&self, ply: &Ply) -> bool {
        let legal_moves = self.get_pseudo_legal_moves(ply.origin);

        legal_moves.contains(ply)
    }

    pub fn make_move(&self, ply: Ply) -> Board {
        let mut new_game_state = self.clone();

        let player = self.player_at_square(ply.origin).unwrap();
        let piece = self.square_get(ply.origin).unwrap().piece;

        let dir = player.advancing_direction();

        // Detecting if move was en passant
        if piece == Type::Pawn
            && ply.origin.col != ply.destination.col
            && self.square_get(ply.destination).is_none()
        {
            new_game_state.square_set(ply.destination - dir, None);
        }

        // Updating en_passant_square
        new_game_state.en_passant_square =
            if piece == Type::Pawn && ply.destination.row == (ply.origin + 2 * dir).row {
                Some(ply.origin + dir)
            } else {
                None
            };

        // Promotion
        if let Some(new_piece) = ply.promotion {
            // TODO
            // new_game_state.coord_set(ply.origin).as_mut().unwrap().piece = new_piece;
        }

        new_game_state.square_set(ply.destination, *new_game_state.square_get(ply.origin));
        new_game_state.square_set(ply.origin, None);

        new_game_state.turn = new_game_state.turn.opponent();
        new_game_state
    }

    fn verify_status(&self) -> String {
        // Verifica se o jogo acaba em checkmate ou não
        todo!()
    }
}
