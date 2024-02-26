use crate::{
    coord::Coord,
    piece::{Kind, Piece, PieceLoc},
    player::Player,
    ply::Ply,
    status::Status,
};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Board {
    pub turn: Player,

    board: [[Option<Piece>; 8]; 8],
    white_pieces: [Option<PieceLoc>; 16],
    black_pieces: [Option<PieceLoc>; 16],
    white_king_loc: Coord,
    black_king_loc: Coord,

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

    fn add_piece(&mut self, p: PieceLoc, t: Player) {
        match t {
            Player::Black => {
                if let Some(slot) = self.black_pieces.iter_mut().find(|p| p.is_none()) {
                    *slot = Some(p);
                }
            }
            Player::White => {
                if let Some(slot) = self.white_pieces.iter_mut().find(|p| p.is_none()) {
                    *slot = Some(p);
                }
            }
        }
    }

    fn sort_pieces(&mut self, player: Player) {
        let mut partition_point = 0;

        match player {
            Player::Black => {
                for i in 0..self.black_pieces.len() {
                    if !self.black_pieces[i].is_none() {
                        self.black_pieces.swap(i, partition_point);
                        partition_point += 1;
                    }
                }
            }
            Player::White => {
                for i in 0..self.white_pieces.len() {
                    if !self.white_pieces[i].is_none() {
                        self.white_pieces.swap(i, partition_point);
                        partition_point += 1;
                    }
                }
            }
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

                board.square_set(
                    pos,
                    Some(Piece {
                        player: color,
                        kind,
                    }),
                );

                match p.color {
                    fen::Color::White => {
                        board.add_piece(PieceLoc { kind, loc: pos }, Player::White)
                    }
                    fen::Color::Black => {
                        board.add_piece(PieceLoc { kind, loc: pos }, Player::Black)
                    }
                }
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
    fn find_king(&self, player: Player) -> Coord {
        match player {
            Player::Black => return self.black_king_loc,
            Player::White => return self.white_king_loc,
        }
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

    // Gives all posible moves in a position
    pub fn get_all_moves(&self) -> Vec<Ply> {
        let mut moves: Vec<Ply> = vec![];

        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.square_get(Coord { row, col }) {
                    if self.turn == piece.player {
                        moves.extend(self.get_legal_moves(Coord { row, col }))
                    }
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

    fn is_square_attacked(&self, origin: Coord, by_player: Player) -> bool {
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
                    self.square_get(pos)
                        == &Some(Piece {
                            kind: Kind::Pawn,
                            player: by_player,
                        })
                })
            || Coord::LIST_KNIGHT
                .iter()
                .map(|&c| origin + c)
                .filter(|&c| c.is_valid())
                .any(|pos| {
                    self.square_get(pos)
                        == &Some(Piece {
                            kind: Kind::Knight,
                            player: by_player,
                        })
                })
    }

    fn update_piece_loc_capture(&mut self, coord: Coord) {
        let piece = self.square_get(coord).unwrap();
        // Checks if a piece was captured and updates PieceLoc of captured piece
        match piece.player {
            Player::Black => {
                if let Some(_) = self.square_get(coord) {
                    for i in 0..16 {
                        if let Some(ploc) = self.white_pieces[i] {
                            if ploc.loc == coord {
                                self.white_pieces[i] = None;
                                self.sort_pieces(Player::White);
                                break;
                            }
                        }
                    }
                }
            }
            Player::White => {
                if let Some(_) = self.square_get(coord) {
                    for i in 0..16 {
                        if let Some(ploc) = self.black_pieces[i] {
                            if ploc.loc == coord {
                                self.black_pieces[i] = None;
                                self.sort_pieces(Player::Black);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn make_move(&self, ply: Ply) -> Board {
        let mut new_game_state = self.clone();

        let piece = new_game_state.square_get(ply.origin).unwrap();
        let dir = piece.player.advancing_direction();

        // Update moved piece PieceLoc
        match piece.player {
            Player::Black => {
                for p in new_game_state.black_pieces {
                    if let Some(mut ploc) = p {
                        if ploc.loc == ply.origin {
                            ploc.loc = ply.destination;
                        }
                    }
                }
            }
            Player::White => {
                for p in new_game_state.white_pieces {
                    if let Some(mut ploc) = p {
                        if ploc.loc == ply.origin {
                            ploc.loc = ply.destination;
                        }
                    }
                }
            }
        }

        // Update captured piece pieceLoc

        // Detect if move was capture or pawn push and update half_move clock
        if piece.kind == Kind::Pawn || new_game_state.is_square_occupied(ply.destination) {
            new_game_state.half_move_clock = 0;
        } else {
            new_game_state.half_move_clock += 1;
        }

        // Detect if move was en passant and remove the captured pawn
        if piece.kind == Kind::Pawn
            && ply.origin.col != ply.destination.col
            && new_game_state.square_get(ply.destination).is_none()
        {
            new_game_state.square_set(ply.destination - dir, None);
        }

        // Updates PieceLoc for en passant case

        // Update en_passant_square
        new_game_state.en_passant_square =
            if piece.kind == Kind::Pawn && ply.destination == ply.origin + 2 * dir {
                Some(ply.origin + dir)
            } else {
                None
            };

        // Update king location
        // Detect if move was castle and move rook to correct position
        match piece.player {
            Player::Black => new_game_state.black_king_loc = ply.destination,
            Player::White => new_game_state.white_king_loc = ply.destination,
        }

        if piece.kind == Kind::King {
            if ply.destination == ply.origin + 2 * Coord::R {
                new_game_state.square_set(
                    ply.destination + Coord::L,
                    Some(Piece {
                        kind: Kind::Rook,
                        player: piece.player,
                    }),
                );

                new_game_state.square_set(ply.destination + Coord::R, None);
            }

            if ply.destination == ply.origin + 2 * Coord::L {
                new_game_state.square_set(
                    ply.destination + Coord::R,
                    Some(Piece {
                        kind: Kind::Rook,
                        player: piece.player,
                    }),
                );

                new_game_state.square_set(ply.destination + 2 * Coord::L, None);
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
        new_game_state.square_set(
            ply.destination,
            Some(Piece {
                kind: ply.promotion.unwrap_or(piece.kind),
                player: piece.player,
            }),
        );

        // Remove piece from old square
        new_game_state.square_set(ply.origin, None);

        // Change turn and return new game state
        new_game_state.turn = new_game_state.turn.opponent();
        new_game_state
    }

    pub fn arbiter(&self, ply: &Ply) -> bool {
        // Verifies if there's a piece in square, and if it's the right turn
        if let Some(piece) = self.square_get(ply.origin) {
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
