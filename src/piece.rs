#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Empty = 0,
    Pawn(Color) = 1,
    Rook(Color) = 2,
    Knight(Color) = 3,
    Bishop(Color) = 4,
    Queen(Color) = 5,
    King(Color) = 6,
}

impl Piece {
    pub fn to_number(&self) -> u8 {
        match self {
            Piece::Empty => 0,
            Piece::Pawn(_) => 1,
            Piece::Rook(_) => 2,
            Piece::Knight(_) => 3,
            Piece::Bishop(_) => 4,
            Piece::Queen(_) => 5,
            Piece::King(_) => 6,
        }
    }
    pub fn get_color(&self) -> Option<Color> {
        match self {
            Piece::Pawn(color) => Some(*color),
            Piece::Rook(color) => Some(*color),
            Piece::Knight(color) => Some(*color),
            Piece::Bishop(color) => Some(*color),
            Piece::Queen(color) => Some(*color),
            Piece::King(color) => Some(*color),
            Piece::Empty => None,
        }
    }
}
