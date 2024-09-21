use crate::{
    game::Movement,
    piece::{Color, Piece},
};
extern crate termion;
use termion::{color, style};

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
        self.board[0][0] = Piece::Rook(Color::Black);
        self.board[0][1] = Piece::Knight(Color::Black);
        self.board[0][2] = Piece::Bishop(Color::Black);
        self.board[0][3] = Piece::King(Color::Black);
        self.board[0][4] = Piece::Queen(Color::Black);
        self.board[0][5] = Piece::Bishop(Color::Black);
        self.board[0][6] = Piece::Knight(Color::Black);
        self.board[0][7] = Piece::Rook(Color::Black);

        // White pieces
        self.board[7][0] = Piece::Rook(Color::White);
        self.board[7][1] = Piece::Knight(Color::White);
        self.board[7][2] = Piece::Bishop(Color::White);
        self.board[7][3] = Piece::King(Color::White);
        self.board[7][4] = Piece::Queen(Color::White);
        self.board[7][5] = Piece::Bishop(Color::White);
        self.board[7][6] = Piece::Knight(Color::White);
        self.board[7][7] = Piece::Rook(Color::White);

        // Pawns
        for i in 0..8 {
            self.board[1][i] = Piece::Pawn(Color::Black);
            self.board[6][i] = Piece::Pawn(Color::White);
        }
    }

    // prints the board to the terminal
    pub fn print(&self) {
        print_number_row();
        for i in 0..8 {
            print!(
                "{}{} {} {}{}",
                style::Invert,
                color::Fg(color::LightCyan),
                get_letter(i),
                color::Fg(color::Reset),
                style::Reset
            );
            for j in 0..8 {
                if j == 0 {
                    print!(
                        "{}{}{}|{}{}",
                        style::Underline,
                        color::Fg(color::LightBlack),
                        color::Bg(color::LightWhite),
                        color::Bg(color::Reset),
                        style::Reset
                    );
                }
                print!(
                    "{}{}{}{}{}{}",
                    color::Fg(color::Black),
                    style::Underline,
                    color::Bg(color::LightWhite),
                    self.board[i][j].to_emoji(),
                    color::Bg(color::Reset),
                    style::Reset
                );

                print!(
                    "{}{}{}|{}{}",
                    style::Underline,
                    color::Fg(color::LightBlack),
                    color::Bg(color::LightWhite),
                    color::Bg(color::Reset),
                    style::Reset
                );
            }
            print!(
                "{}{} {} {}{}",
                style::Invert,
                color::Fg(color::LightCyan),
                get_letter(i),
                color::Fg(color::Reset),
                style::Reset
            );
            if i == 0 {
                let white_captured: Vec<&Piece> = self
                    .captured_pieces
                    .iter()
                    .filter(|&&x| x.get_color().unwrap() == Color::Black)
                    .collect();
                print!("\tWhite captured pieces: ");
                for i in white_captured {
                    print!("{}", i.to_emoji());
                }
            } else if i == 7 {
                let black_captured: Vec<&Piece> = self
                    .captured_pieces
                    .iter()
                    .filter(|&&x| x.get_color().unwrap() == Color::White)
                    .collect();
                print!("\tBlack captured pieces: ");
                for i in black_captured {
                    print!("{}", i.to_emoji());
                }
            }
            println!();
        }
        print_number_row();
    }

    pub fn move_piece(&mut self, mv: &Movement) {
        let piece = self.board[mv.y][mv.x].clone();
        self.board[mv.y][mv.x] = Piece::Empty;
        if self.board[mv.dy][mv.dx] != Piece::Empty {
            self.captured_pieces.push(self.board[mv.dy][mv.dx].clone());
        }
        self.board[mv.dy][mv.dx] = piece;
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

fn get_letter(y: usize) -> String {
    let letter = match y {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => panic!("Incorrect coordinates!"),
    };
    letter.to_string()
}

fn print_number_row() {
    for i in 0..=8 {
        if i == 0 {
            print!(
                "{}{}    {}{}",
                style::Invert,
                color::Fg(color::LightCyan),
                color::Fg(color::Reset),
                style::Reset
            );
            continue;
        }
        print!(
            "{}{}{} {}{}",
            style::Invert,
            color::Fg(color::LightCyan),
            i,
            color::Fg(color::Reset),
            style::Reset
        );
    }
    print!(
        "{}{}   {}{}",
        style::Invert,
        color::Fg(color::LightCyan),
        color::Fg(color::Reset),
        style::Reset
    );
    println!()
}
