use nohash_hasher::BuildNoHashHasher;
use std::collections::HashMap;

use crate::board::Board;
pub struct Engine {
    depth: u8,
    max: i16,
    pub score: i16,
    pub prune_counter: u32,
    pub tt_counter: u32,
    pub visited_counter: u32,
    seen: HashMap<u64, i16, BuildNoHashHasher<u64>>,
}

impl Engine {
    pub fn new(depth: u8) -> Self {
        Engine {
            depth,
            max: 10000,
            score: 0,
            prune_counter: 0,
            tt_counter: 0,
            visited_counter: 0,
            seen: HashMap::default(),
        }
    }

    pub fn get_move(&mut self, board: &mut Board) -> u8 {
        self.find_best_move(board)
    }

    fn find_best_move(&mut self, board: &mut Board) -> u8 {
        let mut best_move = 0;
        let mut best_score = -self.max;
        for m in board.get_moves() {
            board.make_move(m);
            self.visited_counter += 1;
            let score: i16 = self.alpha_beta(board, -self.max, self.max, self.depth - 1, false);
            board.unmake_move(m);
            if score > best_score {
                best_score = score;
                best_move = m;
            }
            if best_score > self.max - 43 {
                println!("Found a solution!");
                return m;
            } else if best_score < -self.max + 43 {
                println!("I'm loosing.");
            }
        }
        self.score = best_score;
        println!("Score: {}", self.score);
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
        if let Some(score) = self.seen.get(&board.hash) {
            self.tt_counter += 1;
            return *score;
        }
        if board.game_over() {
            let value = self.max - (self.depth - depth) as i16;
            if maximizing {
                return -value;
            } else {
                return value;
            }
        } else if depth == 0 {
            return 0;
        }
        let mut best_score = if maximizing { -self.max } else { self.max };
        let moves = board.get_moves();
        for m in moves {
            board.make_move(m);
            self.visited_counter += 1;
            let score: i16 = self.alpha_beta(board, alpha, beta, depth - 1, !maximizing);
            self.seen.insert(board.hash, score);
            board.unmake_move(m);
            if maximizing {
                best_score = i16::max(best_score, score);
                alpha = i16::max(alpha, score);
                if alpha >= beta {
                    self.prune_counter += 1;
                    break;
                }
            }
            if !maximizing {
                best_score = i16::min(best_score, score);
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
