use std::collections::HashSet;
use std::io;
use crate::Color;

pub fn get_unique_player_name(existing_names: &HashSet<String>) -> String {
    let mut name = String::new();
    while name.is_empty() || existing_names.contains(&name) {
        println!("Enter a unique player name:");
        name.clear();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        name = name.trim().to_string();
        if existing_names.contains(&name) {
            println!("This name is already taken, please choose another one.");
        }
    }
    name
}

pub fn get_unique_color(existing_colors: &HashSet<Color>) -> Color {
    let mut color_choices = vec![
        ("Red", Color::Red), 
        ("Green", Color::Green), 
        ("Blue", Color::Blue), 
        ("Yellow", Color::Yellow)
    ];
    let mut color = None;
    
    while color.is_none() {
        println!("Available colors:");
        for (color_name, _) in color_choices.iter().filter(|(_, c)| !existing_colors.contains(c)) {
            println!("{}", color_name);
        }
        println!("Choose a color for the player:");
        let mut color_input = String::new();
        io::stdin().read_line(&mut color_input).expect("Failed to read line");
        
        color = color_choices.iter()
                             .find(|(name, _)| *name == color_input.trim())
                             .map(|&(_, c)| c);

        if let Some(c) = color {
            if existing_colors.contains(&c) {
                println!("This color is already taken, please choose another one.");
                color = None; 
            }
        } else {
            println!("Invalid color, please choose from the available colors.");
        }
    }
    color.unwrap()
}