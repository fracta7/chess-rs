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

pub fn can_move(board: &Board, mv: &Movement, is_white: bool) -> bool {
    match board.board[mv.x][mv.y] {
        Piece::Empty => return false,
        Piece::Pawn(_) => return can_pawn_move(board, mv, is_white),
        Piece::Rook(_) => return can_rook_move(board, mv, is_white),
        Piece::Knight(_) => return can_knight_move(board, mv, is_white),
        Piece::Bishop(_) => return can_bishop_move(board, mv, is_white),
        Piece::Queen(_) => return can_queen_move(board, mv, is_white),
        Piece::King(_) => return can_king_move(board, mv, is_white),
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
    let can_capture =
        forward_move && mv.dx.abs_diff(mv.x) == 1 && is_opponent_piece(board, mv, is_white);

    (legal_first_move && is_empty) || can_capture || (is_move_forward && is_empty)
}

fn can_rook_move(board: &Board, mv: &Movement, is_white: bool) -> bool {
    let is_empty = board.board[mv.dx][mv.dy] == Piece::Empty;
    let is_move_x = mv.dx == mv.x && mv.dy != mv.y;
    let is_move_y = mv.dy == mv.y && mv.dx != mv.x;
    let is_legal_move = is_move_x || is_move_y;
    let mut obstacle_found = false;

    if is_move_x {
        let (start, end) = if mv.y < mv.dy {
            (mv.y + 1, mv.dy)
        } else {
            (mv.dy + 1, mv.y)
        };
        for i in start..end {
            if board.board[mv.x][i] != Piece::Empty {
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
            if board.board[i][mv.y] != Piece::Empty {
                obstacle_found = true;
                break;
            }
        }
    }

    is_legal_move && !obstacle_found && (is_empty || is_opponent_piece(board, mv, is_white))
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

fn get_king_pos(board: &Board, is_white: bool) -> (usize, usize) {
    let mut pos = (0, 0);
    let king_piece = if is_white {
        Piece::King(Color::White)
    } else {
        Piece::King(Color::Black)
    };

    for i in 0..8 {
        for j in 0..8 {
            if board.board[i][j] == king_piece {
                pos = (i, j);
                break;
            }
        }
    }
    pos
}

fn is_check(board: &Board, is_white: bool) -> bool {
    let king_pos = get_king_pos(board, is_white);
    for i in 0..8 {
        for j in 0..8 {
            let mv = Movement {
                x: i,
                y: j,
                dx: king_pos.0,
                dy: king_pos.1,
            };
            // we inverse is_white to check for opponent pieces
            let can_capture_king = can_move(board, &mv, !is_white);
            if can_capture_king {
                return true;
            }
        }
    }
    false
}
