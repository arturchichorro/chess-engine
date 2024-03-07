#![allow(dead_code)]

use coord::Coord;
mod board;
mod coord;
mod engine;
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
    // let mut game = game::Game::new_from_fen(
    //     "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
    // );

    // game::Game::new_from_fen("6n1/p1qkp3/3p4/P3P3/5B2/2P1K3/7P/R7 w - - 0 1").play();

    // let mut game = game::Game::new();
    // game.play();
    // let game_six = game::Game::new_from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",);
    // println!("{}", perft::perft(*game_six.states.last().unwrap(), 2));

    let game = game::Game::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/4P3/6P1/8 b - - 0 1");
    let board = game.states.last().unwrap();
    // println!("{:?}", board.new_get_legal_moves(Coord { row: 1, col: 3 }));

    println!(
        "{:?}",
        board.is_piece_pinned(board.get_piece_by_coord(Coord { row: 3, col: 5 }).unwrap()),
    );

    // perft::perft_suite();
    // perft::perft_one_pos();
    perft::perft_divider();
}
