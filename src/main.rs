use board::{get_xy, Board};

mod board;
mod game;
mod piece;

fn main() {
    let mut board = Board::new();
    board.init_new();
    board.print();
}
