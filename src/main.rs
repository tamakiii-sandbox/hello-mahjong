use mahjong::mahjong::{game::initialize, view::TileView};

fn main() {
    let game = initialize(4);
    let view: Vec<TileView> = game.wall.into_iter().map(TileView::from).collect();
    let json = serde_json::to_string(&view).unwrap();
    println!("{}", json);
}
