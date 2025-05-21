use std::i32;
use std::cmp::{min, max};

use crate::bitboard::{BitBoard, Piece};

fn heuristic(board: &BitBoard, piece: Piece) -> i32 {
    let bitboard = match piece {
        Piece::Player => board.player_mask,
        Piece::AI => board.ai_mask,
        Piece::Empty => return 0,
    };

    let mid = board.cols as i32 / 2;
    let mut score = 0i32;

    for c in 0..board.cols {
        let col_start = c * (board.rows + 1);
        let mask = ((1u128 << board.rows) - 1) << col_start;
        let count = (bitboard & mask).count_ones() as i32;

        // Gradient weight: max in center, decreasing toward edges
        let weight = (mid - c as i32).abs() * - 1 + mid;

        score += weight * count;
    }

    score

}

pub fn minimax(
    board: &BitBoard,
    depth: u8,
    alpha: i32,
    beta: i32,
    is_maximizing: bool
) -> (Option<u8>, i32) {
    if depth == 0 {
        let score = heuristic(board, Piece::AI);
        return (None, score);
    }

    // Then check terminal states
    if board.check_win(Piece::AI) {
        return (None, 100_000_000);
    } else if board.check_win(Piece::Player) {
        return (None, -100_000_000);
    } else if board.is_full() {
        return (None, 0);
    }

    let valid_moves = board.get_valid_locations();

    if is_maximizing {
        // ai turn - maximizing player
        let mut best_score = i32::MIN;
        let mut best_col = None;
        let mut alpha = alpha;

        for col in valid_moves {
            let new_board = board.drop_piece(col, Piece::AI).unwrap();
            let (_, score) = minimax(&new_board, depth - 1, alpha, beta, false);

            if score > best_score {
                best_score = score;
                best_col = Some(col);
            }

            alpha = max(alpha, score);

            if alpha >= beta {
                break;
            }
        }

        (best_col, best_score)
    } else {
        // player turn - minimizing player
        let mut best_score = i32::MAX;
        let mut best_col = None;
        let mut beta = beta;

        for col in valid_moves {
            let new_board = board.drop_piece(col, Piece::Player).unwrap();
            let (_, score) = minimax(&new_board, depth - 1, alpha, beta, true);

            if score < best_score {
                best_score = score;
                best_col = Some(col);
            }            

            beta = min(beta, score);

            if alpha >= beta {
                break;
            }
        }

        (best_col, best_score)
    }
}