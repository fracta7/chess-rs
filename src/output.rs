use termion::{color, style};

use crate::{
    board::Board,
    piece::{Color, Piece},
};

impl Board {
    // prints the board to the terminal
    pub fn print(&self, xy: Option<(usize, usize)>, highlight: bool) {
        print_number_row();
        for i in 0..8 {
            print!(
                "{}{} {} {}{}",
                style::Invert,
                color::Fg(color::LightCyan),
                get_letter(i),
                color::Fg(color::Reset),
                style::Reset
            );
            for j in 0..8 {
                if j == 0 {
                    print!(
                        "{}{}{}|{}{}",
                        style::Underline,
                        color::Fg(color::LightBlack),
                        color::Bg(color::LightWhite),
                        color::Bg(color::Reset),
                        style::Reset
                    );
                }
                if xy.is_some() {
                    if xy.unwrap().1 == i && xy.unwrap().0 == j && highlight {
                        print!(
                            "{}{}{}{}{}{}{}",
                            color::Fg(color::Black),
                            style::Blink,
                            style::Underline,
                            color::Bg(color::LightWhite),
                            self.board[i][j].to_emoji(),
                            color::Bg(color::Reset),
                            style::Reset
                        );
                    } else {
                        print!(
                            "{}{}{}{}{}{}",
                            color::Fg(color::Black),
                            style::Underline,
                            color::Bg(color::LightWhite),
                            self.board[i][j].to_emoji(),
                            color::Bg(color::Reset),
                            style::Reset
                        );
                    }
                } else {
                    print!(
                        "{}{}{}{}{}{}",
                        color::Fg(color::Black),
                        style::Underline,
                        color::Bg(color::LightWhite),
                        self.board[i][j].to_emoji(),
                        color::Bg(color::Reset),
                        style::Reset
                    );
                }

                print!(
                    "{}{}{}|{}{}",
                    style::Underline,
                    color::Fg(color::LightBlack),
                    color::Bg(color::LightWhite),
                    color::Bg(color::Reset),
                    style::Reset
                );
            }
            print!(
                "{}{} {} {}{}",
                style::Invert,
                color::Fg(color::LightCyan),
                get_letter(i),
                color::Fg(color::Reset),
                style::Reset
            );
            if i == 0 {
                let white_captured: Vec<&Piece> = self
                    .captured_pieces
                    .iter()
                    .filter(|&&x| x.get_color().unwrap() == Color::Black)
                    .collect();
                print!("\t");
                for i in white_captured {
                    print!("{}", i.to_emoji());
                }
            } else if i == 7 {
                let black_captured: Vec<&Piece> = self
                    .captured_pieces
                    .iter()
                    .filter(|&&x| x.get_color().unwrap() == Color::White)
                    .collect();
                print!("\t");
                for i in black_captured {
                    print!("{}", i.to_emoji());
                }
            }
            println!();
        }
        print_number_row();
    }
}

pub fn output_loop(board: &Board, side: String, is_selection: bool, xy: Option<(usize, usize)>) {
    clear_screen();
    print_info();
    if is_selection {
        board.print(None, false);
        println!("Select a piece: {}", side);
    } else {
        board.print(xy, true);
        println!("Select where to move: {}", side);
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_info() {
    let game_info = "╔════════════════════════╗\n\
                           ║───┬Game info───────────║\n\
                           ║ c ┆ cancel selection   ║\n\
                           ║ q ┆ quit               ║\n\
                           ╚═══╧════════════════════╝";
    println!("{}", game_info);
}

fn get_letter(y: usize) -> String {
    let letter = match y {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => panic!("Incorrect coordinates!"),
    };
    letter.to_string()
}

fn print_number_row() {
    for i in 0..=8 {
        if i == 0 {
            print!(
                "{}{}    {}{}",
                style::Invert,
                color::Fg(color::LightCyan),
                color::Fg(color::Reset),
                style::Reset
            );
            continue;
        }
        print!(
            "{}{}{} {}{}",
            style::Invert,
            color::Fg(color::LightCyan),
            i,
            color::Fg(color::Reset),
            style::Reset
        );
    }
    print!(
        "{}{}   {}{}",
        style::Invert,
        color::Fg(color::LightCyan),
        color::Fg(color::Reset),
        style::Reset
    );
    println!()
}
