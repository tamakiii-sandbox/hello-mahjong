use mahjong::mahjong::game::Game;

#[test]
fn test_game_initialization() {
    let game = Game::new();
    println!("Integration test: Game created!");
    assert!(
        game.wall.is_empty(),
        "The wall should be empty when the game is initialized."
    );
}

// use mahjong::mahjong::game::Game;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_draw_tile_from_wall() {
//         let mut game = Game::new();
//         let initial_wall_size = game.wall_size();
//         assert!(initial_wall_size > game.wall_size());
//     }
// }

// #[cfg(test)]
// mod tests {
//     use mahjong::game::Game;
//     use mahjong::tile::TileType;

//     #[test]
//     fn test_draw_tile_from_wall() {
//         let mut game = Game::new();
//         let initial_wall_size = game.wall_size();
//         let drawn_tile = game.draw_tile().expect("Failed to draw a tile");
//         assert!(initial_wall_size > game.wall_size());
//         assert!(game.wall_contains(&drawn_tile) == false);
//     }
// }
