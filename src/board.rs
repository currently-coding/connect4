use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

#[derive(Clone)]
pub struct Board {
    pub board: [u64; 2], // just need 42 bits
    pub active: usize,
    pub hash: u32,
    zobrist_table: [[u32; NUM_CELLS as usize]; 2],
    pub col_height: [u8; COLS as usize],
}

pub const COLS: u8 = 7;
pub const ROWS: u8 = 6;
pub const BOARD_MASK: u64 = 0b0111111011111101111110111111011111101111110111111u64;
pub const NUM_CELLS: u8 = COLS * (ROWS + 1);

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

    pub fn occupancy(&self) -> u64 {
        self.board[0] | self.board[1]
    }
    pub fn make_move(&mut self, col: u8) -> bool {
        if !self.put(col) {
            return false;
        }
        self.active ^= 1;
        true
    }

    pub fn unmake_move(&mut self, col: u8) {
        self.active ^= 1;
        self.remove(col);
    }

    fn put(&mut self, col: u8) -> bool {
        let fill = self.col_height[col as usize];
        if fill == ROWS {
            return false;
        }
        self.col_height[col as usize] += 1;
        let square = COLS * col + fill;
        self.hash ^= self.zobrist_table[self.active][square as usize];
        self.board[self.active] ^= 1u64 << square;
        true
    }

    fn remove(&mut self, col: u8) {
        let square = COLS * col + self.col_height[col as usize] - 1;
        self.hash ^= self.zobrist_table[self.active][square as usize];
        self.board[self.active] ^= 1u64 << square;
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

    pub fn full(&self) -> bool {
        self.occupancy() >= BOARD_MASK
    }

    pub fn game_over(&self, side: usize) -> bool {
        let b = self.board[side];
        Self::check_win(b)
    }

    #[inline]
    fn check_win(bb: u64) -> bool {
        const DIRS: [u8; 4] = [1, 6, 7, 8];
        DIRS.iter().any(|&dir| {
            let m1 = bb & (bb >> dir);
            let m2 = m1 & (m1 >> (2 * dir));
            m2 != 0
        })
    }
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
        board.col_height[2] = 4;
        board.put(2);
        assert_eq!(board.board[0], 0b11111 << 14);
        board.board[0] = 1u64 << 7;
        board.col_height[1] = 1;
        board.put(1);
        assert_eq!(board.board[0], 0b11 << 7);
    }

    #[test]
    fn test_remove() {
        let mut board = Board::new();
        board.put(2);
        let bb = board.board[0];
        board.put(2);
        board.remove(2);
        assert_eq!(board.board[0], bb);
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
        board.make_move(2);
        board.print();
        let hash = board.hash;
        board.unmake_move(2);
        board.print();
        board.make_move(4);
        board.print();
        assert_eq!(board.hash, hash);
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
        assert!(board.game_over(board.active ^ 1));
        let mut board = Board::new();
        for a in 0..4 {
            board.put(a);
        }
        // end move by switching sides - usually make_move handles that
        board.active ^= 1;
        assert!(board.game_over(board.active ^ 1));
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
        assert!(board.game_over(board.active ^ 1));
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
