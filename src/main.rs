use board::Board;
use game::Movement;

mod board;
mod game;
mod piece;

fn main() {
    let mut board = Board::new();
    board.init_new();
    board.print();
    let mv = Movement {
        x: 3,
        y: 6,
        dx: 3,
        dy: 4,
    };
    board.move_piece(&mv);
    println!();
    board.print();
}
