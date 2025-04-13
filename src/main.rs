mod game;
mod board;
mod terminal;

fn main() {
    let size = terminal::size_input();
    let game = game::Game::new(size);
    println!("Game created with size {:?}", game.size);
    println!();
    game.print_board();
}