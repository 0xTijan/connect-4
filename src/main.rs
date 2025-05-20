mod bitboard;
mod terminal;
mod minimax;

use minimax::{best_move};
use bitboard::{BitBoard, Piece};

use std::io;

fn main() {
    let settings = terminal::get_player_settings_input();
    let difficulty = terminal::difficulty_input(); // user inputs difficulty
    let mut board = BitBoard::new(settings.0, settings.1, settings.2);
    board.print();

    let mut current = Piece::Player;

    loop {
        let col: u8;

        if current == Piece::Player {
            // Ask player for column input
            col = terminal::get_player_column_input(settings.1);
        } else {
            // AI move using minimax
            println!("AI is thinking...");
            col = best_move(&board, difficulty); // adjust depth if needed
            println!("AI chooses column: {}", col);
        }

        // Try to apply move
        if let Some(new_board) = board.drop_piece(col, current) {
            board = new_board;
            board.print();

            if board.check_win(current) {
                println!("{:?} wins!", current);
                break;
            }

            if board.is_full() {
                println!("It's a draw!");
                break;
            }

            // Switch player
            current = if current == Piece::Player { Piece::AI } else { Piece::Player };
        } else {
            println!("Column {} is full. Try again.", col);
        }
    }
}
