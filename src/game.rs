use std::io;

use crate::{board::Board, ply::Ply};

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

        // Test Fen
        // Board::new_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b K- e3 0 1")
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
}
