#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileType {
    Bamboo(u8),
    Character(u8),
    Circle(u8),
    Wind(Wind),
    Dragon(Dragon),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dragon {
    Red,
    Green,
    White,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile {
    pub tile_type: TileType,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Tile { tile_type }
    }
}

#[cfg(test)]
mod tests {
    use super::{Dragon, Tile, TileType};

    #[test]
    fn test_tile_new() {
        let tile = Tile::new(TileType::Dragon(Dragon::Green));
        assert!(tile.tile_type == TileType::Dragon(Dragon::Green));
    }
}
