#![allow(warnings)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::game::{get_king_pos, is_check, Movement};
    use crate::piece::{Color, Piece};
    const check: &str = "There should be a check.";
    const no_check: &str = "There shouldn't be a check.";

    #[test]
    fn test_is_check_simple() {
        // Create a new board and initialize it
        let mut board = Board::new();
        board.init_new();

        let mv = Movement {
            x: 3,
            y: 7,
            dx: 3,
            dy: 5,
        };
        board.move_piece(&mv);
        let mv = Movement {
            x: 4,
            y: 1,
            dx: 4,
            dy: 4,
        };
        board.move_piece(&mv);

        assert!(is_check(&board, true), "{}", check);
    }
    #[test]
    fn test_is_check_default() {
        // Create a new board and initialize it
        let mut board = Board::new();
        board.init_new();

        assert!(!is_check(&board, true), "{}", check);
    }

    #[test]
    fn test_is_check_simple2() {
        // Create a new board and initialize it
        let mut board = Board::new();
        board.init_new();

        let mv = Movement {
            x: 3,
            y: 6,
            dx: 4,
            dy: 5,
        };
        board.move_piece(&mv);
        let mv = Movement {
            x: 0,
            y: 0,
            dx: 3,
            dy: 2,
        };
        board.move_piece(&mv);

        assert!(is_check(&board, true), "{}", check);
    }
    #[test]
    fn test_is_check_simple3() {
        // Create a new board and initialize it
        let mut board = Board::new();
        board.init_new();

        let mv = Movement {
            x: 3,
            y: 6,
            dx: 4,
            dy: 5,
        };
        board.move_piece(&mv);
        let mv = Movement {
            x: 0,
            y: 0,
            dx: 4,
            dy: 2,
        };
        board.move_piece(&mv);

        assert!(!is_check(&board, true), "{}", no_check);
    }

    #[test]
    fn test_king_pos_white_default() {
        let mut board = Board::new();
        board.init_new();
        let pos = get_king_pos(&board, true);
        assert!(pos.0 == 3 && pos.1 == 7, "Not equal");
    }

    #[test]
    fn test_king_pos_black_default() {
        let mut board = Board::new();
        board.init_new();
        let pos = get_king_pos(&board, false);
        assert!(pos.0 == 3 && pos.1 == 0, "Not equal");
    }

    #[test]
    fn test_king_pos_moved1() {
        let mut board = Board::new();
        board.init_new();
        let mv = Movement {
            x: 3,
            y: 7,
            dx: 7,
            dy: 3,
        };
        board.move_piece(&mv);
        let pos = get_king_pos(&board, true);
        assert!(pos.0 == 7 && pos.1 == 3, "Not equal");
    }

    #[test]
    fn test_king_pos_moved2() {
        let mut board = Board::new();
        board.init_new();
        let mv = Movement {
            x: 3,
            y: 7,
            dx: 5,
            dy: 0,
        };
        board.move_piece(&mv);
        let pos = get_king_pos(&board, true);
        assert!(pos.0 == 5 && pos.1 == 0, "Not equal");
    }
}
