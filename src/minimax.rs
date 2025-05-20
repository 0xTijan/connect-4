use crate::bitboard::{BitBoard, Piece};


pub fn best_move(board: &BitBoard, depth: u8) -> u8 {
    // for each possible move simulate minimax
    // track and return the best move
    let valid_moves = board.get_valid_locations();
    let mut best_score = i32::MIN;
    let mut best_move = 0;

    for &col in &valid_moves {
        let new_board = board.drop_piece(col, Piece::AI).unwrap();
        let score = minimax(&new_board, depth, i32::MIN, i32::MAX, Piece::Player);
        if score > best_score {
            best_score = score;
            best_move = col;
        }
        println!("Score for column {}: {}", col, score);
    }

    best_move
}

fn minimax(board: &BitBoard, depth: u8, mut alpha: i32, mut beta: i32, player: Piece) -> i32 {
    // if game is over or depth is 0 - return heuristic of board
    if  
        depth == 0 ||
        board.is_full() ||
        board.check_win(Piece::Player) ||
        board.check_win(Piece::AI)
    {
        return heuristic(board);
    }

    let valid_moves = board.get_valid_locations();

    if player == Piece::AI {
        // maximizing player - ai
        let mut best_score = i32::MIN;

        for &col in &valid_moves {
            let new_board = board.drop_piece(col, Piece::AI).unwrap();
            let score = minimax(&new_board, depth - 1, alpha, beta, Piece::Player);
            best_score = best_score.max(score);
            alpha = alpha.max(score);
            if beta <= alpha {
                break; // beta cut-off
            }
        }

        return best_score;
    } else {
        // minimizing player - player
        let mut best_score = i32::MAX;

        for &col in &valid_moves {
            let new_board = board.drop_piece(col, Piece::AI).unwrap();
            let score = minimax(&new_board, depth - 1, alpha, beta, Piece::AI);
            best_score = best_score.min(score);
            beta = beta.min(score);
            if beta <= alpha {
                break; // alpha cut-off
            }
        }

        return best_score;
    }
}

fn heuristic(board: &BitBoard) -> i32 {
    if board.check_win(Piece::AI) {
        return 1_000_000;
    } else if board.check_win(Piece::Player) {
        return -1_000_000;
    } else {
        return 0;
    }
}