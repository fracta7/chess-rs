use board::{get_xy, Board};
use input::input_loop;
mod board;
mod game;
mod input;
mod output;
mod piece;

fn main() {
    let mut board = Board::new();
    board.init_new();
    input_loop(&mut board);
}
