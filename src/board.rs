use crate::{
    coord::Coord,
    piece::{Kind, Piece},
    player::Player,
    ply::Ply,
    status::Status,
};
use auto_enums::auto_enum;

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

    #[auto_enum]
    fn get_pseudo_legal_moves<'a>(&'a self, coord: Coord) -> impl Iterator<Item = Ply> + 'a {
        #[auto_enum(Iterator)]
        if let Some(piece_in_square) = self.get_piece_by_coord(coord) {
            #[auto_enum(Iterator)]
            match piece_in_square.kind {
                Kind::Pawn => self
                    .get_pawn_moves(coord)
                    .chain(self.get_pawn_captures(coord))
                    .chain(self.get_pawn_en_passant(coord).into_iter()),
                Kind::Knight => self.get_knight_moves(coord),
                Kind::Bishop => {
                    self.get_queen_rook_bishop_moves(coord, Coord::LIST_DIAGONAL.into_iter())
                }

                Kind::Rook => {
                    self.get_queen_rook_bishop_moves(coord, Coord::LIST_CARDINAL.into_iter())
                }

                Kind::Queen => self
                    .get_queen_rook_bishop_moves(coord, Coord::LIST_CARDINAL_DIAGONAL.into_iter()),
                Kind::King => self.get_king_moves(coord).chain(self.get_castling_moves()),
            }
        } else {
            std::iter::empty()
        }
    }

    #[auto_enum(Iterator)]
    pub fn get_legal_moves<'a>(&'a self, coord: Coord) -> impl Iterator<Item = Ply> + 'a {
        if let Some(piece_in_square) = self.get_piece_by_coord(coord) {
            let king_loc = match self.turn {
                Player::Black => self.black_king_loc,
                Player::White => self.white_king_loc,
            };

            let checking_pieces = self.square_attacked_by_pieces(king_loc, self.turn.opponent());

            if checking_pieces.len() == 1 {
                match piece_in_square.kind {
                    Kind::King => {
                        Box::new(self.get_king_moves(coord)) as Box<dyn Iterator<Item = Ply>>
                    }

                    _ => {
                        if let Some(_) = self.is_piece_pinned(*piece_in_square) {
                            return std::iter::empty();
                        }

                        match checking_pieces[0].kind {
                            Kind::Pawn => {
                                if let Some(en_passant_square) = self.en_passant_square {
                                    if checking_pieces[0].coord
                                        == en_passant_square
                                            + checking_pieces[0].player.advancing_direction()
                                        && piece_in_square.kind == Kind::Pawn
                                    {
                                        return self.get_pseudo_legal_moves(coord).filter(
                                            move |ply| {
                                                ply.destination == checking_pieces[0].coord
                                                    || ply.destination == en_passant_square
                                            },
                                        );
                                    }
                                }

                                Box::new(
                                    self.get_pseudo_legal_moves(coord).filter(move |ply| {
                                        ply.destination == checking_pieces[0].coord
                                    }),
                                ) as Box<dyn Iterator<Item = Ply>>
                            }
                            Kind::Knight => Box::new(
                                self.get_pseudo_legal_moves(coord)
                                    .filter(move |ply| ply.destination == checking_pieces[0].coord),
                            )
                                as Box<dyn Iterator<Item = Ply>>,
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

                                Box::new(self.get_pseudo_legal_moves(coord).filter(move |ply| {
                                    possible_destinations.contains(&ply.destination)
                                })) as Box<dyn Iterator<Item = Ply>>
                            }
                            Kind::King => panic!("King cannot be the checking piece"),
                        }
                    }
                }
            } else if checking_pieces.len() == 2 {
                match piece_in_square.kind {
                    Kind::King => {
                        Box::new(self.get_king_moves(coord)) as Box<dyn Iterator<Item = Ply>>
                    }
                    _ => Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Ply>>,
                }
            } else {
                if let Some(pin_direction) = self.is_piece_pinned(*piece_in_square) {
                    match piece_in_square.kind {
                        Kind::Pawn => Box::new(
                            self.get_pawn_moves(coord)
                                .chain(self.get_pawn_captures(coord))
                                .chain(self.get_pawn_en_passant(coord))
                                // TODO: refactor using pseudo_legal_moves
                                .filter(move |ply| {
                                    ply.destination == piece_in_square.coord + pin_direction
                                        || ply.destination
                                            == piece_in_square.coord + 2 * pin_direction
                                }),
                        ) as Box<dyn Iterator<Item = Ply>>,
                        Kind::Knight => {
                            Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Ply>>
                        }
                        Kind::Rook => Box::new(self.get_queen_rook_bishop_moves(
                            coord,
                            Coord::LIST_CARDINAL.into_iter().filter(move |&dir| {
                                dir == pin_direction || dir == -1 * pin_direction
                            }),
                        )) as Box<dyn Iterator<Item = Ply>>,
                        Kind::Bishop => Box::new(self.get_queen_rook_bishop_moves(
                            coord,
                            Coord::LIST_DIAGONAL.into_iter().filter(move |&dir| {
                                dir == pin_direction || dir == -1 * pin_direction
                            }),
                        )) as Box<dyn Iterator<Item = Ply>>,
                        Kind::Queen => Box::new(
                            self.get_queen_rook_bishop_moves(
                                coord,
                                Coord::LIST_CARDINAL_DIAGONAL
                                    .into_iter()
                                    .filter(move |&dir| {
                                        dir == pin_direction || dir == -1 * pin_direction
                                    }),
                            ),
                        ) as Box<dyn Iterator<Item = Ply>>,
                        Kind::King => unreachable!(),
                    }
                } else {
                    if piece_in_square.kind == Kind::Pawn
                        && self.is_pawn_enpassant_pinned(*piece_in_square)
                    {
                        return self
                            .get_pawn_moves(coord)
                            .chain(self.get_pawn_captures(coord));
                    }

                    Box::new(self.get_pseudo_legal_moves(coord)) as Box<dyn Iterator<Item = Ply>>
                }
            }
        } else {
            std::iter::empty()
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

    fn is_pawn_enpassant_pinned(&self, p: Piece) -> bool {
        if p.kind != Kind::Pawn {
            return false;
        }

        if let Some(en_passant_square) = self.en_passant_square {
            let king_loc = match p.player {
                Player::Black => self.black_king_loc,
                Player::White => self.white_king_loc,
            };

            let dir = Coord::find_dir_between_coords(p.coord, king_loc);

            for i in 1..8 {
                let pos = king_loc + i * dir;
                if pos == en_passant_square + p.player.opponent().advancing_direction() {
                    continue;
                }
                if !pos.is_valid() || (self.is_square_occupied(pos) && pos != p.coord) {
                    return false;
                };
                if pos == p.coord {
                    for j in 1..8 {
                        let pos_after_piece = p.coord + j * dir;
                        if pos_after_piece
                            == en_passant_square + p.player.opponent().advancing_direction()
                        {
                            continue;
                        }
                        if !pos_after_piece.is_valid() {
                            return false;
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
                                return false;
                            }

                            if piece.kind == pinning_piece || piece.kind == Kind::Queen {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn find_king(&self, player: Player) -> Coord {
        match player {
            Player::Black => self.black_king_loc,
            Player::White => self.white_king_loc,
        }
    }

    // Gives all posible moves in a position
    pub fn get_all_moves<'a>(&'a self) -> impl Iterator<Item = Ply> + 'a {
        match self.turn {
            Player::Black => &self.black_pieces,
            Player::White => &self.white_pieces,
        }
        .into_iter()
        .flat_map(|p| self.get_legal_moves(p.coord))
    }

    fn get_king_moves<'a>(&'a self, origin: Coord) -> impl Iterator<Item = Ply> + 'a {
        let player = self.player_at_square(origin).unwrap();

        Coord::LIST_CARDINAL_DIAGONAL
            .iter()
            .map(move |&delta| origin + delta)
            .filter(move |&pos| {
                pos.is_valid()
                    && self.player_at_square(pos) != Some(player)
                    && !self.is_square_attacked(pos, player.opponent())
            })
            .map(move |pos| Ply {
                origin,
                destination: pos,
                promotion: None,
            })
    }

    fn get_knight_moves<'a>(&'a self, origin: Coord) -> impl Iterator<Item = Ply> + 'a {
        let player = self.player_at_square(origin).unwrap();

        Coord::LIST_KNIGHT
            .iter()
            .map(move |&delta| origin + delta)
            .filter(move |&pos| pos.is_valid() && self.player_at_square(pos) != Some(player))
            .map(move |pos| Ply {
                origin,
                destination: pos,
                promotion: None,
            })
    }

    fn get_queen_rook_bishop_moves<'a>(
        &'a self,
        origin: Coord,
        directions: impl Iterator<Item = Coord> + 'a,
    ) -> impl Iterator<Item = Ply> + 'a {
        let player = self.player_at_square(origin).unwrap();

        directions
            .map(move |dir| {
                (1..)
                    .map(move |i| origin + dir * i)
                    .take_while(|&c| c.is_valid())
                    .take_while(move |&c| self.player_at_square(c) != Some(player))
                    .take_while(move |&c| self.player_at_square(c - dir) != Some(player.opponent()))
                    .map(move |c| Ply {
                        origin,
                        destination: c,
                        promotion: None,
                    })
            })
            .flatten()
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

    #[auto_enum(Iterator)]
    fn get_pawn_moves<'a>(&'a self, origin: Coord) -> impl Iterator<Item = Ply> + 'a {
        let player = self.player_at_square(origin).unwrap();
        let dir = player.advancing_direction();
        let destination = origin + dir;

        if self.is_square_occupied(destination) {
            std::iter::empty()
        } else {
            #[auto_enum(Iterator)]
            let iter = if destination.row == 0 || destination.row == 7 {
                Kind::PROMOTIONS.iter().map(move |&promo| Ply {
                    origin,
                    destination,
                    promotion: Some(promo),
                })
            } else {
                std::iter::once(Ply {
                    origin,
                    destination,
                    promotion: None,
                })
            };

            #[auto_enum(Iterator)]
            let iter =
                if origin.row == player.pawn_row() && !self.is_square_occupied(destination + dir) {
                    iter.chain(std::iter::once(Ply {
                        origin,
                        destination: destination + dir,
                        promotion: None,
                    }))
                } else {
                    iter
                };

            iter
        }
    }

    #[auto_enum]
    fn get_pawn_captures<'a>(&'a self, origin: Coord) -> impl Iterator<Item = Ply> + 'a {
        let player = self.player_at_square(origin).unwrap();

        [Coord::L, Coord::R]
            .iter()
            .map(move |&c| origin + c + player.advancing_direction())
            .filter(move |&pos| {
                pos.is_valid() && self.player_at_square(pos) == Some(player.opponent())
            })
            .map(move |pos| {
                #[auto_enum(Iterator)]
                if pos.row == 7 || pos.row == 0 {
                    Kind::PROMOTIONS.iter().map(move |&promo| Ply {
                        origin,
                        destination: pos,
                        promotion: Some(promo),
                    })
                } else {
                    std::iter::once(Ply {
                        origin,
                        destination: pos,
                        promotion: None,
                    })
                }
            })
            .flatten()
    }

    fn get_pawn_en_passant(&self, origin: Coord) -> Option<Ply> {
        let player = self.player_at_square(origin).unwrap();

        let Some(en_passant_square) = self.en_passant_square else {
            return None;
        };

        if (origin + player.advancing_direction()).row != en_passant_square.row {
            return None;
        }

        if (origin.col + 1 == en_passant_square.col) || (origin.col - 1 == en_passant_square.col) {
            Some(Ply {
                origin,
                destination: en_passant_square,
                promotion: None,
            })
        } else {
            None
        }
    }

    pub fn is_square_attacked(&self, origin: Coord, by_player: Player) -> bool {
        let king_loc = match by_player {
            Player::Black => self.white_king_loc,
            Player::White => self.black_king_loc,
        };

        Coord::LIST_CARDINAL
            .iter()
            .map(|c| (c, Kind::Rook))
            .chain(Coord::LIST_DIAGONAL.iter().map(|c| (c, Kind::Bishop)))
            .any(|(&dir, kind)| {
                (1..)
                    .map(move |i| origin + dir * i)
                    .take_while(|&c| c.is_valid())
                    .take_while(|&c| {
                        self.player_at_square(c) != Some(by_player.opponent()) || c == king_loc
                    })
                    .take_while(move |&c| {
                        self.player_at_square(c - dir) != Some(by_player) || c - dir == origin
                    })
                    .filter(|&c| self.player_at_square(c) == Some(by_player))
                    .any(|c| {
                        self.kind_at_square(c) == Some(kind)
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

    // TODO: iteratoooooor!
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

    fn get_castling_moves<'a>(&'a self) -> impl Iterator<Item = Ply> + 'a {
        let player = self.turn;
        let row = player.home_row();
        let permissions = match player {
            Player::White => (self.white_can_oo, self.white_can_ooo),
            Player::Black => (self.black_can_oo, self.black_can_ooo),
        };

        (0..2).filter_map(move |i| match i {
            0 if permissions.0
                && !self.is_square_attacked(Coord { row, col: 4 }, player.opponent())
                && !self.is_square_attacked(Coord { row, col: 5 }, player.opponent())
                && !self.is_square_attacked(Coord { row, col: 6 }, player.opponent())
                && !self.is_square_occupied(Coord { row, col: 5 })
                && !self.is_square_occupied(Coord { row, col: 6 }) =>
            {
                Some(Ply {
                    origin: Coord { row, col: 4 },
                    destination: Coord { row, col: 6 },
                    promotion: None,
                })
            }

            1 if permissions.1
                && !self.is_square_attacked(Coord { row, col: 4 }, player.opponent())
                && !self.is_square_attacked(Coord { row, col: 2 }, player.opponent())
                && !self.is_square_attacked(Coord { row, col: 3 }, player.opponent())
                && !self.is_square_occupied(Coord { row, col: 1 })
                && !self.is_square_occupied(Coord { row, col: 2 })
                && !self.is_square_occupied(Coord { row, col: 3 }) =>
            {
                Some(Ply {
                    origin: Coord { row, col: 4 },
                    destination: Coord { row, col: 2 },
                    promotion: None,
                })
            }

            _ => None,
        })
    }

    pub fn arbiter(&self, ply: &Ply) -> bool {
        // Verifies if there's a piece in square, and if it's the right turn
        if let Some(piece) = self.get_piece_by_coord(ply.origin) {
            if piece.player != self.turn {
                return false;
            }
        }
        self.get_legal_moves(ply.origin).any(|x| &x == ply)
    }

    pub fn verify_status(&self) -> Status {
        let king_pos = self.find_king(self.turn);

        if self.half_move_clock == 100 {
            return Status::Draw;
        }

        if self.get_all_moves().next().is_some() {
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
