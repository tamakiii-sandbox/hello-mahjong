pub type HandView = Vec<String>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BambooEmoji {
    One = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CharacterEmoji {
    One = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CircleEmoji {
    One = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WindEmoji {
    East = 0,
    South,
    West,
    North,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DragonEmoji {
    Red = 0,
    Green,
    White,
}

impl From<BambooEmoji> for String {
    fn from(emoji: BambooEmoji) -> Self {
        ["🀐", "🀑", "🀒", "🀓", "🀔", "🀕", "🀖", "🀗", "🀘"][emoji as usize].to_string()
    }
}

impl From<CharacterEmoji> for String {
    fn from(emoji: CharacterEmoji) -> Self {
        ["🀇", "🀈", "🀉", "🀊", "🀋", "🀌", "🀍", "🀎", "🀏"][emoji as usize].to_string()
    }
}

impl From<CircleEmoji> for String {
    fn from(emoji: CircleEmoji) -> Self {
        ["🀙", "🀚", "🀛", "🀜", "🀝", "🀞", "🀟", "🀠", "🀡"][emoji as usize].to_string()
    }
}

impl From<WindEmoji> for String {
    fn from(emoji: WindEmoji) -> Self {
        ["🀀", "🀁", "🀂", "🀃"][emoji as usize].to_string()
    }
}

impl From<DragonEmoji> for String {
    fn from(emoji: DragonEmoji) -> Self {
        ["🀆", "🀅", "🀄"][emoji as usize].to_string()
    }
}

impl BambooEmoji {
    pub fn from_usize(value: usize) -> Self {
        match value {
            0 => BambooEmoji::One,
            1 => BambooEmoji::Two,
            2 => BambooEmoji::Three,
            3 => BambooEmoji::Four,
            4 => BambooEmoji::Five,
            5 => BambooEmoji::Six,
            6 => BambooEmoji::Seven,
            7 => BambooEmoji::Eight,
            8 => BambooEmoji::Nine,
            _ => panic!("Invalid value for BambooEmoji"),
        }
    }
}

impl CharacterEmoji {
    pub fn from_usize(value: usize) -> Self {
        match value {
            0 => CharacterEmoji::One,
            1 => CharacterEmoji::Two,
            2 => CharacterEmoji::Three,
            3 => CharacterEmoji::Four,
            4 => CharacterEmoji::Five,
            5 => CharacterEmoji::Six,
            6 => CharacterEmoji::Seven,
            7 => CharacterEmoji::Eight,
            8 => CharacterEmoji::Nine,
            _ => panic!("Invalid value for CharacterEmoji"),
        }
    }
}

impl CircleEmoji {
    pub fn from_usize(value: usize) -> Self {
        match value {
            0 => CircleEmoji::One,
            1 => CircleEmoji::Two,
            2 => CircleEmoji::Three,
            3 => CircleEmoji::Four,
            4 => CircleEmoji::Five,
            5 => CircleEmoji::Six,
            6 => CircleEmoji::Seven,
            7 => CircleEmoji::Eight,
            8 => CircleEmoji::Nine,
            _ => panic!("Invalid value for CircleEmoji"),
        }
    }
}

impl WindEmoji {
    pub fn from_usize(value: usize) -> Self {
        match value {
            0 => WindEmoji::East,
            1 => WindEmoji::South,
            2 => WindEmoji::West,
            3 => WindEmoji::North,
            _ => panic!("Invalid value for WindEmoji"),
        }
    }
}

impl DragonEmoji {
    pub fn from_usize(value: usize) -> Self {
        match value {
            0 => DragonEmoji::Red,
            1 => DragonEmoji::Green,
            2 => DragonEmoji::White,
            _ => panic!("Invalid value for DragonEmoji"),
        }
    }
}
