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

    pub fn draw_tile(&mut self, player_index: usize) {
        let tile = self.game.draw_tile().unwrap();
        self.game.players[player_index].draw_tile(tile);
    }

    pub fn discard_tile(&mut self, player_index: usize, tile_index: usize) {
        let tile = self.game.players[player_index].remove_tile(tile_index);
        self.game.discard_tile(tile.unwrap());
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
