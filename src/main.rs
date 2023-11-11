// src/main.rs
mod input_helpers;

use parchis_game::game::Game;
use parchis_game::{Player, Pawn, Color};
use input_helpers::{clear_screen, get_unique_player_name, get_unique_color, print_available_colors, print_header};
use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    clear_screen();
    print_header();
    let mut player_names = HashSet::new();
    let mut player_colors = HashSet::new();
    let mut players = Vec::new();

    let number_of_players = loop {
        print!("Enter the number of players (2-4): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            match input.trim().parse::<usize>() {
                Ok(num) if (2..=4).contains(&num) => break num,
                _ => println!("Invalid input. Please enter a number between 2 and 4."),
            }
        } else {
            println!("Error reading input. Please try again.");
        }
    };

    for player_number in 1..=number_of_players {
        clear_screen();
        println!("Player {} setup:", player_number);
        let name = get_unique_player_name(&player_names);
        player_names.insert(name.clone());

        print_available_colors(&player_colors);
        let color = get_unique_color(&player_colors);
        player_colors.insert(color);

        let pawns = [
            Pawn { id: 1, color, position: 0 },
            Pawn { id: 2, color, position: 0 },
            Pawn { id: 3, color, position: 0 },
            Pawn { id: 4, color, position: 0 },
        ];

        players.push(Player {
            name: name,
            pawns,
        });
    }

    let mut game = Game::new(players);

    while !game.game_has_winner() {
        clear_screen();
        game.print_turn_banner();
        game.show_pawn_positions();

        game.play_turn();

        if game.game_has_winner() {
            println!("Player {} has won the game!", game.players[game.current_turn].name);
            break;
        }

        println!("\nPress Enter to continue to the next turn.");
        io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    }

    println!("Congratulations, {} has won the game!", game.players[game.current_turn].name);
}
