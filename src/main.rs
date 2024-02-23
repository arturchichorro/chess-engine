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
    std::env::set_var("RUST_BACKTRACE", "1");

    // game::Game::new_from_fen("1r2kr2/pp1p1p2/2p4p/6pP/P1PP4/1P6/5PP1/R3K2R w KQ g6 0 21").play();
    // game::Game::new_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b K- e3 0 1").play();
    // let game = game::Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",);
    // game.play();
    // let game_six = game::Game::new_from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",);
    // println!("{}", perft::perft(*game_six.states.last().unwrap(), 2));

    perft::perft_suite();
}
