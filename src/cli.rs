use mahjong::mahjong::{
    game::initialize, game_master::GameMaster, player::Hand, view::TileView, view_emoji::HandView,
};
use std::io::{stdin, stdout, Write};

pub enum Event {
    DrawTile(usize),           // player_index
    DiscardTile(usize, usize), // player_index, tile_index
    CheckForWin(usize),        // player_index
    Quit,
}

pub fn run() {
    let game = initialize(4);
    let mut gm = GameMaster::new(game);

    loop {
        let player = gm.current_player();
        let emojis = hand_to_emoji(&player.hand).join(" ");
        println!("Player {}'s hand: {:?}", gm.current_turn() + 1, emojis);

        let action = prompt_for_action();
        let event = match action.as_str() {
            "d" => {
                let draw_event = Event::DrawTile(gm.current_turn());
                handle_event(&mut gm, draw_event);
                let discard_index = prompt_for_discard_index();
                Event::DiscardTile(gm.current_turn(), discard_index)
            }
            "c" => Event::CheckForWin(gm.current_turn()),
            "q" => Event::Quit,
            _ => {
                println!("Invalid input. Please enter a valid action.");
                continue;
            }
        };

        {
            let emojis = hand_to_emoji(&player.hand).join(" ");
            println!("Player {}'s hand: {:?}", gm.current_turn() + 1, emojis);
        }

        handle_event(&mut gm, event);
    }
}

fn prompt_for_action() -> String {
    println!("Choose an action: (d)raw, (c)heck for win, or (q)uit:");
    read_input().trim().to_lowercase()
}

fn handle_event(gm: &mut GameMaster, event: Event) {
    match event {
        Event::DrawTile(player_index) => {
            gm.draw_tile(player_index);
        }
        Event::DiscardTile(player_index, tile_index) => {
            gm.discard_tile(player_index, tile_index);
            gm.next_turn();
        }
        Event::CheckForWin(_player_index) => {
            println!("Checking for a winning hand is not implemented yet.");
        }
        Event::Quit => {
            println!("Quitting the game.");
            std::process::exit(0);
        }
    }
}

fn prompt_for_discard_index() -> usize {
    println!("Choose a tile to discard (0-based index):");
    let discard_index = read_input().trim().parse::<usize>().unwrap_or_else(|_| {
        println!("Invalid index. Using default index 0.");
        0
    });
    discard_index
}

fn read_input() -> String {
    let mut input = String::new();
    print!("> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    input
}

fn hand_to_emoji(hand: &Hand) -> HandView {
    hand.iter()
        .map(|tile| TileView::from(tile.clone()).emoji)
        .collect::<Vec<String>>()
}
