mod game;
mod board;

fn main() {
    let game = game::Game::new((7, 6));
    println!("Game created with size {:?}", game.size);
}