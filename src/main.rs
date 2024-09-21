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
        dy: 1,
    };
    board.move_piece(&mv);
    println!();
    board.print();
    let mv = Movement {
        x: 0,
        y: 0,
        dx: 4,
        dy: 7,
    };
    board.move_piece(&mv);
    println!();
    board.print();
}
