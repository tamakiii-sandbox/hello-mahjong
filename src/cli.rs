// use mahjong::mahjong::{game::initialize, view::TileView};
// use crate::mahjong::game::{initialize, Game};
// use crate::mahjong::gamemaster::GameMaster;

use mahjong::mahjong::{game::initialize, game_master::GameMaster, tile::Tile, view::TileView};
use std::io::{stdin, stdout, Write};

pub fn run() {
    let game = initialize(4);
    let mut gm = GameMaster::new(game);

    loop {
        {
            // Display the current player's hand.
            let player = gm.current_player();
            let emojis = hand_to_emoji(&player.hand).join(" ");
            println!("Player {}'s hand: {:?}", gm.current_turn() + 1, emojis);
        }

        // Ask the player if they want to draw a tile or check for a winning hand.
        println!("Choose an action: (d)raw, (c)heck for win, or (q)uit:");
        let action = read_input().trim().to_lowercase();

        // If the player choose to draw a tile, draw a tile and add it to their hand.
        if action == "d" {
            // if gm.game.wall.is_empty() {}
            // let index = gm.player_index(&player);
            let index = gm.current_turn();
            gm.draw_tile(index);
        } else if action == "c" {
            println!("Checking for a winning hand is not implemented yet.");
        } else if action == "q" {
            println!("Quitting the game.")
        } else {
            println!("Invalid input. Please enter a valid action.");
            continue;
        }

        {
            let player = gm.current_player();
            let emojis = hand_to_emoji(&player.hand).join(" ");
            println!("Player {}'s hand: {:?}", gm.current_turn() + 1, emojis);
        }

        // Ask the player to choose a tile to discard
        println!("Choose a tile to discard (0-based index):");
        let discard_index = read_input().trim().parse::<usize>().unwrap_or_else(|_| {
            println!("Invalid index. Using default index 0.");
            0
        });

        // Remove the chosen tile from their handand add it to the discard pile.
        gm.discard_tile(gm.current_turn(), discard_index);

        gm.next_turn();
    }
}

fn read_input() -> String {
    let mut input = String::new();
    print!("> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    input
}

fn hand_to_emoji(hand: &Vec<Tile>) -> Vec<String> {
    hand.iter()
        .map(|tile| TileView::from(tile.clone()).emoji)
        .collect::<Vec<String>>()
}
