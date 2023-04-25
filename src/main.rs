use mahjong::mahjong::game::{generate_random_wall, Game};

fn main() {
    let wall = generate_random_wall();
    let game = Game::new(wall);
    println!("Game created!");
}
