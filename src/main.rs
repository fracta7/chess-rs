#![allow(warnings)]
use board::Board;
use input::input_loop;
mod board;
mod game;
mod input;
mod output;
mod piece;
mod test;

fn main() {
    let mut board = Board::new();
    board.init_new();
    input_loop(&mut board);
}
