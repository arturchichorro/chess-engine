use crate::{
    board::Board,
    coord::Coord,
    piece::{Kind, Piece},
    player::Player,
    ply::Ply,
    status::Status,
};

pub fn search(board: Board, depth: i32) -> i32 {
    let move_vec: Vec<Ply> = board.get_all_moves();

    if depth == 0 {
        return evaluate(board);
    }

    if move_vec.is_empty() {
        match board.turn {
            Player::Black => {
                if board.is_square_attacked(board.black_king_loc, board.turn.opponent()) {
                    return -1000000000;
                }
            }
            Player::White => {
                if board.is_square_attacked(board.white_king_loc, board.turn.opponent()) {
                    return -1000000000;
                }
            }
        }
        return 0;
    }

    let mut best_evaluation = -1000000000;

    for i in 0..move_vec.len() {
        let new_board_state = board.make_move(move_vec[i]);
        let evaluation = -search(new_board_state, depth - 1);

        best_evaluation = best_evaluation.max(evaluation);
    }
    // Need to find a way to return move
    best_evaluation
}

pub fn evaluate(board: Board) -> i32 {
    let white_eval = count_material(board, Player::White);
    let black_eval = count_material(board, Player::Black);

    let mult = match board.turn {
        Player::Black => -1,
        Player::White => 1,
    };

    (white_eval - black_eval) * mult
}

fn count_material(board: Board, player: Player) -> i32 {
    let mut result: i32 = 0;

    match player {
        Player::Black => {
            for p in board.black_pieces {
                if let Some(piece) = p {
                    match piece.kind {
                        Kind::Pawn => result += Kind::PAWN_VALUE,
                        Kind::Rook => result += Kind::ROOK_VALUE,
                        Kind::Knight => result += Kind::KNIGHT_VALUE,
                        Kind::Bishop => result += Kind::BISHOP_VALUE,
                        Kind::Queen => result += Kind::QUEEN_VALUE,
                        Kind::King => {}
                    }
                }
            }
        }
        Player::White => {
            for p in board.white_pieces {
                if let Some(piece) = p {
                    match piece.kind {
                        Kind::Pawn => result += Kind::PAWN_VALUE,
                        Kind::Rook => result += Kind::ROOK_VALUE,
                        Kind::Knight => result += Kind::KNIGHT_VALUE,
                        Kind::Bishop => result += Kind::BISHOP_VALUE,
                        Kind::Queen => result += Kind::QUEEN_VALUE,
                        Kind::King => {}
                    }
                }
            }
        }
    }

    result
}
