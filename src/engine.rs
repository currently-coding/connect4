use lru::LruCache;
use std::num::NonZeroUsize;

pub const MAX_MOVES: u8 = 42;

use crate::{board::Board, ttentry::TTEntry};
pub struct Engine {
    depth: u8,
    pub score: i8,
    pub prune_counter: u32,
    pub tt_counter: u32,
    pub visited_counter: u32,
    pub seen: LruCache<u32, TTEntry>,
    max: i8,
}

impl Engine {
    pub fn new(depth: u8) -> Self {
        let cache_size = NonZeroUsize::new(9_000_000).unwrap(); // Safe if not zero
        let max = 50;
        Engine {
            depth,
            max,
            score: 0,
            prune_counter: 0,
            tt_counter: 0,
            visited_counter: 0,
            seen: LruCache::new(cache_size),
        }
    }

    pub const MOVES: [u8; 7] = [3, 2, 4, 1, 5, 6, 0];

    pub fn get_move(&mut self, board: &mut Board) -> u8 {
        self.find_best_move(board)
    }

    fn find_best_move(&mut self, board: &mut Board) -> u8 {
        let mut best_move = 0;
        let mut best_score = -self.max;
        for m in Engine::MOVES {
            if board.make_move(m) == 0 {
                continue;
            }
            self.visited_counter += 1;
            let score: i8 = -self.negamax(board, -self.max, self.max, self.depth - 1, 1);
            println!("Move {}: {}", m, score);

            board.unmake_move(m);
            if score > best_score {
                best_score = score;
                best_move = m;
            }
        }
        self.score = best_score;

        if self.score > self.max - MAX_MOVES as i8 {
            println!("Found a solution!");
        } else if self.score < -self.max + MAX_MOVES as i8 {
            println!("I'm loosing.");
        }
        best_move
    }
    fn negamax(&mut self, board: &mut Board, mut alpha: i8, beta: i8, depth: u8, side: i8) -> i8 {
        if let Some(ttentry) = self.seen.get(&board.hash) {
            self.tt_counter += 1;
            let ttflag = ttentry.flag();
            let ttscore = ttentry.score();
            if ttentry.depth() >= depth
                && (ttflag == 0
                    || ttflag == -1 && ttscore >= beta
                    || ttflag == 1 && ttscore <= alpha)
            {
                return ttscore;
            }
        }
        if board.game_over() {
            let value = -(self.max - (self.depth - depth - 1) as i8);
            println!("Found terminal state: {}", value);
            board.print();

            return value;
        } else if depth == 0 {
            // board.print();
            // println!("Exiting due to depth=0");
            return 0;
        }
        let original_alpha = alpha;
        for m in Engine::MOVES {
            if board.make_move(m) == 0 {
                continue;
            }
            self.visited_counter += 1;
            let score = -self.negamax(board, -beta, -alpha, depth - 1, -side);
            board.unmake_move(m);

            alpha = alpha.max(score);
            if alpha >= beta {
                self.prune_counter += 1;
                // prunes WAYY too often
                break;
            }
        }
        let flag: i8 = if alpha <= original_alpha {
            1
        } else if alpha >= beta {
            -1
        } else {
            0
        };
        match self.seen.get(&board.hash) {
            Some(entry) if entry.depth() <= depth => {
                self.seen.put(board.hash, TTEntry::new(depth, alpha, flag)); // updated entry
            }
            None => {
                self.seen.put(board.hash, TTEntry::new(depth, alpha, flag)); // new entry
            }
            _ => {}
        }
        alpha
    }
}
