use crate::board::Board;

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
    pub fn can_move(&self, board: &Board, x: usize, y: usize) -> bool {
        match board.board[x][y] {
            Piece::Empty => return false,
            Piece::Pawn(_) => return true,
            Piece::Rook(_) => return true,
            Piece::Knight(_) => return true,
            Piece::Bishop(_) => return true,
            Piece::Queen(_) => return true,
            Piece::King(_) => return true,
        }
    }
}
