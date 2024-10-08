#![allow(warnings)]

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
    pub white_mv_history: Vec<Movement>,
    pub black_mv_history: Vec<Movement>,
}

impl Board {
    // initializes empty board
    pub fn new() -> Self {
        let board = Board {
            board: vec![vec![Piece::Empty; 8]; 8],
            captured_pieces: vec![],
            white_mv_history: vec![],
            black_mv_history: vec![],
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

    pub fn move_piece(&mut self, mv: &Movement) {
        let piece = self.board[mv.y][mv.x].clone();
        self.board[mv.y][mv.x] = Piece::Empty;
        if self.board[mv.dy][mv.dx] != Piece::Empty {
            self.captured_pieces.push(self.board[mv.dy][mv.dx].clone());
        }
        self.board[mv.dy][mv.dx] = piece;
    }
    pub fn add_mv(&mut self, mv: &Movement, is_white: bool) {
        if is_white {
            self.white_mv_history.push(mv.clone());
        } else {
            self.black_mv_history.push(mv.clone());
        }
    }
}

// Returns x and y coordinates based on letter and number coordinates
pub fn get_xy(xy: &str) -> (usize, usize) {
    if let Some(y_char) = xy.chars().nth(0) {
        if let Some(x_char) = xy.chars().nth(1) {
            let y = match y_char {
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
            let x = match x_char {
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
    }
    (8, 8) // Invalid coordinate
}
