use std::i32;
use std::cmp::{min, max};
use std::collections::HashSet;

use crate::bitboard::{BitBoard, Piece};


fn evaluate_heuristic(board: &BitBoard, piece: Piece) -> i32 {
    let win_score = 1_000_000;
    let double_threat_bonus = 5000;

    let (player_mask, opponent_mask) = match piece {
        Piece::AI => (board.ai_mask, board.player_mask),
        Piece::Player => (board.player_mask, board.ai_mask),
        _ => return 0,
    };

    if board.check_win(piece) {
        return win_score;
    }
    let opponent_piece = match piece {
        Piece::AI => Piece::Player,
        Piece::Player => Piece::AI,
        _ => return 0,
    };
    if board.check_win(opponent_piece) {
        return -win_score;
    }

    let mut score = 0;

    for len in (2..board.connect).rev() {
        let weight = match len {
            l if l == board.connect - 1 => 1000,
            l if l == board.connect - 2 => 100,
            _ => 10,
        };

        let player_count = count_unblocked_sequences(board, player_mask, opponent_mask, len);
        let opponent_count = count_unblocked_sequences(board, opponent_mask, player_mask, len);

        score += weight * player_count as i32;
        score -= weight * opponent_count as i32;
    }

    let player_threats = find_threat_cells(board, player_mask, opponent_mask, board.connect - 1);
    let opponent_threats = find_threat_cells(board, opponent_mask, player_mask, board.connect - 1);

    if player_threats.len() >= 2 {
        score += double_threat_bonus;
    }
    if opponent_threats.len() >= 2 {
        score -= double_threat_bonus;
    }

    let center_score = center_preference(board, piece);

    score + center_score
}

fn count_unblocked_sequences(
    board: &BitBoard,
    player: u128,
    opponent: u128,
    target_len: u8,
) -> usize {
    let stride = board.rows + 1;
    let directions = [1, stride, stride - 1, stride + 1];
    let mut count = 0;

    for &dir in &directions {
        let mut m = player;
        for _ in 1..target_len {
            m &= m >> dir;
        }
        let mask = player | opponent;
        let potential1 = m >> dir;
        let potential2 = m << (dir as u32 * target_len as u32);
        let threats = (potential1 | potential2) & !mask;
        count += threats.count_ones() as usize;
    }

    count
}

fn find_threat_cells(
    board: &BitBoard,
    player: u128,
    opponent: u128,
    target_len: u8,
) -> HashSet<u8> {
    let stride = board.rows + 1;
    let directions = [1, stride, stride - 1, stride + 1];
    let mut threats = HashSet::new();
    let mask = player | opponent;

    for &dir in &directions {
        let mut m = player;
        for _ in 1..target_len {
            m &= m >> dir;
        }

        let end1 = m >> dir;
        let end2 = m << (dir as u32 * target_len as u32);
        let free = (end1 | end2) & !mask;

        for i in 0..128 {
            if (free >> i) & 1 != 0 {
                threats.insert(i as u8);
            }
        }
    }

    threats
}

fn center_preference(board: &BitBoard, piece: Piece) -> i32 {
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

        // Weight: highest at center, decreases linearly to edges
        let weight = mid - (mid - c as i32).abs();

        score += weight * count;
    }

    // Scale down to ensure it doesn't overpower threat evaluation
    score / 5
}

pub fn minimax(
    board: &BitBoard,
    depth: u8,
    alpha: i32,
    beta: i32,
    is_maximizing: bool
) -> (Option<u8>, i32) {
    let player = if is_maximizing { Piece::AI } else { Piece::Player };

    if board.check_win(Piece::AI) {
        return (None, 100_000_000);
    } else if board.check_win(Piece::Player) {
        return (None, -100_000_000);
    } else if board.is_full() {
        return (None, 0);
    }
    
    if depth == 0 {
        return (None, evaluate_heuristic(board, player));
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