#![allow(warnings)]

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

pub fn can_move(board: &Board, mv: &Movement, is_white: bool, no_check: bool) -> bool {
    match board.board[mv.y][mv.x] {
        Piece::Empty => return false,
        Piece::Pawn(_) => return can_pawn_move(board, mv, is_white, no_check),
        Piece::Rook(_) => return can_rook_move(board, mv, is_white, no_check),
        Piece::Knight(_) => return can_knight_move(board, mv, is_white, no_check),
        Piece::Bishop(_) => return can_bishop_move(board, mv, is_white, no_check),
        Piece::Queen(_) => return can_queen_move(board, mv, is_white, no_check),
        Piece::King(_) => return can_king_move(board, mv, is_white, no_check),
    }
}

fn is_opponent_piece(board: &Board, mv: &Movement, is_white: bool) -> bool {
    match board.board[mv.dy][mv.dx].get_color() {
        Some(color) => color != (if is_white { Color::White } else { Color::Black }),
        None => false,
    }
}

fn can_pawn_move(board: &Board, mv: &Movement, is_white: bool, no_check: bool) -> bool {
    let is_empty = board.board[mv.dy][mv.dx] == Piece::Empty;
    let forward_move = if is_white { mv.y > mv.dy } else { mv.y < mv.dy };
    let is_first_move = (is_white && mv.y == 6) || (!is_white && mv.y == 1);
    let legal_first_move = is_first_move && mv.dx == mv.x && mv.dy.abs_diff(mv.y) == 2;
    let mut obstacles_found = false;
    if legal_first_move && forward_move {
        let start = if is_white { mv.y - 1 } else { mv.y + 1 };
        for i in start..mv.dy {
            if board.board[i][mv.x] != Piece::Empty {
                obstacles_found = true;
                break;
            }
        }
    }
    let is_move_forward = forward_move && mv.dx == mv.x && mv.dy.abs_diff(mv.y) == 1;
    let can_capture = forward_move
        && mv.dy.abs_diff(mv.y) == 1
        && mv.dx.abs_diff(mv.x) == 1
        && is_opponent_piece(board, mv, is_white);
    let can_move = (legal_first_move && is_empty && !obstacles_found)
        || can_capture
        || (is_move_forward && is_empty);
    let mut will_result_check = false;

    if can_move && !no_check {
        let mut temp_board = board.clone();
        temp_board.move_piece(mv);
        will_result_check = is_check(&temp_board, is_white);
    }

    can_move && !will_result_check
}

fn is_obstacle_rook(board: &Board, mv: &Movement, is_move_x: bool, is_move_y: bool) -> bool {
    let mut obstacle_found = false;

    if is_move_x {
        let (start, end) = if mv.y < mv.dy {
            (mv.y + 1, mv.dy)
        } else {
            (mv.dy + 1, mv.y)
        };
        for i in start..end {
            if board.board[i][mv.x] != Piece::Empty {
                obstacle_found = true;
                break;
            }
        }
    } else if is_move_y {
        let (start, end) = if mv.x < mv.dx {
            (mv.x + 1, mv.dx)
        } else {
            (mv.dx + 1, mv.x)
        };
        for i in start..end {
            if board.board[mv.y][i] != Piece::Empty {
                obstacle_found = true;
                break;
            }
        }
    }
    obstacle_found
}

fn can_rook_move(board: &Board, mv: &Movement, is_white: bool, no_check: bool) -> bool {
    let is_empty = board.board[mv.dy][mv.dx] == Piece::Empty;
    let is_move_x = mv.dx == mv.x && mv.dy != mv.y;
    let is_move_y = mv.dy == mv.y && mv.dx != mv.x;
    let is_legal_move = is_move_x || is_move_y;
    let obstacle_found = is_obstacle_rook(board, mv, is_move_x, is_move_y);
    let can_move =
        is_legal_move && !obstacle_found && (is_empty || is_opponent_piece(board, mv, is_white));
    let mut will_result_check = false;

    if can_move && !no_check {
        let mut temp_board = board.clone();
        temp_board.move_piece(mv);
        will_result_check = is_check(&temp_board, is_white);
    }

    can_move && !will_result_check
}

fn can_knight_move(board: &Board, mv: &Movement, is_white: bool, no_check: bool) -> bool {
    let is_empty = board.board[mv.dy][mv.dx] == Piece::Empty;
    let move_x = mv.dx.abs_diff(mv.x);
    let move_y = mv.dy.abs_diff(mv.y);
    let legal_move = (move_x == 2 && move_y == 1) || (move_x == 1 && move_y == 2);
    let can_move = legal_move && (is_empty || is_opponent_piece(board, mv, is_white));
    let mut will_result_check = false;
    if can_move && !no_check {
        let mut temp_board = board.clone();
        temp_board.move_piece(mv);
        will_result_check = is_check(&temp_board, is_white);
    }
    can_move && !will_result_check
}

