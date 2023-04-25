use super::tile::Tile;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: usize,
    pub hand: Vec<Tile>,
    pub melds: Vec<Meld>,
    pub points: i32,
    pub discards: Vec<Tile>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Meld {
    Chow(Tile, Tile, Tile),
    Pung(Tile),
    Kong(Tile),
}

impl Player {
    pub fn new(id: usize) -> Self {
        Player {
            id,
            hand: Vec::new(),
            melds: Vec::new(),
            points: 0,
            discards: Vec::new(),
        }
    }

    pub fn draw_tile(&mut self, tile: Tile) {
        self.hand.push(tile);
    }

    pub fn remove_tile(&mut self, index: usize) -> Option<Tile> {
        if index < self.hand.len() {
            let discarded_tile = self.hand.remove(index);
            self.discards.push(discarded_tile.clone());
            Some(discarded_tile)
        } else {
            None
        }
    }

    // pub fn form_chow(&mut self, t1: Tile, t2: Tile, t3: Tile) -> Result<(), String> {
    //     // Implement the logic to check and form a Chow meld here
    // }

    // pub fn form_pung(&mut self, t1: Tile, t2: Tile, t3: Tile) -> Result<(), String> {
    //     // Implement the logic to check and form a Pung meld here
    // }

    // pub fn form_kong(&mut self, t1: Tile, t2: Tile, t3: Tile, t4: Tile) -> Result<(), String> {
    //     // Implement the logic to check and form a Kong meld here
    // }
}

#[cfg(test)]
mod tests {
    use crate::mahjong::tile::{Tile, TileType};

    use super::Player;

    #[test]
    fn test_new() {
        let player = Player::new(1);
        assert_eq!(1, player.id);
    }

    #[test]
    fn test_draw_tile() {
        let mut player = Player::new(1);
        let tile = Tile::new(TileType::Bamboo(1));

        assert_eq!(0, player.hand.len());
        player.draw_tile(tile);
        assert_eq!(1, player.hand.len());
        assert_eq!(TileType::Bamboo(1), player.hand[0].tile_type);
    }

    #[test]
    fn test_remove_tile() {
        let mut player = Player::new(1);
        let tile = Tile::new(TileType::Character(2));

        player.draw_tile(tile);
        let actual = player.remove_tile(0);
        assert_eq!(TileType::Character(2), actual.unwrap().tile_type);
    }
}
