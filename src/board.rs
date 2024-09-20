use crate::{
    game::Movement,
    piece::{Color, Piece},
};

#[derive(Debug, Clone)]
pub struct Board {
    pub board: Vec<Vec<Piece>>,
    pub captured_pieces: Vec<Piece>,
}

impl Board {
    // initializes empty board
    pub fn new() -> Self {
        let board = Board {
            board: vec![vec![Piece::Empty; 8]; 8],
            captured_pieces: vec![],
        };

        return board;
    }

    // sets up the board with default position
    pub fn init_new(&mut self) {
        // Black pieces
        let back_rank_black = [
            Piece::Rook(Color::Black),
            Piece::Knight(Color::Black),
            Piece::Bishop(Color::Black),
            Piece::King(Color::Black),
            Piece::Queen(Color::Black),
            Piece::Bishop(Color::Black),
            Piece::Knight(Color::Black),
            Piece::Rook(Color::Black),
        ];
        let back_rank_white = [
            Piece::Rook(Color::White),
            Piece::Knight(Color::White),
            Piece::Bishop(Color::White),
            Piece::King(Color::White),
            Piece::Queen(Color::White),
            Piece::Bishop(Color::White),
            Piece::Knight(Color::White),
            Piece::Rook(Color::White),
        ];

        self.board[0] = back_rank_black.to_vec();
        self.board[7] = back_rank_white.to_vec();

        for i in 0..8 {
            self.board[1][i] = Piece::Pawn(Color::Black);
            self.board[6][i] = Piece::Pawn(Color::White);
        }
    }

    // prints the board to the terminal
    pub fn print(&self) {
        for i in 0..8 {
            for j in 0..8 {
                print!("{}", self.board[i][j].to_number());
                if j != 7 {
                    print!("|");
                }
            }
            println!();
        }
    }

    pub fn move_piece(&mut self, mv: &Movement) {
        let piece = self.board[mv.x][mv.y].clone();
        self.board[mv.x][mv.y] = Piece::Empty;
        if self.board[mv.dx][mv.dy] != Piece::Empty {
            self.captured_pieces.push(self.board[mv.dx][mv.dy].clone());
        }
        self.board[mv.dx][mv.dy] = piece;
    }
}

// returns x and y coordinates based on letter and number coordinates
pub fn get_xy(xy: &str) -> (usize, usize) {
    let x_char = xy.chars().nth(0).unwrap();
    let y_char = xy.chars().nth(1).unwrap();
    let x = match x_char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => 8,
    };
    let y = match y_char {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => 8,
    };
    return (x, y);
}
