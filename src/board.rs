pub struct Board {
    pub player_boards: [u128; 2]
}

impl Board {
    pub fn new() -> Self {
        Board {
            player_boards: [0; 2]       // empty bitboard for each player
        }
    }
}