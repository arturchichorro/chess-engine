use crate::{board::Board, coord::Coord, piece::Kind, ply::Ply};

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
