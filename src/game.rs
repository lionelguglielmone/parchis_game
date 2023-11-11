// src/game.rs
use crate::{Player, Pawn};
use rand::Rng;
use std::io;
use crate::input_helpers::clear_screen;

#[derive(Debug)]
pub struct Move {
    player_name: String,
    pawn_index: usize,
    positions_advanced: u8,
    final_position: u8,
}

pub struct Game {
    pub players: Vec<Player>,
    pub current_turn: usize,
    pub move_log: Vec<Move>,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Game {
        Game {
            players,
            current_turn: 0,
            move_log: Vec::new(),
        }
    }


    pub fn play_turn(&mut self) {
        self.print_turn_banner();
    
        println!("Press Enter to roll the dice.");
        let _ = io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    
        let dice_roll = self.roll_dice();
        clear_screen();
        self.print_turn_banner();
        println!("You rolled a {}\n", dice_roll);
    
        if dice_roll == 6 {
            self.handle_six_roll();
        } else if !self.any_pawn_outside() {
            println!("No pawns can be moved. You need to roll a 6 to move a pawn out of the house!");
        } else {
            self.move_pawn_if_possible(dice_roll);
        }
    
        self.show_pawn_positions();
        self.next_turn();
    }

    fn roll_dice(&self) -> u8 {
        rand::thread_rng().gen_range(1..=6)
    }


    fn move_pawn_out_of_house(&mut self) {
        let player_name = self.players[self.current_turn].name.clone();
    
        let mut pawn_moved = false;
        let mut pawn_id_to_move = 0;
        for pawn in &mut self.players[self.current_turn].pawns {
            if pawn.position == 0 {
                pawn.position = 1;
                println!("Pawn {} moves out to start", pawn.id);
                pawn_moved = true;
                pawn_id_to_move = pawn.id; 
                break; 
            }
        }
    
        if pawn_moved {
            self.record_move(&player_name, pawn_id_to_move, 6, 1);
        }
    }

    fn move_pawn_if_possible(&mut self, dice_roll: u8) {
        let player_name = self.players[self.current_turn].name.clone();
    
        let mut pawn_moved = false;
        let mut move_record_details = (0, 0, 0);
    
        for pawn in &mut self.players[self.current_turn].pawns {
            if pawn.position > 0 && pawn.position < 68 {
                let old_position = pawn.position;
                pawn.position = (pawn.position as u8).saturating_add(dice_roll).min(68) as u8;
                println!("Pawn {} moves from position {} to {}", pawn.id, old_position, pawn.position);
                pawn_moved = true;
                move_record_details = (pawn.id, dice_roll, pawn.position);
                break;
            }
        }
    
        if pawn_moved {
            self.record_move(&player_name, move_record_details.0, move_record_details.1, move_record_details.2);
        }
    }

    fn record_move(&mut self, player_name: &str, pawn_id: u8, dice_roll: u8, final_position: u8) {
        let move_record = Move {
            player_name: player_name.to_string(),
            pawn_index: pawn_id as usize,
            positions_advanced: dice_roll,
            final_position: final_position,
        };

        self.move_log.push(move_record);
    }

    pub fn show_pawn_positions(&self) {
        let player = &self.players[self.current_turn];
        let header: Vec<String> = player.pawns.iter().enumerate().map(|(i, _)| format!("Pawn {}", i + 1)).collect();
        let positions: Vec<String> = player.pawns.iter().map(|p| if p.position == 0 { "House".to_owned() } else { p.position.to_string() }).collect();
        
        let max_length = positions.iter().map(|pos| pos.len()).max().unwrap_or(0).max(5);
        let divider = "+".repeat(header.len() * (max_length + 3) - 1);
        let header_row = header.iter().map(|title| format!("{:^width$}", title, width=max_length)).collect::<Vec<_>>().join(" | ");
        let position_row = positions.iter().map(|pos| format!("{:^width$}", pos, width=max_length)).collect::<Vec<_>>().join(" | ");
        
        println!("{}", divider);
        println!("{}", header_row);
        println!("{}", divider);
        println!("{}", position_row);
        println!("{}", divider);
        println!(); 
    }

    fn next_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.players.len();
    }

    pub fn game_has_winner(&self) -> bool {
        self.players.iter().any(|player| {
            player.pawns.iter().all(|pawn| pawn.position >= 68)
        })
    }

    pub fn print_turn_banner(&self) {
        clear_screen();
        let banner_text = format!(" Player {}'s Turn ", self.players[self.current_turn].name);
        let banner_border = "*".repeat(banner_text.len());

        println!("\n{}\n{}\n{}", banner_border, banner_text, banner_border);
    }

    fn handle_six_roll(&mut self) {
        println!("You rolled a six! You can move a pawn out of the house or move a pawn on the board.");
        println!("Do you want to move a pawn out of the house? (yes/no)");
    
        let mut decision = String::new();
        io::stdin().read_line(&mut decision).expect("Failed to read line");
    
        if decision.trim().eq_ignore_ascii_case("yes") && self.any_pawn_in_house() {
            self.move_pawn_out_of_house();
        } else {
            println!("Choose which pawn to move on the board:");
            self.move_pawn_if_possible(6); 
        }
    }
    
    fn any_pawn_outside(&self) -> bool {
        self.players[self.current_turn]
            .pawns
            .iter()
            .any(|p| p.position > 0)
    }

    fn any_pawn_in_house(&self) -> bool {
        self.players[self.current_turn].pawns.iter().any(|p| p.position == 0)
    }
}