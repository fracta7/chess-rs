use board::{get_xy, Board};
use game::Movement;
use std::io::{self, Write};
mod board;
mod game;
mod piece;

fn main() {
    let mut board = Board::new();
    board.init_new();
    clear_screen();
    loop {
        board.print();
        println!("Select a piece: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");
        let input = input.trim();
        let xy = get_xy(input);
        clear_screen();
        board.print();
        println!("Select where to move: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");
        let input = input.trim();
        let dxy = get_xy(input);
        let mv = Movement {
            x: xy.0,
            y: xy.1,
            dx: dxy.0,
            dy: dxy.1,
        };
        board.move_piece(&mv);
        clear_screen();
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
