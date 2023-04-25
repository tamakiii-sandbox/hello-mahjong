use super::game::Game;
use super::player::Player;
use super::tile::Tile;

pub struct GameMaster {
    game: Game,
}

impl GameMaster {
    pub fn new(game: Game) -> Self {
        GameMaster { game }
    }

    pub fn next_turn(&mut self) {
        self.game.current_turn = (self.game.current_turn + 1) % self.game.players.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::mahjong::game::initialize;

    use super::GameMaster;

    fn setup(num_players: usize) -> GameMaster {
        let game = initialize(num_players);
        GameMaster::new(game)
    }

    #[test]
    fn test_new() {
        setup(4);
        assert!(true);
    }

    #[test]
    fn test_next_turn() {
        let mut gm = setup(4);

        assert_eq!(0, gm.game.current_turn);
        gm.next_turn();
        assert_eq!(1, gm.game.current_turn);
    }
}
