use crate::{board::Board, coord::Coord, piece::Kind, ply::Ply};
use std::io;

#[derive(Debug, Clone)]
pub struct Game {
    states: Vec<Board>,
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

        dbg!(Some(Ply {
            origin: parse_coord(items.get(0)?)?,
            destination: parse_coord(items.get(1)?)?,
            promotion: items.get(2).and_then(|&s| parse_promotion(s)),
        }))
    }
}
