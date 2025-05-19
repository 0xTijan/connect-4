mod bitboard;
mod terminal;

use bitboard::{BitBoard, Piece};

fn main() {
    let size = terminal::size_input();
    let mut board = BitBoard::new(size.0, size.1);
    board.print();

    let mut current = Piece::Player;

    for turn in 0.. {
        let col = board.get_valid_locations()[0]; // choose first valid column
        if let Some(new_board) = board.drop_piece(col, current) {
            board = new_board;
            board.print();
            if board.check_win(current) {
                println!("{:?} wins!", current);
                break;
            }
            if board.is_full() {
                println!("It's a draw!");
                break;
            }
            current = if current == Piece::Player { Piece::AI } else { Piece::Player };
        }
    }
}
