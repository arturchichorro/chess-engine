#![allow(dead_code)]

use std::fmt;
use std::fmt::Write;
use std::io;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Player {
    Black,
    White,
}

impl Player {
    fn opponent(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    fn advancing_direction(&self) -> i32 {
        match self {
            Player::Black => -1,
            Player::White => 1,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::White
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::Black => write!(f, "Black"),
            Player::White => write!(f, "White"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Piece {
    piece: PieceType,
    player: Player,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.piece.character(self.player))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
    const DIR_ROOK: [Coordinate; 4] = [Coordinate::U, Coordinate::D, Coordinate::R, Coordinate::L];
    const DIR_BISHOP: [Coordinate; 4] = [
        Coordinate::UR,
        Coordinate::UL,
        Coordinate::DR,
        Coordinate::DL,
    ];
    const DIR_QUEEN_KING: [Coordinate; 8] = [
        Coordinate::U,
        Coordinate::D,
        Coordinate::R,
        Coordinate::L,
        Coordinate::UR,
        Coordinate::UL,
        Coordinate::DR,
        Coordinate::DL,
    ];
    const DIR_KNIGHT: [Coordinate; 8] = [
        Coordinate::URR,
        Coordinate::UUR,
        Coordinate::ULL,
        Coordinate::UUL,
        Coordinate::DRR,
        Coordinate::DDR,
        Coordinate::DLL,
        Coordinate::DDL,
    ];

    const PROMOTIONS: [PieceType; 4] = [
        PieceType::Queen,
        PieceType::Rook,
        PieceType::Bishop,
        PieceType::Knight,
    ];

    fn character(&self, player: Player) -> char {
        match player {
            Player::White => match self {
                PieceType::Pawn => '♙',
                PieceType::Rook => '♖',
                PieceType::Knight => '♘',
                PieceType::Bishop => '♗',
                PieceType::Queen => '♕',
                PieceType::King => '♔',
            },
            Player::Black => match self {
                PieceType::Pawn => '♟',
                PieceType::Rook => '♜',
                PieceType::Knight => '♞',
                PieceType::Bishop => '♝',
                PieceType::Queen => '♛',
                PieceType::King => '♚',
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coordinate {
    row: i32,
    col: i32,
}

impl Coordinate {
    const R: Self = Coordinate::new(0, 1);
    const D: Self = Coordinate::new(-1, 0);
    const U: Self = Coordinate::new(1, 0);
    const L: Self = Coordinate::new(0, -1);
    const UR: Self = Coordinate::new(1, 1);
    const UL: Self = Coordinate::new(1, -1);
    const DR: Self = Coordinate::new(-1, 1);
    const DL: Self = Coordinate::new(-1, -1);

    // Knight directions
    const UUR: Self = Coordinate::new(2, 1);
    const URR: Self = Coordinate::new(1, 2);
    const UUL: Self = Coordinate::new(2, -1);
    const ULL: Self = Coordinate::new(1, -2);
    const DDR: Self = Coordinate::new(-2, 1);
    const DRR: Self = Coordinate::new(-1, 2);
    const DDL: Self = Coordinate::new(-2, -1);
    const DLL: Self = Coordinate::new(-1, -2);

    const fn new(col: i32, row: i32) -> Coordinate {
        Coordinate { row: col, col: row }
    }

    fn is_valid_location(&self) -> bool {
        !(self.col > 7 || self.row > 7 || self.col < 0 || self.row < 0)
    }

    fn add(&self, delta: &Coordinate) -> Coordinate {
        Coordinate {
            row: self.row + delta.row,
            col: self.col + delta.col,
        }
    }

    fn mult(&self, factor: i32) -> Coordinate {
        Coordinate {
            row: factor * self.row,
            col: factor * self.col,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Ply {
    origin: Coordinate,
    destination: Coordinate,
    promotion: Option<PieceType>,
}

#[derive(Default, Debug, Copy, Clone)]
struct Board {
    board: [[Option<Piece>; 8]; 8],
    turn: Player,
    white_can_oo: bool,
    white_can_ooo: bool,
    black_can_oo: bool,
    black_can_ooo: bool,
    half_move_clock: u64, // Counts the number of moves in a row without pawn moves or capture
    en_passant_square: Option<Coordinate>,
}

impl Board {
    fn new_from_fen(fen: &str) -> Board {
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

        board.en_passant_square = board_from_fen.en_passant_square.map(|x| {
            let row_en_passant = (x % 8) as i32;
            let col_en_passant = (x / 8) as i32;
            Coordinate {
                col: row_en_passant,
                row: col_en_passant,
            }
        });

        board_from_fen
            .pieces
            .into_iter()
            .enumerate()
            .map(|(i, p)| p.map(|x| (i, x)))
            .flatten()
            .for_each(|(i, p)| {
                let row = (i % 8) as i32;
                let col = (i / 8) as i32;
                let player = match p.color {
                    fen::Color::White => Player::White,
                    fen::Color::Black => Player::Black,
                };
                let piece = match p.kind {
                    fen::PieceKind::Pawn => PieceType::Pawn,
                    fen::PieceKind::Knight => PieceType::Knight,
                    fen::PieceKind::Bishop => PieceType::Bishop,
                    fen::PieceKind::Rook => PieceType::Rook,
                    fen::PieceKind::Queen => PieceType::Queen,
                    fen::PieceKind::King => PieceType::King,
                };

                *board.square_idx_mut(Coordinate { row: col, col: row }) =
                    Some(Piece { piece, player })
            });

        board
    }

    fn notation_conversion(row: char, column: i32) -> Option<Coordinate> {
        if (row as i32) >= ('a' as i32)
            && (row as i32) <= ('h' as i32)
            && column >= 1
            && column <= 8
        {
            Some(Coordinate {
                col: (row as i32) - ('a' as i32),
                row: column - 1 as i32,
            })
        } else {
            None
        }
    }
    fn square_idx(&self, origin: Coordinate) -> &Option<Piece> {
        &self.board[origin.row as usize][origin.col as usize]
    }
    fn square_idx_mut(&mut self, origin: Coordinate) -> &mut Option<Piece> {
        &mut self.board[origin.row as usize][origin.col as usize]
    }

    fn print_board(&self, player_pov: Player) {
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

    fn is_occupied(&self, location: Coordinate) -> bool {
        self.square_idx(location).is_some()
    }

    fn player_at_location(&self, location: Coordinate) -> Option<Player> {
        self.square_idx(location).map(|piece| piece.player)
    }

    fn get_queen_rook_bishop_moves(
        &self,
        origin: Coordinate,
        directions: &[Coordinate],
    ) -> Vec<Ply> {
        let player = self.player_at_location(origin).unwrap();

        directions
            .iter()
            .map(|dir| {
                let mut plys = vec![];

                for i in 1.. {
                    let pos = origin.add(&dir.mult(i));

                    if !pos.is_valid_location() {
                        break;
                    }

                    if self.player_at_location(pos) == Some(player) {
                        break;
                    }

                    plys.push(Ply {
                        origin,
                        destination: pos,
                        promotion: None,
                    });

                    if self.player_at_location(pos) == Some(player.opponent()) {
                        break;
                    }
                }

                plys.into_iter()
            })
            .flatten()
            .collect()
    }

    fn get_king_knight_moves(&self, origin: Coordinate, directions: &[Coordinate]) -> Vec<Ply> {
        let player = self.player_at_location(origin).unwrap();

        directions
            .iter()
            .map(|delta| Coordinate {
                row: origin.row + delta.row,
                col: origin.col + delta.col,
            })
            .filter(|&pos| pos.is_valid_location() && self.player_at_location(pos) != Some(player))
            .map(|pos| Ply {
                origin,
                destination: pos,
                promotion: None,
            })
            .collect()
    }

    fn get_pawn_moves(&self, origin: Coordinate) -> Vec<Ply> {
        let player = self.player_at_location(origin).unwrap();

        let dir = player.advancing_direction();

        let mut results: Vec<Ply> = vec![];

        if !self.is_occupied(Coordinate {
            row: origin.row + dir,
            col: origin.col,
        }) {
            if origin.row == 0 || origin.row == 7 {
                for promo in PieceType::PROMOTIONS {
                    results.push(Ply {
                        origin,
                        destination: Coordinate {
                            row: origin.row + dir,
                            col: origin.col,
                        },
                        promotion: Some(promo),
                    })
                }
            } else {
                results.push(Ply {
                    origin,
                    destination: Coordinate {
                        row: origin.row + dir,
                        col: origin.col,
                    },
                    promotion: None,
                })
            }
        } else {
            return results;
        }

        if origin.row % 5 == 1 {
            if !self.is_occupied(Coordinate {
                row: origin.row + dir + dir,
                col: origin.col,
            }) {
                results.push(Ply {
                    origin,
                    destination: Coordinate {
                        row: origin.row + dir + dir,
                        col: origin.col,
                    },
                    promotion: None,
                })
            }
        }

        results
    }

    fn get_pawn_captures(&self, origin: Coordinate) -> Vec<Ply> {
        let player = self.player_at_location(origin).unwrap();
        let dir = player.advancing_direction();
        let left_capture = Coordinate {
            row: origin.row + dir,
            col: origin.col - 1,
        };
        let right_capture = Coordinate {
            row: origin.row + dir,
            col: origin.col + 1,
        };
        let mut results: Vec<Ply> = vec![];

        // Capture to the left
        if left_capture.is_valid_location()
            && self.player_at_location(left_capture) == Some(player.opponent())
        {
            if left_capture.row == 7 || left_capture.row == 0 {
                for promo in PieceType::PROMOTIONS {
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
        if left_capture.is_valid_location()
            && self.player_at_location(left_capture) == Some(player.opponent())
        {
            if right_capture.row == 7 || right_capture.row == 0 {
                for promo in PieceType::PROMOTIONS {
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
            if origin.row + dir == ep_square.row {
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
                if let Some(piece) = self.square_idx(Coordinate { row, col }) {
                    if self.turn == piece.player {
                        moves.extend(self.get_pseudo_legal_moves(Coordinate { row, col }).iter())
                    }
                }
            }
        }

        todo!()
    }

    fn get_castling_moves(&self, origin: Coordinate) -> Vec<Ply> {
        todo!()
    }

    fn get_pseudo_legal_moves(&self, location: Coordinate) -> Vec<Ply> {
        if let Some(piece_in_square) = self.square_idx(location) {
            let legal_moves = match piece_in_square.piece {
                PieceType::Pawn => vec![
                    self.get_pawn_moves(location),
                    self.get_pawn_captures(location),
                ]
                .concat(),
                PieceType::Rook => self.get_queen_rook_bishop_moves(location, &PieceType::DIR_ROOK),
                PieceType::Knight => self.get_king_knight_moves(location, &PieceType::DIR_KNIGHT),
                PieceType::Bishop => {
                    self.get_queen_rook_bishop_moves(location, &PieceType::DIR_BISHOP)
                }
                PieceType::Queen => {
                    self.get_queen_rook_bishop_moves(location, &PieceType::DIR_QUEEN_KING)
                }
                PieceType::King => self.get_king_knight_moves(location, &PieceType::DIR_QUEEN_KING),
            };
            legal_moves
        } else {
            vec![]
        }
    }

    fn arbiter(&self, ply: &Ply) -> bool {
        let legal_moves = self.get_pseudo_legal_moves(ply.origin);

        legal_moves.contains(ply)
    }

    fn make_move(&self, ply: Ply) -> Board {
        let mut new_game_state = self.clone();

        let player = self.player_at_location(ply.origin).unwrap();
        let piece = self.square_idx(ply.origin).unwrap().piece;

        let dir = player.advancing_direction();

        // Detecting if move was en passant
        if piece == PieceType::Pawn
            && ply.origin.col != ply.destination.col
            && self.square_idx(ply.destination).is_none()
        {
            *new_game_state.square_idx_mut(Coordinate {
                col: ply.destination.col,
                row: ply.destination.row - dir,
            }) = None;
        }

        // Updating en_passant_square
        new_game_state.en_passant_square =
            if piece == PieceType::Pawn && ply.destination.row == ply.origin.row + 2 * dir {
                Some(Coordinate {
                    row: ply.origin.row + dir,
                    col: ply.origin.col,
                })
            } else {
                None
            };

        // Promotion
        if let Some(new_piece) = ply.promotion {
            new_game_state
                .square_idx_mut(ply.origin)
                .as_mut()
                .unwrap()
                .piece = new_piece;
        }

        *new_game_state.square_idx_mut(ply.destination) = *new_game_state.square_idx(ply.origin);
        *new_game_state.square_idx_mut(ply.origin) = None;

        new_game_state.turn = new_game_state.turn.opponent();
        new_game_state
    }

    fn verify_status(&self) -> String {
        // Verifica se o jogo acaba em checkmate ou não
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Game {
    states: Vec<Board>,
}

impl Game {
    fn new() -> Game {
        Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    fn new_from_fen(fen: &str) -> Game {
        Game {
            states: vec![Board::new_from_fen(fen)],
        }

        // Test Fen
        // Board::new_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b K- e3 0 1")
    }

    fn get_user_move(&self) -> Option<Ply> {
        println!("Move?");
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split(" ")
            .map(|s| {
                let mut iter = s.chars();
                let c1 = iter.next()?;
                let c2 = iter.next()? as i32 - ('0' as i32);

                if iter.next().is_none() {
                    Board::notation_conversion(c1, c2)
                } else {
                    None
                }
            })
            .collect::<Option<Vec<_>>>()
            .filter(|v| v.len() == 3)
            .map(|v| Ply {
                origin: v[0],
                destination: v[1],
                promotion: None,
            })
    }

    fn play(&mut self) {
        loop {
            self.states
                .last()
                .unwrap()
                .print_board(self.states.last().unwrap().turn);

            let Some(ply) = self.get_user_move() else {
                println!("Invalid input text.");
                continue;
            };

            if !self.states.last().unwrap().arbiter(&ply) {
                println!("That move is not allowed, idiot.");
                continue;
            }

            self.states.push(self.states.last().unwrap().make_move(ply));
        }
    }
}

fn main() {
    // let mut game = Game::new_from_fen("1r2kr2/pp1p1p2/2p4p/6pP/P1PP4/1P6/5PP1/R3K2R w KQ g6 0 21");
    let mut game = Game::new();

    game.play();
}
