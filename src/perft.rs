use crate::{board::Board, game::Game, ply::Ply};
use std::time::Instant;

// Performance tests
pub fn perft(board: &Board, depth: i32) -> u64 {
    let move_vec: Vec<Ply> = board.get_all_moves();
    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    Board::check_everything(board);

    for i in 0..move_vec.len() {
        let new_board_state = board.make_move(move_vec[i]);
        nodes += perft(&new_board_state, depth - 1);
    }
    nodes
}

pub fn perft_divider() {
    let depth = 4;
    let fen = "6n1/p1q1pP1P/3pkP2/P3P3/4KB2/2P3p1/7P/8 w - - 0 1";
    let game = Game::new_from_fen(fen);
    let board = game.states.last().unwrap();
    let mut result = 0;

    Board::check_everything(board);
    let move_vec = board.get_all_moves();
    move_vec.iter().for_each(|ply| {
        let new_board_state = board.make_move(*ply);

        let accum = perft(&new_board_state, depth - 1);

        result += accum;
        println!(
            "{}{} {}{} nodes: {}",
            Board::reverse_notation_conversion(ply.origin).0,
            Board::reverse_notation_conversion(ply.origin).1,
            Board::reverse_notation_conversion(ply.destination).0,
            Board::reverse_notation_conversion(ply.destination).1,
            accum
        );
        // println!("{:?}", new_board_state.get_all_moves());
    });

    println!("Total nodes: {}", result);
}

pub fn perft_one_pos() -> () {
    let fen = "4k3/7p/8/1pp1ppp1/pP1p1P2/8/P1P1P1PP/4K3 w - - 0 1";
    let checks = vec![(1, 6), (2, 264), (3, 9467), (4, 422333)];

    let game = Game::new_from_fen(fen);
    for (depth, value) in checks {
        let result = perft(game.states.last().unwrap(), depth);
        println!("Position {fen}, depth {depth}, result {result}, expected {value}");
        // assert_eq!(result, value);
    }
}

//
pub fn perft_suite() -> () {
    let data = [
        (
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            vec![(1, 20), (2, 400), (3, 8902), (4, 197281), (5, 4865609)],
        ),
        (
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            vec![(1, 48), (2, 2039), (3, 97862), (4, 4085603)],
        ),
        (
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
            vec![(1, 14), (2, 191), (3, 2812), (4, 43238), (5, 674624)],
        ),
        (
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            vec![(1, 6), (2, 264), (3, 9467), (4, 422333)],
        ),
        (
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            vec![(1, 44), (2, 1486), (3, 62379), (4, 2103487)],
        ),
        (
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            vec![(1, 46), (2, 2079), (3, 89890), (4, 3894594)],
        ),
    ];

    for (fen, checks) in data {
        let game = Game::new_from_fen(fen);
        for (depth, value) in checks {
            let start = Instant::now();
            let result = perft(game.states.last().unwrap(), depth);
            let duration = Instant::now().duration_since(start);
            println!("Position {fen}, depth {depth}, result {result}, expected {value}, took {duration:?}, speed {:.2}", result as f64 / duration.as_secs_f64());
            // assert_eq!(result, value);
        }
    }
}
