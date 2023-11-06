// src/lib.rs

// Colors available for pawns
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

// Representation of a pawn in the game
pub struct Pawn {
    pub color: Color,
    pub position: u8, // Position on the board
}

// A player in the game, with four pawns
pub struct Player {
    pub pawns: [Pawn; 4],
}

// The game board
pub struct Board {

}
