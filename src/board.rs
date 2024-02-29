use crate::{
    coord::Coord,
    piece::{Kind, Piece},
    player::Player,
    ply::Ply,
    status::Status,
};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub turn: Player,

    pub board: [[Option<Piece>; 8]; 8],
    pub white_pieces: Vec<Piece>,
    pub black_pieces: Vec<Piece>,
    pub white_king_loc: Coord,
    pub black_king_loc: Coord,

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

    pub fn reverse_notation_conversion(coord: Coord) -> (char, i32) {
        let row = coord.row + 1;
        let col = coord.col + ('a' as i32);

        if row >= 1 && row <= 8 && col >= ('a' as i32) && col <= ('h' as i32) {
            (col as u8 as char, row)
        } else {
            ('a', 0)
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
                let pos = Coord {
                    row: (i / 8) as i32,
                    col: (i % 8) as i32,
                };
                let color = match p.color {
                    fen::Color::White => Player::White,
                    fen::Color::Black => Player::Black,
                };
                let kind = match p.kind {
                    fen::PieceKind::Pawn => Kind::Pawn,
                    fen::PieceKind::Knight => Kind::Knight,
                    fen::PieceKind::Bishop => Kind::Bishop,
                    fen::PieceKind::Rook => Kind::Rook,
                    fen::PieceKind::Queen => Kind::Queen,
                    fen::PieceKind::King => Kind::King,
                };

                if p.kind == fen::PieceKind::King {
                    match color {
                        Player::White => {
                            board.white_king_loc = pos;
                        }
                        Player::Black => {
                            board.black_king_loc = pos;
                        }
                    }
                }

                let piece = Piece {
                    kind,
                    player: color,
                    coord: pos,
                    idx: 0,
                };

                board.add_piece_to_empty_square(piece);
            });

        // println!(
        //     "nº of black pieces: {}, black pieces: {:?}",
        //     board.black_pieces.len(),
        //     board.black_pieces
        // );
        // println!("----------------------");
        // println!(
        //     "nº of white pieces: {}, white pieces: {:?}",
        //     board.white_pieces.len(),
        //     board.white_pieces
        // );
        // println!("----------------------");
        // println!("black king loc {:?}", board.black_king_loc);
        // println!("white king loc {:?}", board.white_king_loc);
        // board.print_board(Player::White);

        // Board::check_everything(&board);

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

    /// Adds a piece to an empty square
    /// Index management is done here
    fn add_piece_to_empty_square(&mut self, mut p: Piece) {
        assert!(self.board[p.coord.row as usize][p.coord.col as usize].is_none());
        match p.player {
            Player::Black => {
                p.idx = self.black_pieces.len();
                self.black_pieces.push(p);
            }
            Player::White => {
                p.idx = self.white_pieces.len();
                self.white_pieces.push(p);
            }
        }
        self.board[p.coord.row as usize][p.coord.col as usize] = Some(p);
    }

    /// Removes a piece from an occupied square by providing the coordinate
    /// Does nothing if the square is not occupied
    /// Index management is done here
    fn remove_piece_from_occupied_square(&mut self, coord: Coord) {
        if let &Some(p) = self.get_piece_by_coord(coord) {
            match p.player {
                Player::Black => {
                    self.black_pieces.remove(p.idx);
                    for piece in self.black_pieces.iter_mut().skip(p.idx) {
                        piece.idx -= 1;
                        self.board[piece.coord.row as usize][piece.coord.col as usize]
                            .iter_mut()
                            .for_each(|x| x.idx -= 1);
                    }
                }
                Player::White => {
                    self.white_pieces.remove(p.idx);
                    for piece in self.white_pieces.iter_mut().skip(p.idx) {
                        piece.idx -= 1;
                        self.board[piece.coord.row as usize][piece.coord.col as usize]
                            .iter_mut()
                            .for_each(|x| x.idx -= 1);
                    }
                }
            };
            self.board[coord.row as usize][coord.col as usize] = None;
        }
    }

    pub fn check_everything(board: &Board) {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(p) = board.get_piece_by_coord(Coord { row, col }) {
                    match p.player {
                        Player::Black => {
                            // println!(
                            //     "black[p.idx] = {:?}, *p = {:?}",
                            //     board.black_pieces[p.idx], *p
                            // );
                            assert!(board.black_pieces[p.idx] == *p)
                        }
                        Player::White => {
                            // println!(
                            //     "white[p.idx] = {:?}, *p = {:?}",
                            //     board.white_pieces[p.idx], *p
                            // );
                            assert!(board.white_pieces[p.idx] == *p)
                        }
                    }
                };
            }
        }
    }

    /// Removes a piece from an occupied square by providing the piece
    fn remove_piece_from_occupied_square_by_piece(&mut self, p: Piece) {
        assert!(self.board[p.coord.row as usize][p.coord.col as usize] == Some(p));
        self.board[p.coord.row as usize][p.coord.col as usize] = None;

        match p.player {
            Player::Black => {
                self.black_pieces.remove(p.idx);
                for piece in &mut self.black_pieces[p.idx..] {
                    piece.idx -= 1;
                    self.board[piece.coord.row as usize][piece.coord.col as usize]
                        .iter_mut()
                        .for_each(|x| x.idx -= 1);
                }
            }
            Player::White => {
                self.white_pieces.remove(p.idx);
                for piece in &mut self.white_pieces[p.idx..] {
                    piece.idx -= 1;
                    self.board[piece.coord.row as usize][piece.coord.col as usize]
                        .iter_mut()
                        .for_each(|x| x.idx -= 1);
                }
            }
        };
    }

    /// Moves a piece from a square to another (includes captures)
    /// Updates king location
    fn move_piece(&mut self, mut p: Piece, destination: Coord) {
        assert!(self.board[p.coord.row as usize][p.coord.col as usize] == Some(p));

        self.remove_piece_from_occupied_square(destination);

        self.board[p.coord.row as usize][p.coord.col as usize] = None;
        p.coord = destination;

        self.board[destination.row as usize][destination.col as usize] = Some(p);
        match p.player {
            Player::Black => {
                self.black_pieces[p.idx] = p;
            }
            Player::White => {
                self.white_pieces[p.idx] = p;
            }
        };

        if p.kind == Kind::King {
            self.move_king(p.player, destination);
        }
    }

    fn promote_piece(&mut self, mut p: Piece, promo: Kind) {
        match p.player {
            Player::Black => {
                p.kind = promo;
                self.black_pieces[p.idx] = p;
            }
            Player::White => {
                p.kind = promo;
                self.white_pieces[p.idx] = p;
            }
        }
        self.board[p.coord.row as usize][p.coord.col as usize] = Some(p);
    }

    fn move_piece_by_coord(&mut self, origin: Coord, destination: Coord) {
        if let &Some(mut p) = self.get_piece_by_coord(origin) {
            self.remove_piece_from_occupied_square(destination);
            self.board[p.coord.row as usize][p.coord.col as usize] = None;
            p.coord = destination;
            self.board[destination.row as usize][destination.col as usize] = Some(p);
            match p.player {
                Player::Black => {
                    self.black_pieces[p.idx] = p;
                }
                Player::White => {
                    self.white_pieces[p.idx] = p;
                }
            };
            if p.kind == Kind::King {
                self.move_king(p.player, destination);
            }
        }
    }

    fn move_king(&mut self, player: Player, destination: Coord) {
        match player {
            Player::Black => self.black_king_loc = destination,
            Player::White => self.white_king_loc = destination,
        }
    }

    // fn modify_piece_idx_in_board(&mut self, p: Piece, new_idx: usize) {
    //     if let &Some(mut piece) = self.get_piece_by_coord(p.coord) {
    //         piece.idx = new_idx;
    //         self.board[p.coord.row as usize][p.coord.col as usize] = Some(piece);
    //     }

    //     if let Some(x) = self.board[p.coord.row as usize][p.coord.col as usize].as_mut() {
    //         x.idx = new_idx;
    //     }

    //     self.board[p.coord.row as usize][p.coord.col as usize]
    //         .iter_mut()
    //         .for_each(|x| x.idx = new_idx);
    // }

    fn get_piece_by_coord(&self, coord: Coord) -> &Option<Piece> {
        &self.board[coord.row as usize][coord.col as usize]
    }
    fn get_piece_by_index(&self, idx: usize, player: Player) -> Piece {
        match player {
            Player::Black => {
                assert!(idx < self.black_pieces.len());
                self.black_pieces[idx]
            }
            Player::White => {
                assert!(idx < self.white_pieces.len());
                self.white_pieces[idx]
            }
        }
    }

    fn is_square_occupied(&self, location: Coord) -> bool {
        self.get_piece_by_coord(location).is_some()
    }
    fn player_at_square(&self, location: Coord) -> Option<Player> {
        self.get_piece_by_coord(location).map(|piece| piece.player)
    }
    fn kind_at_square(&self, location: Coord) -> Option<Kind> {
        self.get_piece_by_coord(location).map(|piece| piece.kind)
    }

    fn get_pseudo_legal_moves(&self, coord: Coord) -> Vec<Ply> {
        if let Some(piece_in_square) = self.get_piece_by_coord(coord) {
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
                Kind::King => vec![
                    self.get_king_knight_moves(coord, &Coord::LIST_CARDINAL_DIAGONAL),
                    self.get_castling_moves(),
                ]
                .concat(),
            }
        } else {
            vec![]
        }
    }

    // Returns a vector with all the legal moves possible for the piece at position coord
    fn get_legal_moves(&self, coord: Coord) -> Vec<Ply> {
        let pseudo_legal_moves = self.get_pseudo_legal_moves(coord);
        let current_turn = self.turn;
        let mut results: Vec<Ply> = vec![];

        pseudo_legal_moves.iter().for_each(|m| {
            let pos_after_move = self.make_move(*m);

            let king_pos = pos_after_move.find_king(current_turn);
            if !pos_after_move.is_square_attacked(king_pos, pos_after_move.turn) {
                results.push(*m);
            };
        });
        results
    }

    fn find_king(&self, player: Player) -> Coord {
        match player {
            Player::Black => self.black_king_loc,
            Player::White => self.white_king_loc,
        }
    }
    // Gives all posible moves in a position
    pub fn get_all_moves(&self) -> Vec<Ply> {
        let mut moves: Vec<Ply> = vec![];

        match self.turn {
            Player::Black => {
                for p in &self.black_pieces {
                    moves.extend(self.get_legal_moves(p.coord));
                }
            }
            Player::White => {
                for p in &self.white_pieces {
                    moves.extend(self.get_legal_moves(p.coord));
                }
            }
        }

        moves
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

        let piece = self.get_piece_by_coord(ply.origin).unwrap();
        let dir = piece.player.advancing_direction();

        // Detect if move was capture or pawn push and update half_move clock
        if piece.kind == Kind::Pawn || self.is_square_occupied(ply.destination) {
            new_game_state.half_move_clock = 0;
        } else {
            new_game_state.half_move_clock += 1;
        }

        // Detect if move was en passant and remove the captured pawn
        if piece.kind == Kind::Pawn
            && ply.origin.col != ply.destination.col
            && self.get_piece_by_coord(ply.destination).is_none()
        {
            new_game_state.remove_piece_from_occupied_square(ply.destination - dir);
        }

        // Update en_passant_square
        new_game_state.en_passant_square =
            if piece.kind == Kind::Pawn && ply.destination == ply.origin + 2 * dir {
                Some(ply.origin + dir)
            } else {
                None
            };

        // Detect if move was castle and move rook to correct position
        if piece.kind == Kind::King {
            if ply.destination == ply.origin + 2 * Coord::R {
                new_game_state
                    .move_piece_by_coord(ply.destination + Coord::R, ply.destination + Coord::L);
            }

            if ply.destination == ply.origin + 2 * Coord::L {
                new_game_state.move_piece_by_coord(
                    ply.destination + 2 * Coord::L,
                    ply.destination + Coord::R,
                );
            }
        }

        // Update castle permissions
        if piece.kind == Kind::King {
            match piece.player {
                Player::Black => {
                    new_game_state.black_can_oo = false;
                    new_game_state.black_can_ooo = false;
                }
                Player::White => {
                    new_game_state.white_can_oo = false;
                    new_game_state.white_can_ooo = false;
                }
            }
        }

        [
            (
                &mut new_game_state.white_can_ooo,
                Player::White.home_row(),
                0,
            ),
            (
                &mut new_game_state.white_can_oo,
                Player::White.home_row(),
                7,
            ),
            (
                &mut new_game_state.black_can_ooo,
                Player::Black.home_row(),
                0,
            ),
            (
                &mut new_game_state.black_can_oo,
                Player::Black.home_row(),
                7,
            ),
        ]
        .into_iter()
        .for_each(|(castle_perm, row, col)| {
            let coord = Coord { row, col };
            if ply.origin == coord || ply.destination == coord {
                *castle_perm = false;
            }
        });

        // Set piece (including promotions) on new square
        if let Some(promo) = ply.promotion {
            new_game_state.promote_piece(piece, promo);
        }
        new_game_state.move_piece_by_coord(ply.origin, ply.destination);

        // Change turn and return new game state
        new_game_state.turn = new_game_state.turn.opponent();
        new_game_state
    }

    fn get_pawn_moves(&self, origin: Coord) -> Vec<Ply> {
        let player = self.player_at_square(origin).unwrap();
        let dir = player.advancing_direction();

        let mut results: Vec<Ply> = vec![];

        if !self.is_square_occupied(origin + dir) {
            if (origin + dir).row == 0 || (origin + dir).row == 7 {
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

        if origin.row == 1 && player == Player::White || origin.row == 6 && player == Player::Black
        {
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

        let mut results: Vec<Ply> = [Coord::L, Coord::R]
            .iter()
            .map(|&c| origin + c + player.advancing_direction())
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
        if let Some(en_passant_square) = self.en_passant_square {
            if (origin + player.advancing_direction()).row == en_passant_square.row {
                if (origin.col + 1 == en_passant_square.col)
                    || (origin.col - 1 == en_passant_square.col)
                {
                    results.push(Ply {
                        origin,
                        destination: en_passant_square,
                        promotion: None,
                    })
                }
            }
        }

        results
    }

    pub fn is_square_attacked(&self, origin: Coord, by_player: Player) -> bool {
        Coord::LIST_CARDINAL
            .iter()
            .map(|c| (c, Kind::Rook))
            .chain(Coord::LIST_DIAGONAL.iter().map(|c| (c, Kind::Bishop)))
            .any(|(&dir, piece)| {
                (1..)
                    .map(move |i| origin + dir * i)
                    .take_while(|&c| c.is_valid())
                    .take_while(|&c| self.player_at_square(c) != Some(by_player.opponent()))
                    .take_while(move |&c| self.player_at_square(c - dir) != Some(by_player))
                    .any(|c| {
                        self.player_at_square(c) == Some(by_player)
                            && (self.kind_at_square(c) == Some(piece)
                                || self.kind_at_square(c) == Some(Kind::Queen))
                    })
            })
            || [Coord::L, Coord::R]
                .iter()
                .map(|&c| origin + c - by_player.advancing_direction())
                .filter(|&c| c.is_valid())
                .any(|pos| {
                    if let Some(p) = self.get_piece_by_coord(pos) {
                        p.kind == Kind::Pawn && p.player == by_player
                    } else {
                        false
                    }
                })
            || Coord::LIST_KNIGHT
                .iter()
                .map(|&c| origin + c)
                .filter(|&c| c.is_valid())
                .any(|pos| {
                    if let Some(p) = self.get_piece_by_coord(pos) {
                        p.kind == Kind::Knight && p.player == by_player
                    } else {
                        false
                    }
                })
            || Coord::LIST_CARDINAL_DIAGONAL
                .iter()
                .map(|&c| origin + c)
                .filter(|&c| c.is_valid())
                .any(|pos| {
                    if let Some(p) = self.get_piece_by_coord(pos) {
                        p.kind == Kind::King && p.player == by_player
                    } else {
                        false
                    }
                })
    }

    fn get_castling_moves(&self) -> Vec<Ply> {
        let player = self.turn;
        let mut results: Vec<Ply> = vec![];

        // Como reduzir estes dois bocados de código quase idênticos?
        match player {
            Player::White => {
                if self.white_can_oo
                    && !self.is_square_attacked(Coord { row: 0, col: 4 }, player.opponent())
                    && !self.is_square_attacked(Coord { row: 0, col: 5 }, player.opponent())
                    && !self.is_square_attacked(Coord { row: 0, col: 6 }, player.opponent())
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
                    && !self.is_square_attacked(Coord { row: 0, col: 4 }, player.opponent())
                    && !self.is_square_attacked(Coord { row: 0, col: 2 }, player.opponent())
                    && !self.is_square_attacked(Coord { row: 0, col: 3 }, player.opponent())
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
                    && !self.is_square_attacked(Coord { row: 7, col: 4 }, player.opponent())
                    && !self.is_square_attacked(Coord { row: 7, col: 5 }, player.opponent())
                    && !self.is_square_attacked(Coord { row: 7, col: 6 }, player.opponent())
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
                    && !self.is_square_attacked(Coord { row: 7, col: 4 }, player.opponent())
                    && !self.is_square_attacked(Coord { row: 7, col: 2 }, player.opponent())
                    && !self.is_square_attacked(Coord { row: 7, col: 3 }, player.opponent())
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
        // Verifies if there's a piece in square, and if it's the right turn
        if let Some(piece) = self.get_piece_by_coord(ply.origin) {
            if piece.player != self.turn {
                return false;
            }
        }
        self.get_legal_moves(ply.origin).contains(ply)
    }

    pub fn verify_status(&self) -> Status {
        let king_pos = self.find_king(self.turn);

        if self.half_move_clock == 100 {
            return Status::Draw;
        }

        if !self.get_all_moves().is_empty() {
            return Status::Ongoing;
        }

        if !self.is_square_attacked(king_pos, self.turn.opponent()) {
            return Status::Draw;
        }

        match self.turn {
            Player::Black => Status::WWin,
            Player::White => Status::BWin,
        }
    }
}
