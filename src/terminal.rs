use std::io;
use crate::minimax::minimax;
use crate::bitboard::{BitBoard, Piece};

#[derive(PartialEq)]
pub enum Mode {
    Ui,
    Terminal,
}

pub fn game_mode_settings_input() -> Mode {
    let mut mode = Mode::Ui;

    loop {
        println!("Play in window (y, n - terminal, default y): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed == "y" {
            mode = Mode::Ui;
            break;
        } else if trimmed == "n" {
            mode = Mode::Terminal;
            break;
        } else if trimmed.is_empty() {
            break;
        } else {
            println!("Chose y or n.");
        }
    }

    mode
}

pub fn get_player_settings_input() -> (u8, u8, u8) {
    let mut row_count = 6;
    let mut column_count = 7;
    let mut win_sequence = 4;

    // get rows input
    loop {
        println!("Enter number of rows (2-20, default 6): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse() {
            if n >= 2 && n <= 20 {
                row_count = n;
                break;
            }
            println!("Rows must be at least 3");
        } else if input.trim().is_empty() {
            break;
        } else {
            println!("Invalid input");
        }
    }

    // get columns input
    let max_col = get_conjugate_value(row_count);
    loop {
        println!("Enter number of columns (2-{}, default 7): ", {max_col});
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse() {
            if n >= 2 && n <= max_col && n <= max_col {
                column_count = n;
                break;
            }
            println!("Columns must be at least 3");
        } else if input.trim().is_empty() {
            break;
        } else {
            println!("Invalid input");
        }
    }

    // get winning sequence input
    let max_win = row_count.min(column_count);
    loop {
        println!("Enter the required winning sequence (2-{}, default 4): ", {max_win});
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse() {
            if n >= 2 && n <= max_win {
                win_sequence = n;
                break;
            }
            println!("Win sequence length must be at least 3.");
        } else if input.trim().is_empty() {
            break;
        } else {
            println!("Invalid input");
        }
    }

    (row_count, column_count, win_sequence)
}

pub fn get_player_column_input(size: u8) -> u8 {
    loop {
        println!("Enter column number (0-{}): ", {size-1});
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(col) if col < size => return col,
            _ => println!("Invalid column."),
        }
    }
}

pub fn difficulty_input() -> u8 {
    let mut difficulty = 10;

    // get rows input
    loop {
        println!("Enter the difficulty (easiest 1 <-> 10 hardest, default 10): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse() {
            if is_valid_difficulty(n) {
                difficulty = n;
                break;
            }
            println!("Rows must be at least 3");
        } else if input.trim().is_empty() {
            break;
        } else {
            println!("Invalid input");
        }
    }

    difficulty
}

pub fn first_player_input() -> bool {
    let mut player_starts = true;

    loop {
        println!("Start first? (y - yes, n - AI starts first, default y): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed == "y" {
            player_starts = true;
            break;
        } else if trimmed == "n" {
            player_starts = false;
            break;
        } else if trimmed.is_empty() {
            break;
        } else {
            println!("Chose y or n.");
        }
    }

    player_starts
}

pub fn main_loop_terminal() {
    let settings = get_player_settings_input();
    let difficulty = difficulty_input(); // user inputs difficulty
    let player_starts = first_player_input();
    let mut board = BitBoard::new(settings.0, settings.1, settings.2);
    println!("{}", board);

    let mut current = match player_starts {
        true => Piece::Player,
        false => Piece::AI,
    };

    loop {
        let col: u8;
        if current == Piece::Player {
            col = get_player_column_input(settings.1);
        } else {
            // AI move using minimax
            println!("AI is thinking...");
            col = match minimax(&board, difficulty, i32::MIN, i32::MAX, true).0 {
                Some(c) => c,
                None => {
                    println!("No valid moves for AI!");
                    break;
                }
            }; // adjust depth if needed
            println!("AI chooses column: {}", col);
        }

        // Try to apply move
        if let Some(new_board) = board.drop_piece(col, current) {
            board = new_board;
            println!("{}", board);

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

const MAX_DIFFICULTY: u8 = 20;
const MIN_DIFFICULTY: u8 = 1;

fn is_valid_difficulty(difficulty: u8) -> bool {
    difficulty >= MIN_DIFFICULTY && difficulty <= MAX_DIFFICULTY
}

// board area must be < 128 so that it fits in one number (with paddings between columns)
fn get_conjugate_value(x: u8) -> u8 {
    (128/(x+1))-1
}