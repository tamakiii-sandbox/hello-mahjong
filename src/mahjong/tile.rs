pub enum TileType {
    Bamboo(u8),
    Character(u8),
    Circle(u8),
    Wind(Wind),
    Dragon(Dragon),
}

pub enum Wind {
    East,
    South,
    West,
    North,
}

pub enum Dragon {
    Red,
    Green,
    White,
}

pub struct Tile {
    tile_type: TileType,
}
