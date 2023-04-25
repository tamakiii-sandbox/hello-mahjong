use crate::mahjong::tile::{Dragon, Tile, TileType, Wind};
use crate::mahjong::view_emoji::{
    BambooEmoji, CharacterEmoji, CircleEmoji, DragonEmoji, WindEmoji,
};
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
    pub emoji: String,
}

impl From<Tile> for TileView {
    fn from(tile: Tile) -> Self {
        let tile_type = TileTypeView::from(tile.tile_type.clone());
        let emoji = match tile.tile_type {
            TileType::Bamboo(n) => String::from(BambooEmoji::from_usize(n as usize - 1)),
            TileType::Character(n) => String::from(CharacterEmoji::from_usize(n as usize - 1)),
            TileType::Circle(n) => String::from(CircleEmoji::from_usize(n as usize - 1)),
            TileType::Wind(w) => String::from(WindEmoji::from_usize(WindView::from(w) as usize)),
            TileType::Dragon(d) => {
                String::from(DragonEmoji::from_usize(DragonView::from(d) as usize))
            }
        };
        TileView { tile_type, emoji }
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

    fn sort_json_keys(json: &str) -> String {
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        let sorted_value = sort_value(value);
        serde_json::to_string(&sorted_value).unwrap()
    }

    fn sort_value(value: serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Object(mut map) => {
                let mut sorted_map = serde_json::Map::new();
                let keys: Vec<String> = map.keys().cloned().collect();
                for key in keys {
                    let v = map.remove(&key).unwrap();
                    sorted_map.insert(key, sort_value(v));
                }
                serde_json::Value::Object(sorted_map)
            }
            serde_json::Value::Array(arr) => {
                serde_json::Value::Array(arr.into_iter().map(sort_value).collect())
            }
            _ => value,
        }
    }

    #[test]
    fn test_from() {
        let mut wall: Vec<Tile> = Vec::new();
        wall.push(Tile::new(TileType::Bamboo(1)));
        wall.push(Tile::new(TileType::Character(1)));
        wall.push(Tile::new(TileType::Circle(1)));

        let view: Vec<TileView> = wall.into_iter().map(TileView::from).collect();

        let json = serde_json::to_string(&view).unwrap();
        let expected = json!([
            {"tile_type": {"Bamboo": 1}, "emoji": "🀐"},
            {"tile_type": {"Character": 1}, "emoji": "🀇"},
            {"tile_type": {"Circle": 1}, "emoji": "🀙"}
        ]);

        assert!(true, "Unit tests of views");
        assert_eq!(sort_json_keys(&json), sort_json_keys(&expected.to_string()));
    }
}
