mod bitboard;
mod terminal;
mod minimax;

use minimax::{best_move};
use bitboard::{BitBoard, Piece};

use std::io;

fn main() {
    let size = terminal::size_input(); // user inputs board size
    let mut board = BitBoard::new(size.0, size.1, 5);
    board.print();

    let mut current = Piece::Player;

    loop {
        let col: u8;

        if current == Piece::Player {
            // Ask player for column input
            println!("Your turn. Choose a column");

            col = loop {
                let mut input = String::new();
                if io::stdin().read_line(&mut input).is_err() {
                    println!("Error reading input. Try again.");
                    continue;
                }

                match input.trim().parse::<u8>() {
                    Ok(c) if board.get_valid_locations().contains(&c) => break c,
                    _ => println!("Invalid column. Try again:"),
                }
            };
        } else {
            // AI move using minimax
            println!("AI is thinking...");
            col = best_move(&board); // adjust depth if needed
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
