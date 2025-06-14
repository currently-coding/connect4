use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use std::panic;

#[derive(Clone)]
pub struct Board {
    pub board: [u64; 2], // just need 42 bits
    pub active: usize,
    pub hash: u32,
    zobrist_table: [[u32; (COLS * (ROWS + 1)) as usize]; 2],
    pub col_height: [u8; COLS as usize],
}

pub const COLS: u8 = 7;
pub const ROWS: u8 = 6;

impl Board {
    pub fn new() -> Self {
        println!("Creating new Board.");
        Board {
            board: [0, 0],
            active: 0,
            hash: 0,
            zobrist_table: init_zobrist_table(),
            col_height: [0u8; COLS as usize],
        }
    }

    pub fn make_move(&mut self, col: u8) -> u8 {
        if self.put(col) == 0 {
            return 0;
        }
        self.active ^= 1;
        1
    }

    pub fn occupancy(&self) -> u64 {
        self.board[0] | self.board[1]
    }

    pub fn unmake_move(&mut self, col: u8) {
        self.active ^= 1;
        self.remove(col);
    }

    fn put(&mut self, col: u8) -> u8 {
        let fill = self.col_height[col as usize];
        if fill == ROWS {
            return 0;
        }
        let square = COLS * col + fill;
        self.hash ^= self.zobrist_table[self.active][square as usize];
        let target_mask: u64 = 1u64 << square;
        self.board[self.active] ^= target_mask;
        self.col_height[col as usize] += 1;
        1
    }

    fn remove(&mut self, col: u8) {
        let fill = self.col_height[col as usize];
        if fill == 0 {
            panic!("cannot remove piece from empty column");
        }
        let square = COLS * col + fill - 1;
        self.hash ^= self.zobrist_table[self.active][square as usize];
        let target_mask: u64 = 1u64 << square;
        self.board[self.active] ^= target_mask;
        self.col_height[col as usize] -= 1;
    }

    pub fn print(&self) {
        let mut tile;
        for row in (0..COLS - 1).rev() {
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

    pub fn draw(&self) -> bool {
        self.occupancy() >= 0b0111111011111101111110111111011111101111110111111u64
    }

    pub fn game_over(&self, side: usize) -> bool {
        // undo active switch by make_move
        let bb = self.board[side];
        // vertical
        (bb & bb << 8 & bb << 16 & bb << 24) > 0
        // horizontal
        || (bb & bb << 1 & bb << 2 & bb << 3) > 0
        // left diagonal 
        || (bb & bb << 7 & bb << 14 & bb << 21) > 0
        // right diagonal
        || (bb & bb << 6 & bb << 12 & bb << 18) > 0
    }

    // fn get_col_fill(&self, col: u8) -> u8 {
    //     if !(0..=ROWS).contains(&col) {
    //         panic!("Column out of range.")
    //     }
    //     let mask: u64 = 0b0111111u64 << (col * COLS);
    //     let mut bb: u64 = self.occupancy();
    //     bb &= mask;
    //     let leading_zeros = bb.leading_zeros();
    //     if leading_zeros == 64 {
    //         return 0;
    //     }
    //     let fill: u8 = (64 - leading_zeros - bb.trailing_zeros()) as u8;
    //     fill
    // }
    //
    // pub fn get_moves(&self) -> Vec<u8> {
    //     let mut moves = Vec::new();
    //     for col in [3, 2, 4, 1, 5, 0, 6] {
    //         if self.get_col_fill(col) != ROWS {
    //             moves.push(col);
    //         }
    //     }
    //     moves
    // }
}

fn init_zobrist_table() -> [[u32; (COLS * (ROWS + 1)) as usize]; 2] {
    let seed: [u8; 32] = [0; 32];
    let mut result = [[0u32; (COLS * (ROWS + 1)) as usize]; 2];
    let mut rng = ChaChaRng::from_seed(seed);
    for side in result.iter_mut() {
        for square in 0..side.len() {
            if square >= 42 {
                side[square] = side[square - 42];
            } else if square >= 35 {
                side[square] = side[square - 28];
            } else if square >= 28 {
                side[square] = side[square - 14];
            } else {
                side[square] = rng.next_u32();
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_zobrist_make_unmake() {
        let mut board = Board::new();
        board.board[board.active] = 0b1111 << 13;
        let copy = board.hash;
        board.make_move(5);
        board.unmake_move(5);
        assert_eq!(board.hash, copy);
    }

    #[test]
    fn test_symmetrical_zobrist_moves() {
        let mut board = Board::new();
        let hash1 = board.hash;
        board.make_move(0);
        let bb = board.board[0];
        let hash2 = board.hash;
        board.unmake_move(0);
        assert_eq!(hash1, board.hash);
        board.make_move(6);
        println!("{:064b}", board.board[0]);
        println!("{:064b}", bb);
        assert_eq!(board.hash, hash2);
    }

    #[test]
    fn test_symmetrical_zobrist_squares() {
        let board = Board::new();
        assert_eq!(board.zobrist_table[0][29], board.zobrist_table[0][15]);
        assert_eq!(board.zobrist_table[0][38], board.zobrist_table[0][10]);
        assert_eq!(board.zobrist_table[0][38], board.zobrist_table[0][10]);
        assert_eq!(board.zobrist_table[0][43], board.zobrist_table[0][1]);
        assert_eq!(board.zobrist_table[0][42], board.zobrist_table[0][0]);
    }

    #[test]
    fn test_game_over() {
        let mut board = Board::new();
        for _ in 0..4 {
            board.put(2);
        }
        // end move by switching sides - usually make_move handles that
        board.active ^= 1;
        assert!(board.game_over(board.active));
        let mut board = Board::new();
        for a in 0..4 {
            board.put(a);
        }
        // end move by switching sides - usually make_move handles that
        board.active ^= 1;
        assert!(board.game_over(board.active));
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
        board.make_move(0);
        assert!(board.game_over(board.active));
    }

    #[test]
    fn test_invalid_moves() {
        let mut board = Board::new();
        for _ in 0..6 {
            board.make_move(3);
            board.make_move(1);
        }
        board.unmake_move(1);
        board.print();
        let bb = board.board;
        let active = board.active;
        let hash = board.hash;
        board.make_move(3);
        assert_eq!(board.active, active);
        assert_eq!(board.hash, hash);
        assert_eq!(board.board, bb);
    }
}
