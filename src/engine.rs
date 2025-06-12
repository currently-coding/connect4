use crate::board::Board;

pub struct Engine {
    depth: u8,
    max: i16,
    min: i16,
    pub prune_counter: u32,
    pub visited_counter: u32,
}

impl Engine {
    pub fn new(depth: u8) -> Self {
        Engine {
            depth,
            max: 10000,
            min: -10000,
            prune_counter: 0,
            visited_counter: 0,
        }
    }

    pub fn make_move(&mut self, board: &mut Board) {
        let m: u8 = self.find_best_move(board);
        board.make_move(m);
    }

    fn find_best_move(&mut self, board: &mut Board) -> u8 {
        let mut best_move = 0;
        let mut best_score = self.min;
        for m in board.get_moves() {
            board.make_move(m);
            self.visited_counter += 1;
            let score: i16 = self.alpha_beta(board, self.min, self.max, self.depth - 1, true);
            board.unmake_move(m);
            if score > best_score {
                best_score = score;
                best_move = m;
            }
            if best_score == self.max {
                println!("Found the optimal solution!");
                return m;
            }
        }
        best_move
    }

    fn alpha_beta(
        &mut self,
        board: &mut Board,
        mut alpha: i16,
        mut beta: i16,
        depth: u8,
        maximizing: bool,
    ) -> i16 {
        if board.game_over() {
            return self.max - (self.depth - depth) as i16;
        } else if depth == 0 {
            return 0;
        }
        let mut best_score = if maximizing { self.min } else { self.max };
        let moves = board.get_moves();
        for m in moves {
            board.make_move(m);
            self.visited_counter += 1;
            let score: i16 = self.alpha_beta(board, alpha, beta, depth - 1, !maximizing);
            board.unmake_move(m);
            if maximizing {
                if score > best_score {
                    best_score = score;
                }
                alpha = i16::max(alpha, score);
                if alpha >= beta {
                    self.prune_counter += 1;
                    break;
                }
            }
            if !maximizing {
                if score < best_score {
                    best_score = score;
                }
                beta = i16::min(beta, score);
                if alpha >= beta {
                    self.prune_counter += 1;
                    break;
                }
            }
        }
        best_score
    }
}
