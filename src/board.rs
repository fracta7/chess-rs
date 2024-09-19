use crate::piece::{Color, Piece};

#[derive(Debug)]
pub struct Board {
    pub board: Vec<Vec<Piece>>,
}

impl Board {
    // initializes empty board
    pub fn new() -> Self {
        let board = Board {
            board: vec![vec![Piece::Empty; 8]; 8],
        };

        return board;
    }

    // sets up the board with default position
    pub fn init_new(&mut self) {
        // Set up Black pieces
        self.board[0][0] = Piece::Rook(Color::Black);
        self.board[0][1] = Piece::Knight(Color::Black);
        self.board[0][2] = Piece::Bishop(Color::Black);
        self.board[0][3] = Piece::King(Color::Black);
        self.board[0][4] = Piece::Queen(Color::Black);
        self.board[0][5] = Piece::Bishop(Color::Black);
        self.board[0][6] = Piece::Knight(Color::Black);
        self.board[0][7] = Piece::Rook(Color::Black);

        for i in 0..8 {
            self.board[1][i] = Piece::Pawn(Color::Black);
        }

        // set up empty slots
        for i in 2..6 {
            for j in 0..8 {
                self.board[i][j] = Piece::Empty;
            }
        }

        // set up white pieces
        for i in 0..8 {
            self.board[6][i] = Piece::Pawn(Color::White);
        }

        self.board[7][0] = Piece::Rook(Color::White);
        self.board[7][1] = Piece::Knight(Color::White);
        self.board[7][2] = Piece::Bishop(Color::White);
        self.board[7][3] = Piece::King(Color::White);
        self.board[7][4] = Piece::Queen(Color::White);
        self.board[7][5] = Piece::Bishop(Color::White);
        self.board[7][6] = Piece::Knight(Color::White);
        self.board[7][7] = Piece::Rook(Color::White);
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
