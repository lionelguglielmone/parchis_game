use std::collections::HashSet;
use std::io::{self, Write};
use crate::Color;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_header() {
    clear_screen();
    println!("=================================");
    println!("      Welcome to Parchis!         ");
    println!("=================================");
}

pub fn print_available_colors(existing_colors: &HashSet<Color>) {
    let color_map = vec![
        (1, "Red", Color::Red),
        (2, "Green", Color::Green),
        (3, "Blue", Color::Blue),
        (4, "Yellow", Color::Yellow),
    ];

    println!("\nAvailable colors:");
    for (num, color_name, color) in &color_map {
        if !existing_colors.contains(color) {
            println!("{}: {}", num, color_name);
        }
    }
    println!("Enter the number corresponding to your color choice:");
}

pub fn get_unique_player_name(existing_names: &HashSet<String>) -> String {
    let mut name = String::new();
    loop {
        print!("Enter a unique player name: ");
        io::stdout().flush().unwrap();
        name.clear();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let trimmed_name = name.trim();
        if !trimmed_name.is_empty() && !existing_names.contains(trimmed_name) {
            break trimmed_name.to_string();
        } else if trimmed_name.is_empty() {
            println!("Name cannot be empty. Please enter a unique name.");
        } else {
            println!("This name is already taken, please choose another one.");
        }
    }
}

pub fn get_unique_color(existing_colors: &HashSet<Color>) -> Color {
    let color_map = vec![
        (1, Color::Red),
        (2, Color::Green),
        (3, Color::Blue),
        (4, Color::Yellow),
    ];

    print!("Choose a color for the player by number: ");
    io::stdout().flush().unwrap();

    loop {
        let mut color_input = String::new();
        io::stdin().read_line(&mut color_input).expect("Failed to read line");

        match color_input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= color_map.len() => {
                if let Some(&(_, color)) = color_map.iter().find(|&&(n, _)| n == num) {
                    if !existing_colors.contains(&color) {
                        return color;
                    } else {
                        println!("This color is already taken, please choose another one.");
                    }
                }
            },
            _ => {
                println!("Invalid input. Please enter a number between 1 and {}.", color_map.len());
            }
        }
    }
}