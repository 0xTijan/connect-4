pub struct Board(pub u128);

impl Board {
    pub fn new(size: (usize, usize)) -> Self {
        Board(0)    // empty bitboard
    }
}