#[derive(Clone, Debug)]
pub struct BitBoard {
    player_mask: u128,  // bitboard for player pieces
    ai_mask: u128,      // bitboard for AI pieces
    heights: Vec<u8>,
    rows: u8,
    cols: u8,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Piece {
    Empty,
    Player,
    AI,
}

impl BitBoard {
    // create a new BitBoard with the given number of rows and columns
    pub fn new(rows: u8, cols: u8) -> Self {
        Self {
            player_mask: 0, // initialize empty boards
            ai_mask: 0,
            heights: vec![0; cols as usize],
            rows,
            cols,
        }
    }

    // returns integer [0, rows * cols) where the bit is located in the bitboard (which bit in the u128 is this cell at)
    // bits are ordered from left to right, bottom to top of the column
    fn bit_index(&self, row: u8, col: u8) -> u8 {
        col * self.rows + row
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

        let rows = self.rows;
        let directions = [
            1,          // vertical
            rows,       // horizontal (moving to next column)
            rows + 1,   // diagonal / (one up, one right)
            rows - 1    // diagonal \ (one down, one right)
        ];
        
        // check for each direction
        for dir in directions {
            // shift the mask in direction once
            let shifted = mask & (mask >> dir);
            // check shifted and double shifted (3 times total shifted) if they have any overlapping bits
            if (shifted & (shifted >> (2 * dir))) != 0 {
                // overlapping bits means there are 4 in a row
                return true;
            }
        }

        false
    }

    // prints the board state to the terminal
    pub fn print(&self) {
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
    }
}
