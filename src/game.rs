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
        Piece::Knight(_) => return true,
        Piece::Bishop(_) => return true,
        Piece::Queen(_) => return true,
        Piece::King(_) => return true,
    }
}

fn can_pawn_move(board: &Board, mv: &Movement, is_white: bool) -> bool {
    let is_empty = board.board[mv.dx][mv.dy] == Piece::Empty;
    let piece_color = board.board[mv.dx][mv.dy].get_color();

    let forward_move = if is_white { mv.y > mv.dy } else { mv.y < mv.dy };
    let is_first_move = (is_white && mv.y == 6) || (!is_white && mv.y == 1);
    let legal_first_move = is_first_move && mv.dx == mv.x && mv.dy.abs_diff(mv.y) == 2;
    let is_move_forward = forward_move && mv.dx == mv.x && mv.dy.abs_diff(mv.y) == 1;

    let opponent_piece = match piece_color {
        Some(color) => color != (if is_white { Color::White } else { Color::Black }),
        None => false,
    };

    let can_capture = forward_move && mv.dx.abs_diff(mv.x) == 1 && !is_empty && opponent_piece;
    let legal_move = is_move_forward && is_empty;

    (legal_first_move && is_empty) || can_capture || legal_move
}

fn can_rook_move(board: &Board, mv: &Movement, is_white: bool) -> bool {
    let is_empty = board.board[mv.dx][mv.dy] == Piece::Empty;
    let piece_color = board.board[mv.dx][mv.dy].get_color();

    let is_move_x = mv.dx == mv.x && mv.dy != mv.y;
    let is_move_y = mv.dy == mv.y && mv.dx != mv.x;
    let legal_move = is_move_x || is_move_y;

    let opponent_piece = match piece_color {
        Some(color) => color != (if is_white { Color::White } else { Color::Black }),
        None => false,
    };

    (legal_move && is_empty) || (legal_move && !is_empty && opponent_piece)
}
