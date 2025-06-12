use crate::board::Board;

pub struct Engine {
    depth: u8,
}

pub const MOVES: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];

impl Engine {
    pub fn new(depth: u8) -> Self {
        Engine { depth }
    }

    pub fn make_move(board: &Board) -> u8 {
        let m: u8 = find_best_move(board);
        return m;
    }
}

fn find_best_move(board: &Board) -> u8 {
    let valid = board.get_moves(MOVES);
    let best_move;
    for m in &MOVES[0..valid] {
        let m = *m;
        board.make_move(m);
        let score = alpha_beta(board, alpha, beta, depth - 1);

        board.unmake_move(m);
    }
    best_move
}
