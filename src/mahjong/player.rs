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

    // Implement other functions related to player actions as needed
}
