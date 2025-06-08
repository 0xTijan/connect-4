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
        println!("Igraj v novem oknu (y, n - terminal, privzeto y): ");
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
            println!("Izberi y ali n.");
        }
    }

    mode
}

pub fn get_player_settings_input() -> (u8, u8, u8) {
    let mut row_count = 6;
    let mut column_count = 7;
    let mut win_sequence = 4;

    // dobi input za vrstice
    loop {
        println!("Vpiši število vrstic (2-20, privzeto 6): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse() {
            if n >= 2 && n <= 20 {
                row_count = n;
                break;
            }
            println!("Igra mora imeti vsaj 2 vrstice.");
        } else if input.trim().is_empty() {
            break;
        } else {
            println!("Neveljaven vnos.");
        }
    }

    // dobi input za stolpce
    let max_col = get_conjugate_value(row_count);
    loop {
        println!("Vpiši število stolpcev (2-{}, privzeto 7): ", {max_col});
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse() {
            if n >= 2 && n <= max_col && n <= max_col {
                column_count = n;
                break;
            }
            println!("Igra mora imeti vsaj 2 stolpce.");
        } else if input.trim().is_empty() {
            break;
        } else {
            println!("Napačen vnos.");
        }
    }

    // dobi input za dolžino zmagovalne kombinacije
    let max_win = row_count.min(column_count);
    loop {
        println!("Vpiši dolžino zmagovalne kombinacije (2-{}, privzeto 4): ", {max_win});
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse() {
            if n >= 2 && n <= max_win {
                win_sequence = n;
                break;
            }
            println!("Dolžina zmagovalne kombinacije mora biti vsaj 2.");
        } else if input.trim().is_empty() {
            break;
        } else {
            println!("Napačen vnos.");
        }
    }

    (row_count, column_count, win_sequence)
}

pub fn get_player_column_input(size: u8) -> u8 {
    loop {
        println!("Vpiši številko stolpca (0-{}): ", {size-1});
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(col) if col < size => return col,
            _ => println!("Neveljaven vnos, poskusi ponovno."),
        }
    }
}

pub fn difficulty_input() -> u8 {
    let mut difficulty = 8;

    // dobi input za težavnost
    loop {
        println!("Vnesi težavnost igre (najlažje 1 <-> 10 najtežje, privzeto 8): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(n) = input.trim().parse() {
            if is_valid_difficulty(n) {
                difficulty = n;
                break;
            }
            println!("Neveljavna težavnost, izberi med 1 in 10.");
        } else if input.trim().is_empty() {
            break;
        } else {
            println!("Neveljaven vnos.");
        }
    }

    difficulty
}

pub fn first_player_input() -> bool {
    let mut player_starts = true;

    loop {
        println!("Kdo začne? (j - ti, a - AI začne, privzeto j): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if trimmed == "j" {
            player_starts = true;
            break;
        } else if trimmed == "a" {
            player_starts = false;
            break;
        } else if trimmed.is_empty() {
            break;
        } else {
            println!("Izberi j ali a.");
        }
    }

    player_starts
}

pub fn main_loop_terminal() {
    let settings = get_player_settings_input();
    let difficulty = difficulty_input();
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
            // AI poteza z minimax
            println!("AI razmišlja...");
            col = match minimax(&board, difficulty, i32::MIN, i32::MAX, true).0 {
                Some(c) => c,
                None => {
                    println!("AI ne more izvesti poteze, igra je končana.");
                    break;
                }
            };
            println!("AI izbere stolpec {}", col);
        }

        // Poskusi dodati žeton v stolpec
        if let Some(new_board) = board.drop_piece(col, current) {
            board = new_board;
            println!("{}", board);

            if board.check_win(current) {
                println!("{:?} zmaga!", current);
                break;
            }

            if board.is_full() {
                println!("Izenačeno!");
                break;
            }

            // zamenjaj trenutnega igralca
            current = if current == Piece::Player { Piece::AI } else { Piece::Player };
        } else {
            println!("Stolpec {} je poln. Poskusi ponovno.", col);
        }
    }
}

const MAX_DIFFICULTY: u8 = 10;
const MIN_DIFFICULTY: u8 = 1;

fn is_valid_difficulty(difficulty: u8) -> bool {
    difficulty >= MIN_DIFFICULTY && difficulty <= MAX_DIFFICULTY
}

// površina plošče mora biti < 128, da se prilega v eno število (s presledki med stolpci)
fn get_conjugate_value(x: u8) -> u8 {
    (128/(x+1))-1
}