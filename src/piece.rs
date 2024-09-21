#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Empty,
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub fn to_emoji(&self) -> String {
        let emoji = match self {
            Piece::Empty => " ".to_string(),
            Piece::Pawn(Color::White) => "♙".to_string(),
            Piece::Pawn(Color::Black) => "♟".to_string(),
            Piece::Rook(Color::White) => "♖".to_string(),
            Piece::Rook(Color::Black) => "♜".to_string(),
            Piece::Knight(Color::White) => "♘".to_string(),
            Piece::Knight(Color::Black) => "♞".to_string(),
            Piece::Bishop(Color::White) => "♗".to_string(),
            Piece::Bishop(Color::Black) => "♝".to_string(),
            Piece::Queen(Color::White) => "♕".to_string(),
            Piece::Queen(Color::Black) => "♛".to_string(),
            Piece::King(Color::White) => "♔".to_string(),
            Piece::King(Color::Black) => "♚".to_string(),
        };
        emoji
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
