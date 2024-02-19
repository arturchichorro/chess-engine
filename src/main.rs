use std::fmt;
use std::fmt::Write;
use std::io;

#[derive(Debug, Copy, Clone)]
enum Player {
    Black,
    White,
}

impl Player {
    fn opposite(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
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

#[derive(Debug, Copy, Clone)]
struct Piece {
    piece: PieceType,
    player: Player,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.piece.character(self.player))
    }
}

#[derive(Debug, Copy, Clone)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
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

#[derive(Debug, Copy, Clone)]
struct Ply {
    origin: (u8, u8),
    destination: (u8, u8),
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
    en_passant_square: Option<(u8, u8)>,
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
            let row_en_passant = (x % 8) as u8;
            let col_en_passant = (x / 8) as u8;
            (row_en_passant, col_en_passant)
        });

        board_from_fen
            .pieces
            .into_iter()
            .enumerate()
            .map(|(i, p)| p.map(|x| (i, x)))
            .flatten()
            .for_each(|(i, p)| {
                let row = (i % 8) as u8;
                let col = (i / 8) as u8;
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

                *board.square_idx(col, row) = Some(Piece { piece, player })
            });

        board
    }

    fn notation_conversion(row: char, column: u8) -> Option<(u8, u8)> {
        if (row as u8) >= ('a' as u8) && (row as u8) <= ('h' as u8) && column >= 1 && column <= 8 {
            Some(((row as u8) - ('a' as u8), column - 1))
        } else {
            None
        }
    }
    fn square_idx(&mut self, column: u8, row: u8) -> &mut Option<Piece> {
        &mut self.board[column as usize][row as usize]
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
    fn arbiter(&self, ply: Ply, promotes: Option<PieceType>) -> bool {
        true
        // Ideia: get_legal_moves, função que olha para uma posição e turno e devolve lista de jogadas legais
    }

    fn make_move(&self, ply: Ply, promotes: Option<PieceType>) -> Board {
        let mut new_game_state = self.clone();

        *new_game_state.square_idx(ply.destination.1, ply.destination.0) =
            *new_game_state.square_idx(ply.origin.1, ply.origin.0);
        *new_game_state.square_idx(ply.origin.1, ply.origin.0) = None;

        new_game_state.turn = new_game_state.turn.opposite();
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
            .map(|s| -> Option<(u8, u8)> {
                let mut iter = s.chars();
                let c1 = iter.next()?;
                let c2 = iter.next()? as u8 - ('0' as u8);

                if iter.next().is_none() {
                    Board::notation_conversion(c1, c2)
                } else {
                    None
                }
            })
            .collect::<Option<Vec<_>>>()
            .filter(|v| v.len() == 2)
            .map(|v| Ply {
                origin: v[0],
                destination: v[1],
            })
    }

    fn play(&mut self) {
        loop {
            self.states
                .last()
                .unwrap()
                .print_board(self.states.last().unwrap().turn);

            let Some(coords) = self.get_user_move() else {
                println!("Invalid input text.");
                continue;
            };

            if !self.states.last().unwrap().arbiter(coords, None) {
                println!("That move is not allowed, idiot.");
                continue;
            }

            self.states
                .push(self.states.last().unwrap().make_move(coords, None));
        }
    }
}

fn main() {
    let mut game = Game::new();

    println!("{:?}", game);
    game.play();
}
