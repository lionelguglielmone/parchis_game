pub mod game; 
pub mod input_helpers; 


#[derive(Copy, Clone, Eq, PartialEq, Hash)] 
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

pub struct Pawn {
    pub id: u8,
    pub color: Color,
    pub position: u8,
}


pub struct Player {
    pub pawns: [Pawn; 4],
    pub name: String,
}
