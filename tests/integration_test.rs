use mahjong::mahjong::game::Game;

#[test]
fn test_game_initialization() {
    let game = Game::new();
    println!("Integration test: Game created!");
    assert_eq!(game.wall_size(), 136);
}
