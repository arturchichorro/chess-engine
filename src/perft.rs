use crate::{board::Board, game::Game, ply::Ply};

// Performance tests
pub fn perft(board: Board, depth: i32) -> u64 {
    let move_vec: Vec<Ply> = board.get_all_moves();
    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    for i in 0..move_vec.len() {
        let new_board_state = board.make_move(move_vec[i]);
        nodes += perft(new_board_state, depth - 1);
    }
    nodes
}

//
pub fn perft_suite() -> () {
    let game_one = Game::new();
    println!(
        "Position 1, Depth 1 nodes: {}. Expected value: 20",
        perft(*game_one.states.last().unwrap(), 1)
    );
    println!(
        "Position 1, Depth 2 nodes: {}. Expected value: 400",
        perft(*game_one.states.last().unwrap(), 2)
    );
    println!(
        "Position 1, Depth 3 nodes: {}. Expected value: 8902",
        perft(*game_one.states.last().unwrap(), 3)
    );
    println!(
        "Position 1, Depth 4 nodes: {}. Expected value: 197281",
        perft(*game_one.states.last().unwrap(), 4)
    );
    println!(
        "Position 1, Depth 5 nodes: {}. Expected value: 4865609",
        perft(*game_one.states.last().unwrap(), 5)
    );
    let game_two =
        Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    println!(
        "Position 2, Depth 1 nodes: {}. Expected value: 48",
        perft(*game_two.states.last().unwrap(), 1)
    );
    println!(
        "Position 2, Depth 2 nodes: {}. Expected value: 2039",
        perft(*game_two.states.last().unwrap(), 2)
    );
    println!(
        "Position 2, Depth 3 nodes: {}. Expected value: 97862",
        perft(*game_two.states.last().unwrap(), 3)
    );
    println!(
        "Position 2, Depth 4 nodes: {}. Expected value: 4085603",
        perft(*game_two.states.last().unwrap(), 4)
    );
    let game_three = Game::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    println!(
        "Position 3, Depth 1 nodes: {}. Expected value: 14",
        perft(*game_three.states.last().unwrap(), 1)
    );
    println!(
        "Position 3, Depth 2 nodes: {}. Expected value: 191",
        perft(*game_three.states.last().unwrap(), 2)
    );
    println!(
        "Position 3, Depth 3 nodes: {}. Expected value: 2812",
        perft(*game_three.states.last().unwrap(), 3)
    );
    println!(
        "Position 3, Depth 4 nodes: {}. Expected value: 43238",
        perft(*game_three.states.last().unwrap(), 4)
    );
    println!(
        "Position 3, Depth 5 nodes: {}. Expected value: 674624",
        perft(*game_three.states.last().unwrap(), 5)
    );
    let game_four =
        Game::new_from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    println!(
        "Position 4, Depth 1 nodes: {}. Expected value: 6",
        perft(*game_four.states.last().unwrap(), 1)
    );
    println!(
        "Position 4, Depth 2 nodes: {}. Expected value: 264",
        perft(*game_four.states.last().unwrap(), 2)
    );
    println!(
        "Position 4, Depth 3 nodes: {}. Expected value: 9467",
        perft(*game_four.states.last().unwrap(), 3)
    );
    println!(
        "Position 4, Depth 4 nodes: {}. Expected value: 422333",
        perft(*game_four.states.last().unwrap(), 4)
    );
    let game_five = Game::new_from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    println!(
        "Position 5, Depth 1 nodes: {}. Expected value: 44",
        perft(*game_five.states.last().unwrap(), 1)
    );
    println!(
        "Position 5, Depth 2 nodes: {}. Expected value: 1486",
        perft(*game_five.states.last().unwrap(), 2)
    );
    println!(
        "Position 5, Depth 3 nodes: {}. Expected value: 62379",
        perft(*game_five.states.last().unwrap(), 3)
    );
    println!(
        "Position 5, Depth 4 nodes: {}. Expected value: 2103487",
        perft(*game_five.states.last().unwrap(), 4)
    );
    let game_six = Game::new_from_fen(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
    );
    println!(
        "Position 6, Depth 1 nodes: {}. Expected value: 46",
        perft(*game_six.states.last().unwrap(), 1)
    );
    println!(
        "Position 6, Depth 2 nodes: {}. Expected value: 2079",
        perft(*game_six.states.last().unwrap(), 2)
    );
    println!(
        "Position 6, Depth 3 nodes: {}. Expected value: 89890",
        perft(*game_six.states.last().unwrap(), 3)
    );
    println!(
        "Position 6, Depth 4 nodes: {}. Expected value: 3894594",
        perft(*game_six.states.last().unwrap(), 4)
    );
}
