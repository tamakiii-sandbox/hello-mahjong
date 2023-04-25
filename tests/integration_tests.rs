#[cfg(test)]
mod tests {
    use mahjong::game::Game;
    use mahjong::tile::TileType;

    #[test]
    fn test_draw_tile_from_wall() {
        let mut game = Game::new();
        let initial_wall_size = game.wall_size();
        let drawn_tile = game.draw_tile().expect("Failed to draw a tile");
        assert!(initial_wall_size > game.wall_size());
        assert!(game.wall_contains(&drawn_tile) == false);
    }
}