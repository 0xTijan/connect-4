use crate::board::Board;

pub struct Game {
    pub size: (usize, usize),
    pub board: Board,
    pub difficulty: u8,
}

impl Game {
    pub fn new(size: (usize, usize)) -> Self {
        let board = Board::new();
        Game { size, board, difficulty: 0 }
    }

    pub fn print_board(&self) {
        for row in (0..self.size.1).rev() { // print top to bottom
            for col in 0..self.size.0 {
                let bit_index = col * 7 + row;
                let mask = 1u128 << bit_index;
    
                let ch = if self.board.player_boards[0] & mask != 0 {
                    'X' // Player 1
                } else if self.board.player_boards[1] & mask != 0 {
                    'O' // Player 2
                } else {
                    '.' // Empty
                };
    
                print!("{}  ", ch);
            }
            println!();
        }

        // display column numbers
        println!("{}", "-".repeat(self.size.0 * 3 - 1)); 
        for i in 0..self.size.0 {
            if i < 10 {
                print!("{}  ", i);
            } else {
                print!("{} ", i);
            }
        }
    }
}