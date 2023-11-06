// src/main.rs
mod input_helpers;

use parchis_game::game::Game;
use parchis_game::{Player, Pawn, Color};
use input_helpers::{get_unique_player_name, get_unique_color};
use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    let mut player_names = HashSet::new();
    let mut player_colors = HashSet::new();
    let mut players_list = Vec::new();

    let number_of_players = loop {
        print!("Enter the number of players (2-4): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(num) if num >= 2 && num <= 4 => break num,
            _ => println!("Invalid input. Please enter a number between 2 and 4."),
        }
    };

    for _ in 0..number_of_players {
        let name = get_unique_player_name(&player_names);
        player_names.insert(name.clone());
        players_list.push(name);
    }

    let mut players = Vec::new(); 
    for name in players_list {
        let color = get_unique_color(&player_colors);
        player_colors.insert(color);

        // Create pawns for the player
        let pawns = [
            Pawn { color, position: 0 },
            Pawn { color, position: 0 },
            Pawn { color, position: 0 },
            Pawn { color, position: 0 },
        ];

        // Create the Player and add them to the game
        players.push(Player {
            name: name.to_string(),
            pawns,
        });
    }

    // Initialize the game with the collected players
    let mut game = Game::new(players);

    // Start the game loop
    println!("Game started with {} players", game.players.len());

    game.next_turn();
    println!("It's now player {}'s turn", game.players[game.current_turn].name);

}
