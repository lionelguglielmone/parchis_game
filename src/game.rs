// src/game.rs
use super::{Player, Pawn, Color}; 

pub struct Board {

}

impl Board {
    pub fn new() -> Board {
        Board {

        }
    }

}

pub struct Game {
    pub players: Vec<Player>,
    pub board: Board,
    pub current_turn: usize,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Game {
        Game {
            players,
            board: Board::new(),
            current_turn: 0,
        }
    }

    pub fn next_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.players.len();
    }
    
    // TODO: methods for the game logic
}
