use crate::mahjong::tile::{Dragon, Tile, TileType, Wind};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TileTypeView {
    Bamboo(u8),
    Character(u8),
    Circle(u8),
    Wind(WindView),
    Dragon(DragonView),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WindView {
    East,
    South,
    West,
    North,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DragonView {
    Red,
    Green,
    White,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TileView {
    pub tile_type: TileTypeView,
}

impl From<Tile> for TileView {
    fn from(tile: Tile) -> Self {
        let tile_type = TileTypeView::from(tile.tile_type);
        TileView { tile_type }
    }
}

impl From<TileType> for TileTypeView {
    fn from(tile_type: TileType) -> Self {
        match tile_type {
            TileType::Bamboo(n) => Self::Bamboo(n),
            TileType::Character(n) => Self::Character(n),
            TileType::Circle(n) => Self::Circle(n),
            TileType::Wind(w) => Self::Wind(WindView::from(w)),
            TileType::Dragon(d) => Self::Dragon(DragonView::from(d)),
        }
    }
}

impl From<Wind> for WindView {
    fn from(wind: Wind) -> Self {
        match wind {
            Wind::East => Self::East,
            Wind::South => Self::South,
            Wind::West => Self::West,
            Wind::North => Self::North,
        }
    }
}

impl From<Dragon> for DragonView {
    fn from(dragon: Dragon) -> Self {
        match dragon {
            Dragon::Red => Self::Red,
            Dragon::Green => Self::Green,
            Dragon::White => Self::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mahjong::{
        tile::{Tile, TileType},
        view::TileView,
    };
    use serde_json::{self, json};

    #[test]
    fn test_from() {
        let mut wall: Vec<Tile> = Vec::new();
        wall.push(Tile::new(TileType::Bamboo(1)));
        wall.push(Tile::new(TileType::Character(1)));
        wall.push(Tile::new(TileType::Circle(1)));

        let view: Vec<TileView> = wall.into_iter().map(TileView::from).collect();

        let json = serde_json::to_string(&view).unwrap();
        let expected = json!([
            {"tile_type": {"Bamboo": 1}},
            {"tile_type": {"Character": 1}},
            {"tile_type": {"Circle": 1}}
        ]);

        assert!(true, "Unit tests of views");
        assert_eq!(json, expected.to_string());
    }
}
