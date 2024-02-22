#![allow(dead_code)]

mod board;
mod coord;
mod game;
mod perft;
mod piece;
mod player;
mod ply;
mod status;

fn main() {
    // game::Game::new_from_fen("1r2kr2/pp1p1p2/2p4p/6pP/P1PP4/1P6/5PP1/R3K2R w KQ g6 0 21").play();
    // game::Game::new_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b K- e3 0 1").play();

    let mut game = game::Game::new();
    // game.play();

    println!("nodes: {}", perft::perft(*game.states.last().unwrap(), 5));
}
