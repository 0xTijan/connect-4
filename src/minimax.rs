use crate::bitboard::{BitBoard, Piece};

// Struct to store minimax results
pub struct MinimaxRes {
    pub score: i32,
    pub index: Option<i32>,
}

pub fn best_move(board: &BitBoard) -> u8 {
    let mut cloned_board = board.clone(); // Clone to preserve original board
    let result = minimax(&mut cloned_board, &Piece::AI, i32::MIN, i32::MAX);
    result.index.unwrap() as u8
}

fn minimax(board: &mut BitBoard, curr_player: &Piece, mut alpha: i32, mut beta: i32) -> MinimaxRes {
    let valid_moves = board.get_valid_locations();

    // Terminal state check
    if board.check_win(Piece::Player) {
        return MinimaxRes {
            score: -1,
            index: None,
        };
    } else if board.check_win(Piece::AI) {
        return MinimaxRes {
            score: 1,
            index: None,
        };
    } else if board.is_full() {
        return MinimaxRes {
            score: 0,
            index: None,
        };
    }

    let mut best_test_play = MinimaxRes {
        score: if *curr_player == Piece::AI { i32::MIN } else { i32::MAX },
        index: None,
    };

    for col in valid_moves {
        // Simulate move
        let mut new_board = board.drop_piece(col, *curr_player).unwrap();

        // Recursive minimax
        let next_player = if *curr_player == Piece::AI {
            Piece::Player
        } else {
            Piece::AI
        };

        let res = minimax(&mut new_board, &next_player, alpha, beta);

        // Reset board
        let current_move = MinimaxRes {
            score: res.score,
            index: Some(col as i32),
        };

        if *curr_player == Piece::AI {
            // Maximizing (AI)
            if current_move.score > best_test_play.score {
                best_test_play = current_move;
            }
            alpha = alpha.max(best_test_play.score);
        } else {
            // Minimizing (Human)
            if current_move.score < best_test_play.score {
                best_test_play = current_move;
            }
            beta = beta.min(best_test_play.score);
        }

        // Alpha-beta pruning
        if beta <= alpha {
            break;
        }
    }

    best_test_play
}

fn heuristic(board: &BitBoard, piece: &Piece) -> i32 {
    0
}