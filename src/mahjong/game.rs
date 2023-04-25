use crate::mahjong::tile::{Dragon, Tile, TileType, Wind};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Game {
    pub wall: Vec<Tile>,
}

impl Game {
    pub fn new(wall: Vec<Tile>) -> Self {
        Game { wall }
    }

    pub fn draw_tile(&mut self) -> Option<Tile> {
        self.wall.pop()
    }

    pub fn wall_size(&self) -> usize {
        self.wall.len()
    }

    pub fn wall_contains(&self, tile: &Tile) -> bool {
        self.wall.contains(tile)
    }
}

pub fn initialize() -> Game {
    let wall = generate_random_wall();
    let game = Game::new(wall);
    game
}

fn generate_wall() -> Vec<Tile> {
    // Generate a full Mahjong wall with 144 tiles
    let mut wall = Vec::new();

    for _ in 0..4 {
        for i in 1..=9 {
            wall.push(Tile::new(TileType::Bamboo(i)));
            wall.push(Tile::new(TileType::Character(i)));
            wall.push(Tile::new(TileType::Circle(i)));
        }
        wall.push(Tile::new(TileType::Wind(Wind::East)));
        wall.push(Tile::new(TileType::Wind(Wind::South)));
        wall.push(Tile::new(TileType::Wind(Wind::West)));
        wall.push(Tile::new(TileType::Wind(Wind::North)));

        wall.push(Tile::new(TileType::Dragon(Dragon::Red)));
        wall.push(Tile::new(TileType::Dragon(Dragon::Green)));
        wall.push(Tile::new(TileType::Dragon(Dragon::White)));
    }

    wall
}

pub fn generate_random_wall() -> Vec<Tile> {
    let mut wall = generate_wall();

    // Shuffle the wall
    let mut rng = thread_rng();
    wall.shuffle(&mut rng);
    wall
}

#[cfg(test)]
mod tests {
    use super::generate_random_wall;
    use super::initialize;
    use super::Game;

    #[test]
    fn test_game_new() {
        let wall = generate_random_wall();
        let game = Game::new(wall);
        assert_eq!(game.wall_size(), 136);
    }

    #[test]
    fn test_initialize() {
        let game = initialize();
        assert_eq!(game.wall_size(), 136);
    }
}
