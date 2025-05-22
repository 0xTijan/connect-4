use std::io;

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
            if n >= 2 && n <= 20 && n <= max_col {
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
        println!("Enter the difficulty (easiest 1 <-> 20 hardest, default 10): ");
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

const MAX_DIFFICULTY: u8 = 20;
const MIN_DIFFICULTY: u8 = 1;

fn is_valid_difficulty(difficulty: u8) -> bool {
    difficulty >= MIN_DIFFICULTY && difficulty <= MAX_DIFFICULTY
}

// board area must be < 128 so that it fits in one number (with paddings between columns)
fn get_conjugate_value(x: u8) -> u8 {
    (128/(x+1))-1
}