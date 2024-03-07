use std::num::NonZeroU64;

use crate::{
    coord::Coord,
    piece::{self, Kind, Piece},
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
                match p.player {
                    Player::Black => self.black_king_loc = destination,
                    Player::White => self.white_king_loc = destination,
                }
            }
        }
    }

    pub fn get_piece_by_coord(&self, coord: Coord) -> &Option<Piece> {
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

    /// Function for debugging.
    /// Checks if both the board and the piece vectors in the data structure are properly synced up.
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

    // TODO: returns iter
    fn get_pseudo_legal_moves(&self, coord: Coord) -> Vec<Ply> {
        if let Some(piece_in_square) = self.get_piece_by_coord(coord) {
            match piece_in_square.kind {
                Kind::Pawn => {
                    vec![self.get_pawn_moves(coord), self.get_pawn_captures(coord)].concat()
                    // TODO: returns iter
                }
                Kind::Knight => self.get_knight_moves(coord, &Coord::LIST_KNIGHT), // TODO: returns iter
                Kind::Bishop => self.get_queen_rook_bishop_moves(coord, &Coord::LIST_DIAGONAL), // TODO: returns iter
                Kind::Rook => self.get_queen_rook_bishop_moves(coord, &Coord::LIST_CARDINAL),
                Kind::Queen => {
                    self.get_queen_rook_bishop_moves(coord, &Coord::LIST_CARDINAL_DIAGONAL)
                }
                Kind::King => vec![
                    self.get_king_moves(coord, &Coord::LIST_CARDINAL_DIAGONAL), // TODO: returns iter
                    self.get_castling_moves(), // TODO: returns iter
                ]
                .concat(),
            }
        } else {
            vec![]
        }
    }

    // Returns a vector with all the legal moves possible for the piece at position coord
    // TODO: returns iter
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

    pub fn new_get_legal_moves(&self, coord: Coord) -> Vec<Ply> {
        if let Some(piece_in_square) = self.get_piece_by_coord(coord) {
            let king_loc = match self.turn {
                Player::Black => self.black_king_loc,
                Player::White => self.white_king_loc,
            };

            let checking_pieces = self.square_attacked_by_pieces(king_loc, self.turn.opponent());

            if checking_pieces.len() == 1 {
                match piece_in_square.kind {
                    Kind::King => self.get_king_moves(coord, &Coord::LIST_CARDINAL_DIAGONAL),
                    _ => {
                        if let Some(_) = self.is_piece_pinned(*piece_in_square) {
                            return vec![];
                        }
                        match checking_pieces[0].kind {
                            Kind::Pawn | Kind::Knight => self
                                .get_pseudo_legal_moves(coord)
                                .into_iter()
                                .filter(|ply| ply.destination == checking_pieces[0].coord)
                                .collect(),
                            Kind::Rook | Kind::Bishop | Kind::Queen => {
                                let dir = Coord::find_dir_between_coords(
                                    checking_pieces[0].coord,
                                    king_loc,
                                );

                                let mut pos = king_loc + dir;
                                let mut possible_destinations: Vec<Coord> = vec![];
                                while pos != checking_pieces[0].coord + dir {
                                    possible_destinations.push(pos);
                                    pos = pos + dir;
                                }

                                self.get_pseudo_legal_moves(coord)
                                    .into_iter()
                                    .filter(|ply| possible_destinations.contains(&ply.destination))
                                    .collect()
                            }
                            Kind::King => panic!("King cannot be the checking piece"),
                        }
                    }
                }
            } else if checking_pieces.len() == 2 {
                match piece_in_square.kind {
                    Kind::King => self.get_king_moves(coord, &Coord::LIST_CARDINAL_DIAGONAL),
                    _ => {
                        vec![]
                    }
                }
            } else {
                if let Some(pin_direction) = self.is_piece_pinned(*piece_in_square) {
                    match piece_in_square.kind {
                        Kind::Pawn => self
                            .get_pawn_captures(coord)
                            .into_iter()
                            .filter(|ply| {
                                ply.destination
                                    == piece_in_square.coord
                                        // + piece_in_square.player.advancing_direction()
                                        + pin_direction
                            })
                            .collect(),
                        Kind::Knight => vec![],
                        Kind::Rook => self.get_queen_rook_bishop_moves(
                            coord,
                            Coord::LIST_CARDINAL
                                .into_iter()
                                .filter(|&dir| dir == pin_direction || dir == -1 * pin_direction)
                                .collect::<Vec<_>>()
                                .as_slice(),
                        ),
                        Kind::Bishop => self.get_queen_rook_bishop_moves(
                            coord,
                            Coord::LIST_DIAGONAL
                                .into_iter()
                                .filter(|&dir| dir == pin_direction || dir == -1 * pin_direction)
                                .collect::<Vec<_>>()
                                .as_slice(),
                        ),
                        Kind::Queen => self.get_queen_rook_bishop_moves(
                            coord,
                            Coord::LIST_CARDINAL_DIAGONAL
                                .into_iter()
                                .filter(|&dir| dir == pin_direction || dir == -1 * pin_direction)
                                .collect::<Vec<_>>()
                                .as_slice(),
                        ),
                        Kind::King => unreachable!(),
                    }
                } else {
                    self.get_pseudo_legal_moves(coord)
                }

                // @@ Verificar se estamos pinned
                // *** Se sim:
                // -> caso "piece_in_square.kind = Queen | Bishop | Rook" basta chamar o "get_queen_rook_bishop_moves"
                // apenas com as direções em que o movimento é permitido (filtrar Rook/Bishop/Queen pelas direções
                // permitidas pelo pin). Obtemos essas direções a partir do vetor rei_loc em direção a coord
                // -> caso "piece_in_square.kind = Knight" return []
                // -> caso "piece_in_square.kind = Pawn", não permitir normal pawn moves, permitir capturas apenas
                // no caso em que podem comer o bispo que está a dar pin
                // *** Se não: return get_(piece_in_square.kind_moves)
            }
        } else {
            vec![]
        }
    }

    pub fn is_piece_pinned(&self, p: Piece) -> Option<Coord> {
        if p.kind == Kind::King {
            return None;
        }

        let king_loc = match p.player {
            Player::Black => self.black_king_loc,
            Player::White => self.white_king_loc,
        };

        let dir = Coord::find_dir_between_coords(p.coord, king_loc);

        for i in 1..8 {
            let pos = king_loc + i * dir;
            if !pos.is_valid() || (self.is_square_occupied(pos) && pos != p.coord) {
                return None;
            };
            if pos == p.coord {
                for j in 1..8 {
                    let pos_after_piece = p.coord + j * dir;
                    if !pos_after_piece.is_valid() {
                        return None;
                    }

                    if let Some(piece) = self.get_piece_by_coord(pos_after_piece) {
                        let pinning_piece = {
                            if dir.row == dir.col || dir.row == -1 * dir.col {
                                Kind::Bishop
                            } else {
                                Kind::Rook
                            }
                        };

                        if piece.player == p.player
                            || (piece.kind != pinning_piece && piece.kind != Kind::Queen)
                        {
                            return None;
                        }

                        if piece.kind == pinning_piece || piece.kind == Kind::Queen {
                            return Some(dir);
                        }
                    }
                }
            }
        }
        None
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
                    moves.extend(self.new_get_legal_moves(p.coord));
                }
            }
            Player::White => {
                for p in &self.white_pieces {
                    moves.extend(self.new_get_legal_moves(p.coord));
                }
            }
        }

        moves
    }

    fn get_king_moves(&self, origin: Coord, directions: &[Coord]) -> Vec<Ply> {
        let player = self.player_at_square(origin).unwrap();

        directions
            .iter()
            .map(|&delta| origin + delta)
            .filter(|&pos| {
                pos.is_valid()
                    && self.player_at_square(pos) != Some(player)
                    && !self.is_square_attacked(pos, player.opponent())
            })
            .map(|pos| Ply {
                origin,
                destination: pos,
                promotion: None,
            })
            .collect()
    }

    fn get_knight_moves(&self, origin: Coord, directions: &[Coord]) -> Vec<Ply> {
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
                    .filter(|&c| self.player_at_square(c) == Some(by_player))
                    .any(|c| {
                        self.kind_at_square(c) == Some(piece)
                            || self.kind_at_square(c) == Some(Kind::Queen)
                    })
            })
            || [Coord::L, Coord::R]
                .iter()
                .map(|&c| origin + c - by_player.advancing_direction())
                .filter(|&c| c.is_valid())
                .map(|c| self.get_piece_by_coord(c))
                .flatten()
                .any(|p| p.kind == Kind::Pawn && p.player == by_player)
            || [
                (Coord::LIST_KNIGHT, Kind::Knight),
                (Coord::LIST_CARDINAL_DIAGONAL, Kind::King),
            ]
            .into_iter()
            .any(|(coords, piece)| {
                coords
                    .iter()
                    .map(|&c| origin + c)
                    .filter(|&c| c.is_valid())
                    .map(|c| self.get_piece_by_coord(c))
                    .flatten()
                    .any(|p| p.kind == piece && p.player == by_player)
            })
    }

    fn square_attacked_by_pieces(&self, origin: Coord, by_player: Player) -> Vec<Piece> {
        let mut results: Vec<Piece> = vec![];

        Coord::LIST_CARDINAL
            .iter()
            .map(|c| (c, Kind::Rook))
            .chain(Coord::LIST_DIAGONAL.iter().map(|c| (c, Kind::Bishop)))
            .for_each(|(&dir, piece)| {
                (1..)
                    .map(move |i| origin + dir * i)
                    .take_while(|&c| c.is_valid())
                    .take_while(|&c| self.player_at_square(c) != Some(by_player.opponent()))
                    .take_while(move |&c| self.player_at_square(c - dir) != Some(by_player))
                    .filter(|&c| self.player_at_square(c) == Some(by_player))
                    .for_each(|c| {
                        if self.kind_at_square(c) == Some(piece)
                            || self.kind_at_square(c) == Some(Kind::Queen)
                        {
                            results.push(self.get_piece_by_coord(c).unwrap());
                        }
                    })
            });

        [Coord::L, Coord::R]
            .iter()
            .map(|&c| origin + c - by_player.advancing_direction())
            .filter(|&c| c.is_valid())
            .map(|c| self.get_piece_by_coord(c))
            .flatten()
            .for_each(|p| {
                if p.kind == Kind::Pawn && p.player == by_player {
                    results.push(*p);
                }
            });

        [
            (Coord::LIST_KNIGHT, Kind::Knight),
            (Coord::LIST_CARDINAL_DIAGONAL, Kind::King),
        ]
        .into_iter()
        .for_each(|(coords, piece)| {
            coords
                .iter()
                .map(|&c| origin + c)
                .filter(|&c| c.is_valid())
                .map(|c| self.get_piece_by_coord(c))
                .flatten()
                .for_each(|p| {
                    if p.kind == piece && p.player == by_player {
                        results.push(*p)
                    }
                })
        });

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

    pub fn arbiter(&self, ply: &Ply) -> bool {
        // Verifies if there's a piece in square, and if it's the right turn
        if let Some(piece) = self.get_piece_by_coord(ply.origin) {
            if piece.player != self.turn {
                return false;
            }
        }
        self.new_get_legal_moves(ply.origin).contains(ply)
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
