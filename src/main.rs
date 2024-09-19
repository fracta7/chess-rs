use board::{get_xy, Board};

mod board;
mod piece;

fn main() {
    let mut board = Board::new();
    board.init_new();
    board.print();
    let tuple = get_xy("a1");
    let can_move = board.board[0][0].can_move(&board, tuple.0, tuple.1);
    println!("{}", can_move);
    println!("{}{}", tuple.0, tuple.1);
}
