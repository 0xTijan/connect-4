pub fn size_input() -> (usize, usize) {
    // keep asking for input, until: valid size and valid input
    loop {
        println!("Enter the size of the board (width height): ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        // check if has 2 numbers
        if parts.len() != 2 {
            println!("Invalid input. Please enter two numbers.");
            continue;
        }

        // check if both numbers are valid
        let width: usize = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid width. Please enter a valid number.");
                continue;
            }
        };
        let height: usize = match parts[1].parse() {
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

const MAX_BITS: usize = 128;    // bitboard is stored in u128
const MIN_WIDTH: usize = 4;
const MIN_HEIGHT: usize = 4;

fn is_valid_size(width: usize, height: usize) -> bool {
    width >= MIN_WIDTH &&
    height >= MIN_HEIGHT &&
    width * (height + 1) <= MAX_BITS
}