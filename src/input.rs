#![allow(warnings)]

use std::io::{self, Write};

use crate::{
    board::{get_xy, Board},
    game::{can_move, is_checkmate, Movement},
    output::output,
    piece::Color,
};

pub fn input_loop(board: &mut Board) {
    let mut is_whites_move = true;
    loop {
        output(board, is_whites_move, true, None);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");
        let input = input.trim();
        if input == "q" {
            break;
        } else if input == "c" {
            continue;
        } else if input.is_empty() {
            continue;
        }
        let xy = get_xy(input);
        if xy.0 == 8 || xy.1 == 8 {
            continue;
        }
        if let Some(piece_color) = board.board[xy.1][xy.0].get_color() {
            if piece_color != Color::White && is_whites_move {
                println!("Not your piece!");
                continue;
            } else if piece_color != Color::Black && !is_whites_move {
                continue;
            }
        } else {
            // Handle selecting an empty space
            println!("No piece at this location!");
            continue;
        }
        output(board, is_whites_move, false, Some(xy));
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");
        let input = input.trim();
        if input == "q" {
            break;
        } else if input == "c" {
            continue;
        } else if input.is_empty() {
            continue;
        }
        let dxy = get_xy(input);
        if dxy.0 == 8 || dxy.1 == 8 {
            continue;
        }
        let mv = Movement {
            x: xy.0,
            y: xy.1,
            dx: dxy.0,
            dy: dxy.1,
        };
        let movable = can_move(&board, &mv, is_whites_move, false);
        if movable {
            board.move_piece(&mv);
            board.add_mv(&mv, is_whites_move);
            is_whites_move = !is_whites_move;
        }
        if is_checkmate(&board, !is_whites_move) {
            println!("{} wins!", if is_whites_move { "white" } else { "black" });
            break;
        }
    }
}
