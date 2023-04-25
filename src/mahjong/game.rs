use crate::mahjong::tile::Tile;

pub struct Game {
    pub wall: Vec<Tile>,
}

impl Game {
    // pub fn new() -> Self {
    pub fn new() -> Game {
        let wall = Vec::new();
        Game { wall }
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn test_game_initialization() {
        let game = Game::new();
        assert!(
            game.wall.is_empty(),
            "The wall should be empty when the game is initialized."
        );
    }
}

// use crate::mahjang::tile::{Dragon, Tile, TileType, Wind};
// use rand::seq::SliceRandom;
// use rand::thread_rng;

// pub struct Game {
//     wall: Vec<Tile>,
// }

// impl Game {
//     pub fn new() -> Self {
//         let wall = generate_wall();
//         self { Wall }
//     }

//     pub fn draw_tile(&mut self) -> Option<Tile> {
//         self.wall.pop()
//     }

//     pub fn wall_size(&self) -> usize {
//         self.wall.len()
//     }

//     pub fn wall_contains(&self, tile: &Tile) -> bool {
//         self.wall.contains(tile)
//     }
// }

// fn generate_wall() -> Vec<Tile> {
//     // Generate a full Mahjong wall with 144 tiles
// }
