use crate::mahjong::tile::{Dragon, Tile, TileType, Wind};
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::player::Player;

pub struct Game {
    wall: Vec<Tile>,
    discards: Vec<Tile>,
    pub players: Vec<Player>,
    pub current_turn: usize,
}

impl Game {
    pub fn new(
        wall: Vec<Tile>,
        discards: Vec<Tile>,
        players: Vec<Player>,
        current_turn: usize,
    ) -> Self {
        Game {
            wall,
            discards,
            players,
            current_turn,
        }
    }

    pub fn draw_tile(&mut self) -> Option<Tile> {
        self.wall.pop()
    }

    pub fn discard_tile(&mut self, tile: Tile) {
        self.discards.push(tile);
    }

    pub fn get_wall(&self) -> Vec<Tile> {
        self.wall.clone()
    }

    pub fn wall_size(&self) -> usize {
        self.wall.len()
    }

    pub fn wall_contains(&self, tile: &Tile) -> bool {
        self.wall.contains(tile)
    }

    pub fn next_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.players.len()
    }
}

fn init_players(num_players: usize) -> Vec<Player> {
    let mut players = Vec::new();
    for i in 0..num_players {
        players.push(Player::new(i));
    }
    players
}

fn select_starting_player() -> usize {
    0 // You can change this to select a random starting player if desired
}

pub fn initialize(num_players: usize) -> Game {
    let wall = generate_random_wall();
    let discards = Vec::new();
    let players = init_players(num_players);
    let current_turn = select_starting_player();

    let mut game = Game::new(wall, discards, players, current_turn);

    let initial_hand_size = 13;
    let mut initial_hands: Vec<Vec<Tile>> =
        vec![Vec::with_capacity(initial_hand_size); num_players];

    for hand in initial_hands.iter_mut() {
        for _ in 0..initial_hand_size {
            let tile = game.draw_tile();
            hand.push(tile.unwrap());
        }
    }

    for (player, hand) in game.players.iter_mut().zip(initial_hands) {
        player.hand = hand;
    }

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
    use super::initialize;

    #[test]
    fn test_initialize() {
        let game = initialize(4);
        assert_eq!(4, game.players.len());

        assert_eq!(84, game.wall_size());
        assert_eq!(13, game.players[0].hand.len());
        assert_eq!(13, game.players[1].hand.len());
        assert_eq!(13, game.players[2].hand.len());
        assert_eq!(13, game.players[3].hand.len());

        assert_eq!(136, game.wall_size() + game.players.len() * 13);
    }
}
