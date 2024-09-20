use crate::{
    board::Board,
    piece::{Color, Piece},
};

#[derive(Debug)]
pub struct Movement {
    pub x: usize,
    pub y: usize,
    pub dx: usize,
    pub dy: usize,
}

pub fn can_move(board: &Board, mv: &Movement) -> bool {
    match board.board[mv.x][mv.y] {
        Piece::Empty => return false,
        Piece::Pawn(Color::White) => return can_pawn_move(board, mv, true),
        Piece::Pawn(Color::Black) => return can_pawn_move(board, mv, false),
        Piece::Rook(Color::White) => return can_rook_move(board, mv, true),
        Piece::Rook(Color::Black) => return can_rook_move(board, mv, false),
        Piece::Knight(Color::White) => return can_knight_move(board, mv, true),
        Piece::Knight(Color::Black) => return can_knight_move(board, mv, false),
        Piece::Bishop(Color::White) => return can_bishop_move(board, mv, true),
        Piece::Bishop(Color::Black) => return can_bishop_move(board, mv, false),
        Piece::Queen(Color::White) => return can_queen_move(board, mv, true),
        Piece::Queen(Color::Black) => return can_queen_move(board, mv, false),
        Piece::King(Color::White) => return can_king_move(board, mv, true),
        Piece::King(Color::Black) => return can_king_move(board, mv, false)
    }
}

fn is_opponent_piece(board: &Board, mv: &Movement, is_white: bool) -> bool {
    match board.board[mv.dx][mv.dy].get_color() {
        Some(color) => color != (if is_white { Color::White } else { Color::Black }),
        None => false,
    }
}

fn can_pawn_move(board: &Board, mv: &Movement, is_white: bool) -> bool {
    let is_empty = board.board[mv.dx][mv.dy] == Piece::Empty;
    let forward_move = if is_white { mv.y > mv.dy } else { mv.y < mv.dy };
    let is_first_move = (is_white && mv.y == 6) || (!is_white && mv.y == 1);
    let legal_first_move = is_first_move && mv.dx == mv.x && mv.dy.abs_diff(mv.y) == 2;
    let is_move_forward = forward_move && mv.dx == mv.x && mv.dy.abs_diff(mv.y) == 1;
    let can_capture = forward_move && mv.dx.abs_diff(mv.x) == 1 && is_opponent_piece(board, mv, is_white);

    (legal_first_move && is_empty) || can_capture || (is_move_forward && is_empty)
}

fn can_rook_move(board: &Board, mv: &Movement, is_white: bool) -> bool {
    let is_empty = board.board[mv.dx][mv.dy] == Piece::Empty;
    let is_move_x = mv.dx == mv.x && mv.dy != mv.y;
    let is_move_y = mv.dy == mv.y && mv.dx != mv.x;
    let legal_move = is_move_x || is_move_y;

    legal_move && (is_empty || is_opponent_piece(board, mv, is_white))
}


fn can_knight_move(board: &Board, mv: &Movement, is_white: bool) -> bool {
    let is_empty = board.board[mv.dx][mv.dy] == Piece::Empty;
    let move_x = mv.dx.abs_diff(mv.x);
    let move_y = mv.dy.abs_diff(mv.y);
    let legal_move = (move_x == 2 && move_y == 1) || (move_x == 1 && move_y == 2);

    legal_move && (is_empty || is_opponent_piece(board, mv, is_white))
}

fn can_bishop_move(board: &Board, mv: &Movement, is_white: bool) -> bool {
    let is_empty = board.board[mv.dx][mv.dy] == Piece::Empty;
    let legal_move = mv.dx.abs_diff(mv.x) == mv.dy.abs_diff(mv.y);

    legal_move && (is_empty || is_opponent_piece(board, mv, is_white))
}

fn can_queen_move(board: &Board, mv: &Movement, is_white: bool) -> bool {
    can_rook_move(board, mv, is_white) || can_bishop_move(board, mv, is_white)
}

fn can_king_move(board: &Board, mv: &Movement, is_white: bool) -> bool {
    let is_empty = board.board[mv.dx][mv.dy] == Piece::Empty;
    let legal_move = mv.dx.abs_diff(mv.x) <= 1 && mv.dy.abs_diff(mv.y) <= 1;

    legal_move && (is_empty || is_opponent_piece(board, mv, is_white))
}