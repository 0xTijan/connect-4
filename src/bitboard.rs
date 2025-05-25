use std::fmt;


#[derive(Clone, Debug)]
pub struct BitBoard {
    pub player_mask: u128,  // bitboard for player pieces
    pub ai_mask: u128,      // bitboard for AI pieces
    heights: Vec<u8>,
    pub rows: u8,
    pub cols: u8,
    pub connect: u8,        // number of pieces to connect to win
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Piece {
    Empty,
    Player,
    AI,
}

impl BitBoard {
    // create a new BitBoard with the given number of rows and columns
    pub fn new(rows: u8, cols: u8, connect: u8) -> Self {
        Self {
            player_mask: 0, // initialize empty boards
            ai_mask: 0,
            heights: vec![0; cols as usize],
            rows,
            cols,
            connect
        }
    }

    // returns integer [0, rows * cols) where the bit is located in the bitboard (which bit in the u128 is this cell at)
    // bits are ordered from left to right, bottom to top of the column, with 1 bit gap between columns
    fn bit_index(&self, row: u8, col: u8) -> u8 {
        col * (self.rows + 1) + row
    }

    // returns u128 with only one bit set to 1 at the position given by bit_index, others are 0
    fn bit(&self, row: u8, col: u8) -> u128 {
        1u128 << self.bit_index(row, col)
    }

    // checks if column is not full
    pub fn is_valid_location(&self, col: u8) -> bool {
        self.heights[col as usize] < self.rows
    }

    // returns a vector of all valid columns (where piece can be dropped)
    pub fn get_valid_locations(&self) -> Vec<u8> {
        (0..self.cols).filter(|&c| self.is_valid_location(c)).collect()
    }

    // returns a new BitBoard if the piece was dropped successfully
    pub fn drop_piece(&self, col: u8, piece: Piece) -> Option<Self> {
        // check if column is not full
        if !self.is_valid_location(col) {
            return None;
        }

        // get the row where the piece will be dropped (first empty row in the column from bottom)
        let row = self.heights[col as usize];
        // get the bitmask to set for the piece (in correct position)
        let bit_to_set = self.bit(row, col);

        let mut new_board = self.clone();
        // increase the height of the column - piece is dropped
        new_board.heights[col as usize] += 1;         

        // set the bit in the correct BitBoard (according to the player)
        match piece {
            Piece::Player => new_board.player_mask |= bit_to_set,
            Piece::AI => new_board.ai_mask |= bit_to_set,
            _ => {}
        }

        Some(new_board)
    }

    // returns the piece at given row and column
    pub fn get_piece(&self, row: u8, col: u8) -> Piece {
        let bit = self.bit(row, col);

        if self.player_mask & bit != 0 {
            Piece::Player
        } else if self.ai_mask & bit != 0 {
            Piece::AI
        } else {
            Piece::Empty
        }
    }

    // returns boolean if the whole board is full
    pub fn is_full(&self) -> bool {
        self.heights.iter().all(|&h| h >= self.rows)
    }

    // checks if the given piece has won - using a bitwise shift method
    pub fn check_win(&self, piece: Piece) -> bool {
        // select the corresponding bitmask
        let mask = match piece {
            Piece::Player => self.player_mask,
            Piece::AI => self.ai_mask,
            _ => 0,
        };

        let row_stride = self.rows + 1; // 1 bit gap
        let directions = [
            1,          // vertical
            row_stride,       // horizontal (moving to next column)
            row_stride - 1,   // diagonal / (one up, one right)
            row_stride + 1    // diagonal \ (one down, one right)
        ];
        
        // check for each direction
        for dir in directions {
            let mut current = mask;

            // shift and AND the mask multiple times to detect a sequence - repeat for connect-1 times
            // example: connect 4 => 3 times shift and AND
            for _ in 0..self.connect - 1 {
                // shift the mask in direction once
                current = current & (current >> dir);
            }

            // if bits remain after shifting - won
            if current != 0 {
                return true;
            }
        }

        false
    }

    pub fn can_win_next_move(&self, piece: Piece) -> bool {
        for col in self.get_valid_locations() {
            if let Some(next_board) = self.drop_piece(col, piece) {
                if next_board.check_win(piece) {
                    return true;
                }
            }
        }
        false
    }

    // prints the board state to the terminal
    /*pub fn print(&self) {
        println!();
        for r in (0..self.rows).rev() {
            for c in 0..self.cols {
                // get corresponding piece in that position
                let symbol = match self.get_piece(r, c) {
                    Piece::Empty => '.',
                    Piece::Player => 'X',
                    Piece::AI => 'O',
                };
                print!(" {} ", symbol);
            }
            println!();
        }
        // draw a bottom line
        println!("{}", "-".repeat((self.cols as usize) * 3));
        // print column numbers
        for c in 0..self.cols {
            print!(" {} ", c);
        }
        println!("\n");
    }*/
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?; // blank line for spacing
        for r in (0..self.rows).rev() {
            for c in 0..self.cols {
                let symbol = match self.get_piece(r, c) {
                    Piece::Empty => '.',
                    Piece::Player => 'X',
                    Piece::AI => 'O',
                };
                write!(f, " {} ", symbol)?;
            }
            writeln!(f)?; // newline after each row
        }
        writeln!(f, "{}", "-".repeat((self.cols as usize) * 3))?; // bottom line

        for c in 0..self.cols {
            write!(f, " {} ", c)?;
        }
        writeln!(f)?; // final newline
        Ok(())
    }
}
