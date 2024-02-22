use crate::{board::Board, coord::Coord, piece::Kind, ply::Ply};
use std::io;

#[derive(Debug, Clone)]
pub struct Game {
    pub states: Vec<Board>,
}

impl Game {
    pub fn new() -> Game {
        Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn new_from_fen(fen: &str) -> Game {
        Game {
            states: vec![Board::new_from_fen(fen)],
        }
    }

    pub fn play(&mut self) {
        loop {
            let current_pos = self.states.last().unwrap();

            self.states.last().unwrap().print_board(current_pos.turn);

            let Some(ply) = self.get_user_move() else {
                println!("Invalid input text.");
                continue;
            };

            if !current_pos.arbiter(&ply) {
                println!("That move is not allowed, idiot.");
                continue;
            }

            self.states.push(current_pos.make_move(ply));
            let next_pos = self.states.last().unwrap();

            if self.verify_threefold_repetition() {
                next_pos.print_board(next_pos.turn);
                println!("Draw");
                break;
            }

            match dbg!(next_pos.verify_status()) {
                crate::status::Status::Invalid => {
                    next_pos.print_board(next_pos.turn);
                    println!("Invalid position");
                    break;
                }
                crate::status::Status::BWin => {
                    next_pos.print_board(next_pos.turn);
                    println!("Checkmate: black wins");
                    break;
                }
                crate::status::Status::WWin => {
                    next_pos.print_board(next_pos.turn);
                    println!("Checkmate: white wins");
                    break;
                }
                crate::status::Status::Draw => {
                    next_pos.print_board(next_pos.turn);
                    println!("Draw");
                    break;
                }
                crate::status::Status::Ongoing => continue,
            }
        }
    }

    fn get_user_move(&self) -> Option<Ply> {
        println!("Move?");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parse_coord = |input: &str| -> Option<Coord> {
            let mut iter = input.trim().chars();
            let c1 = iter.next()?;
            let c2 = iter.next()? as i32 - ('0' as i32);

            if iter.next().is_none() {
                Board::notation_conversion(c1, c2)
            } else {
                None
            }
        };

        let parse_promotion = |input: &str| -> Option<Kind> {
            match input.trim() {
                "q" => Some(Kind::Queen),
                "r" => Some(Kind::Rook),
                "b" => Some(Kind::Bishop),
                "n" | "k" => Some(Kind::Knight),
                _ => None,
            }
        };

        let items = input.trim().split(" ").collect::<Vec<&str>>();

        Some(Ply {
            origin: parse_coord(items.get(0)?)?,
            destination: parse_coord(items.get(1)?)?,
            promotion: items.get(2).and_then(|&s| parse_promotion(s)),
        })
    }

    fn verify_threefold_repetition(&self) -> bool {
        let current_state = match self.states.last() {
            Some(state) => state,
            None => return false,
        };

        self.states
            .iter()
            .rev()
            .step_by(2)
            .filter(|&x| x == current_state)
            .count()
            >= 3
    }
}
