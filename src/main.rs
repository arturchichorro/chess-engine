#![allow(dead_code)]

mod board;
mod coord;
mod game;
mod piece;
mod player;
mod ply;

fn main() {
    // game::Game::new_from_fen("1r2kr2/pp1p1p2/2p4p/6pP/P1PP4/1P6/5PP1/R3K2R w KQ g6 0 21").play();
    game::Game::new().play();
}
