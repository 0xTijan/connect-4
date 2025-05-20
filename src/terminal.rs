pub fn size_input() -> (u8, u8) {
    // keep asking for input, until: valid size and valid input
    loop {
        println!("Enter the size of the board (width height separated by space): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        // check if has 2 numbers
        if parts.len() != 2 {
            println!("Invalid input. Please enter two numbers.");
            continue;
        }

        // check if both numbers are valid
        let width: u8 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid width. Please enter a valid number.");
                continue;
            }
        };
        let height: u8 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid height. Please enter a valid number.");
                continue;
            }
        };

        // check if board size is valid
        match is_valid_size(width, height) {
            true => return (width, height),
            false => {
                println!("Invalid size. Please enter a valid size.");
                continue;
            }
        }
    }
}

pub fn difficulty_input() -> u8 {
    // keep asking for input, until: valid size and valid input
    loop {
        println!("Enter the difficulty (1-5): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let val= input.trim().to_string();

        // check if valid number
        let difficulty: u8 = match val.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid difficulty. Please enter a valid number.");
                continue;
            }
        };
        
        // check if board size is valid
        match is_valid_difficulty(difficulty) {
            true => return difficulty,
            false => {
                println!("Invalid difficulty. Please enter a valid difficulty.");
                continue;
            }
        }
    }
}

const MAX_BITS: u8 = 128;    // bit board is stored in u128
const MIN_WIDTH: u8 = 4;
const MIN_HEIGHT: u8 = 4;
const MAX_DIFFICULTY: u8 = 5;
const MIN_DIFFICULTY: u8 = 1;

fn is_valid_size(width: u8, height: u8) -> bool {
    width >= MIN_WIDTH &&
    height >= MIN_HEIGHT &&
    (width + 1) * (height + 1) <= MAX_BITS
}

fn is_valid_difficulty(difficulty: u8) -> bool {
    difficulty >= MIN_DIFFICULTY && difficulty <= MAX_DIFFICULTY
}