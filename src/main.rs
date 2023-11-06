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
    let mut players = Vec::new();

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

        let color = get_unique_color(&player_colors);
        player_colors.insert(color);

        let pawns = [
            Pawn { id: 1, color, position: 0 },
            Pawn { id: 2, color, position: 0 },
            Pawn { id: 3, color, position: 0 },
            Pawn { id: 4, color, position: 0 },
        ];

        players.push(Player {
            name: name.to_string(),
            pawns,
        });
    }

    let mut game = Game::new(players);

    println!("Game started with {} players", game.players.len());
    loop {
        game.play_turn();

        if game.game_has_winner() {
            println!("Player {} has won the game!", game.players[game.current_turn].name);
            break;
        }

        println!("Press Enter to continue to the next turn.");
        io::stdin().read_line(&mut String::new()).expect("Failed to read line");

        println!("It's now player {}'s turn", game.players[game.current_turn].name);
    }
}
