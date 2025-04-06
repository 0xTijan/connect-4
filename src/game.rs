use crate::board::Board;

pub struct Game {
    pub size: (usize, usize),
    pub board: Board,
}

impl Game {
    pub fn new(size: (usize, usize)) -> Self {
        let board = Board::new(size);
        Game { size, board }
    }
}