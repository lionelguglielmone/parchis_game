// src/game.rs
use crate::Player;
use rand::Rng;
use std::io;

pub struct Board {
    
}

impl Board {
    
    pub fn new() -> Self {
        Board {
            
        }
    }

    
}

#[allow(dead_code)]
pub struct Move {
    player_name: String,
    pawn_index: usize,
    positions_advanced: u8,
    final_position: u8,
}

pub struct Game {
    pub players: Vec<Player>,
    pub board: Board,
    pub current_turn: usize,
    pub move_log: Vec<Move>,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Game {
        Game {
            players,
            board: Board::new(),
            current_turn: 0,
            move_log: Vec::new(),
        }
    }

    fn roll_dice(&self) -> u8 {
        rand::thread_rng().gen_range(1..=6)
    }

    pub fn record_move(&mut self, player_name: &str, pawn_id: u8, dice_roll: u8, final_position: u8) {
        let move_record = Move {
            player_name: player_name.to_string(),
            pawn_index: pawn_id as usize,
            positions_advanced: dice_roll,
            final_position,
        };
    
        self.move_log.push(move_record);
    }

    pub fn play_turn(&mut self) {
        let dice_roll = self.roll_dice();
        println!("Player {}'s turn. Rolled a {}", self.players[self.current_turn].name, dice_roll);
    
        println!("Select a pawn to move (enter a number 1-4): ");
        let mut pawn_selection = String::new();
        io::stdin().read_line(&mut pawn_selection).expect("Failed to read line");
        let pawn_id: u8 = pawn_selection.trim().parse().expect("Please enter a valid number.");
    
        let player_name = self.players[self.current_turn].name.clone();
        let pawns = &mut self.players[self.current_turn].pawns;
        if let Some(pawn) = pawns.iter_mut().find(|p| p.id == pawn_id) {
            let previous_position = pawn.position;
            let final_position = previous_position.saturating_add(dice_roll as u8).min(68);
            pawn.position = final_position;
            println!("Pawn {} moved from position {} to {}", pawn_id, previous_position, final_position);
    
            self.record_move(&player_name, pawn_id, dice_roll, final_position); 
        } else {
            println!("Invalid pawn selected. Try again.");
            return; 
        }
    
        self.next_turn(); 
    }

    
    pub fn next_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.players.len();
        println!("Next turn: Player {}", self.players[self.current_turn].name);
    }
    
    pub fn game_has_winner(&self) -> bool {
        self.players.iter().any(|player| {
            player.pawns.iter().all(|pawn| pawn.position >= 68)
        })
    }
}
