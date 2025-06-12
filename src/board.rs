use std::panic;

pub struct Board {
    pub board: [u64; 2], // just need 42 bits
    pub active: usize,
    pub moves: [u8; 7],
}

pub const COLS: u8 = 7;
pub const ROWS: u8 = 6;

impl Board {
    pub fn new() -> Self {
        Board {
            board: [0, 0],
            active: 0,
            moves: [0, 1, 2, 3, 4, 5, 6],
        }
    }

    pub fn make_move(&mut self, col: u8) {
        self.put(col);
        self.active ^= 1;
    }

    pub fn occupancy(&self) -> u64 {
        self.board[0] | self.board[1]
    }

    pub fn unmake_move(&mut self, col: u8) {
        self.active ^= 1;
        self.remove(col);
    }

    fn put(&mut self, col: u8) {
        let fill = self.get_col_fill(col);
        if fill == ROWS {
            panic!("cannot put piece into full column");
        }
        let target_mask: u64 = 1u64 << (COLS * col + fill);
        self.board[self.active] ^= target_mask;
    }

    fn remove(&mut self, col: u8) {
        let fill = self.get_col_fill(col);
        if fill == 0 {
            panic!("cannot remove piece from empty column");
        }
        let target_mask: u64 = 1u64 << (COLS * col + fill - 1);
        self.board[self.active] ^= target_mask;
    }

    pub fn print(&self) {
        let mut tile;
        for row in (0..COLS).rev() {
            for col in 0..COLS {
                tile = COLS * col + row;
                let mask = 1u64 << tile;
                if mask & self.board[0] > 0 {
                    print!("X");
                } else if mask & self.board[1] > 0 {
                    print!("O");
                } else {
                    print!("_")
                }
                print!("|");
            }
            println!()
        }
    }

    pub fn game_over(&self) -> bool {
        let bb = self.board[self.active ^ 1];
        // vertical
        (bb & bb << 8 & bb << 16 & bb << 24) > 0
        // horizontal
        || (bb & bb << 1 & bb << 2 & bb << 3) > 0
        // left diagonal 
        || (bb & bb << 7 & bb << 14 & bb << 21) > 0
        // right diagonal
        || (bb & bb << 6 & bb << 12 & bb << 18) > 0
    }

    fn get_col_fill(&self, col: u8) -> u8 {
        if !(0..=ROWS).contains(&col) {
            panic!("Column out of range.")
        }
        let mask: u64 = 0b0111111u64 << (col * COLS);
        let mut bb: u64 = self.occupancy();
        bb &= mask;
        let leading_zeros = bb.leading_zeros();
        if leading_zeros == 64 {
            return 0;
        }
        let fill: u8 = (64 - leading_zeros - bb.trailing_zeros()) as u8;
        fill
    }

    pub(crate) fn get_moves(&self) -> Vec<u8> {
        let mut moves = Vec::new();
        for col in 0..7 {
            if self.get_col_fill(col) != ROWS {
                moves.push(col);
            }
        }
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_col_fill() {
        let mut board = Board::new();
        assert_eq!(board.get_col_fill(0), 0);
        assert_eq!(board.get_col_fill(1), 0);
        assert_eq!(board.get_col_fill(2), 0);
        assert_eq!(board.get_col_fill(3), 0);
        assert_eq!(board.get_col_fill(4), 0);
        assert_eq!(board.get_col_fill(5), 0);
        board.board[0] = 7;
        assert_eq!(board.get_col_fill(0), 3);
        board.board[0] = 0b11110111111;
        println!("{:064b}", board.board[0]);
        assert_eq!(board.get_col_fill(1), 4);
        board.board[0] = 0b1111 << 7;
        println!("{:064b}", board.board[0]);
        assert_eq!(board.get_col_fill(1), 4);
        board.board[0] = u64::MAX;
        assert_eq!(board.get_col_fill(2), 6);
        assert_eq!(board.get_col_fill(4), 6);
        assert_eq!(board.get_col_fill(6), 6);
        assert_eq!(board.get_col_fill(0), 6);
    }

    #[test]
    fn test_put() {
        let mut board = Board::new();
        board.board[0] = 0;
        board.put(0);
        assert_eq!(board.board[0], 0b1);
        board.put(0);
        assert_eq!(board.board[0], 0b11);
        board.board[0] = 0b1111 << 14;
        println!("{:064b}", board.board[0]);
        board.put(2);
        println!("{:064b}", board.board[0]);
        assert_eq!(board.board[0], 0b11111 << 14);
        board.board[0] = 1u64 << 7;
        board.put(1);
        assert_eq!(board.board[0], 0b11 << 7);
    }

    #[test]
    fn test_remove() {
        let mut board = Board::new();
        board.board[0] = 0b1111 << 13;
        board.remove(2);
        assert_eq!(board.board[0], 0b111 << 13);
    }

    #[test]
    fn test_make_unmake_move() {
        let mut board = Board::new();
        board.board[board.active] = 0b1111 << 13;
        let copy = board.board[board.active];
        board.make_move(2);
        board.unmake_move(2);
        assert_eq!(board.board[0], copy);
    }

    #[test]
    fn test_game_over() {
        let mut board = Board::new();
        for _ in 0..4 {
            board.put(2);
        }
        assert!(board.game_over());
        let mut board = Board::new();
        for a in 0..4 {
            board.put(a);
        }
        assert!(board.game_over());
        let mut board = Board::new();
        board.make_move(0);
        board.make_move(1);
        board.make_move(1);
        board.make_move(3);
        board.make_move(2);
        board.make_move(2);
        board.make_move(2);
        board.make_move(3);
        board.make_move(0);
        board.make_move(3);
        board.make_move(3);
        board.make_move(1);
        board.print();
        assert!(board.game_over());
    }
}