fn is_obstacle_bishop(board: &Board, mv: &Movement) -> bool {
    let (step_x, step_y) = (
        if mv.dx > mv.x { 1 } else { -1 },
        if mv.dy > mv.y { 1 } else { -1 },
    );

    let (mut x, mut y) = (mv.x as isize, mv.y as isize);

    while (x != mv.dx as isize) && (y != mv.dy as isize) {
        x += step_x;
        y += step_y;

        if board.board[y as usize][x as usize] != Piece::Empty {
            return true; // Obstacle found
        }
    }
    false // No obstacles found
}

fn can_bishop_move(board: &Board, mv: &Movement, is_white: bool, no_check: bool) -> bool {
    let is_empty = board.board[mv.dy][mv.dx] == Piece::Empty;
    let legal_move = mv.dx.abs_diff(mv.x) == mv.dy.abs_diff(mv.y);
    let obstacle_found = is_obstacle_bishop(board, mv);
    let can_move =
        legal_move && !obstacle_found && (is_empty || is_opponent_piece(board, mv, is_white));
    let mut will_result_check = false;

    if can_move && !no_check {
        let mut temp_board = board.clone();
        temp_board.move_piece(mv);
        will_result_check = is_check(&temp_board, is_white);
    }

    can_move && !will_result_check
}

fn can_queen_move(board: &Board, mv: &Movement, is_white: bool, no_check: bool) -> bool {
    let can_move = can_rook_move(board, mv, is_white, no_check)
        || can_bishop_move(board, mv, is_white, no_check);
    let mut will_result_check = false;
    if can_move && !no_check {
        let mut temp_board = board.clone();
        temp_board.move_piece(mv);
        will_result_check = is_check(&temp_board, is_white);
    }
    can_move && !will_result_check
}

fn can_king_move(board: &Board, mv: &Movement, is_white: bool, no_check: bool) -> bool {
    let is_empty = board.board[mv.dy][mv.dx] == Piece::Empty;
    let legal_move = mv.dx.abs_diff(mv.x) <= 1 && mv.dy.abs_diff(mv.y) <= 1;
    let can_move = legal_move && (is_empty || is_opponent_piece(board, mv, is_white));
    let mut will_result_check = false;
    if can_move & !no_check {
        let mut temp_board = board.clone();
        temp_board.move_piece(mv);
        will_result_check = is_check(&temp_board, is_white);
    }

    can_move && !will_result_check
}

pub fn get_king_pos(board: &Board, is_white: bool) -> (usize, usize) {
    let mut pos = (0, 0);
    let king_piece = if is_white {
        Piece::King(Color::White)
    } else {
        Piece::King(Color::Black)
    };
    let mut king_found = false;

    for i in 0..8 {
        if king_found {
            break;
        }
        for j in 0..8 {
            if board.board[i][j] == king_piece {
                pos = (j, i);
                break;
            }
        }
    }
    pos
}

pub fn is_check(board: &Board, is_white: bool) -> bool {
    let king_pos = get_king_pos(board, is_white);
    for i in 0..8 {
        for j in 0..8 {
            let piece = board.board[i][j];
            let own = piece.get_color() == Some(if is_white { Color::White } else { Color::Black });

            if piece == Piece::Empty || own {
                continue;
            }
            let mv = Movement {
                x: j,
                y: i,
                dx: king_pos.0,
                dy: king_pos.1,
            };
            // we inverse is_white to check for opponent pieces
            let can_capture_king = can_move(board, &mv, !is_white, true);
            if can_capture_king {
                return true;
            }
        }
    }
    false
}

pub fn is_checkmate(board: &Board, is_white: bool) -> bool {
    if !is_check(board, is_white) {
        return false;
    }

    for i in 0..8 {
        for j in 0..8 {
            let piece = board.board[i][j];

            if piece == Piece::Empty
                || piece.get_color() != Some(if is_white { Color::White } else { Color::Black })
            {
                continue;
            }

            for dy in 0..8 {
                for dx in 0..8 {
                    let mv = Movement { x: j, y: i, dx, dy };

                    // Check if the move is valid
                    if can_move(board, &mv, is_white, false) {
                        // Simulate the move
                        let mut temp_board = board.clone();
                        temp_board.move_piece(&mv);

                        // If this move results in the player not being in check anymore, it's not checkmate
                        if !is_check(&temp_board, is_white) {
                            return false;
                        }
                    }
                }
            }
        }
    }

    // If no move can prevent the check, it is checkmate
    true
}
