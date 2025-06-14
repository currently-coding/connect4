use std::collections::{HashMap, VecDeque};

use nohash_hasher::BuildNoHashHasher;

use crate::{board::Board, ttentry::TTEntry};
pub struct Engine {
    depth: u8,
    pub prune_counter: u32,
    pub tt_counter: u32,
    pub visited_counter: u32,
    pub seen: HashMap<u32, TTEntry, BuildNoHashHasher<u32>>,
    seen_order: VecDeque<u32>,
    max: i8,
    seen_size: u64,
}
pub const MAX_MOVES: u8 = 42;
pub const MAX_TABLE_SIZE: u64 = 9_000_000;

impl Engine {
    pub fn new(depth: u8) -> Self {
        Engine {
            depth,
            max: MAX_MOVES as i8,
            prune_counter: 0,
            tt_counter: 0,
            visited_counter: 0,
            seen: HashMap::default(),
            seen_order: VecDeque::new(),
            seen_size: 0u64,
        }
    }

    pub const MOVES: [u8; 7] = [3, 2, 4, 1, 5, 6, 0];

    pub fn get_move(&mut self, board: &mut Board) -> u8 {
        board.print();
        self.find_best_move(board)
    }

    fn find_best_move(&mut self, board: &mut Board) -> u8 {
        let mut best_move = 0;
        let mut best_score = -self.max;
        for m in Engine::MOVES {
            if board.make_move(m) == 0 {
                continue;
            }
            let score: i8 = -self.negamax(board, -self.max, self.max, self.depth - 1);
            println!("Move {}: {}", m, score);

            board.unmake_move(m);
            if score > best_score {
                best_score = score;
                best_move = m;
            }
        }
        best_move
    }
    fn negamax(&mut self, board: &mut Board, mut alpha: i8, beta: i8, depth: u8) -> i8 {
        if depth == 0 {
            return 0;
        }
        self.visited_counter += 1;
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
        if board.game_over(board.active) {
            return self.max - (self.depth - depth) as i8;
        } else if board.game_over(board.active ^ 1) {
            return -(self.max - (self.depth - depth) as i8);
        } else if board.draw() {
            return 0;
        }

        let original_alpha = alpha;
        for m in Engine::MOVES {
            if board.make_move(m) == 0 {
                continue;
            }
            let score = -self.negamax(board, -beta, -alpha, depth - 1);
            board.unmake_move(m);

            alpha = alpha.max(score);
            if alpha >= beta {
                self.prune_counter += 1;
                // prunes WAYY too often
                break;
            }
        }
        let score = alpha;
        let flag: i8 = if score <= original_alpha {
            1
        } else if score >= beta {
            -1
        } else {
            0
        };
        match self.seen.get(&board.hash) {
            // replace if the current value has a deeper depth
            Some(entry) if depth >= entry.depth() => {
                self.insert_tt(board.hash, TTEntry::new(depth, score, flag)); // updated entry
            }
            None => {
                self.insert_tt(board.hash, TTEntry::new(depth, score, flag)); // new entry
            }
            _ => {}
        }
        score
    }

    fn insert_tt(&mut self, hash: u32, entry: TTEntry) {
        if self.seen_size >= MAX_TABLE_SIZE {
            for _ in 0..1000 {
                self.seen.remove(&self.seen_order[0]);
                self.seen_order.pop_front();
            }
        }
        self.seen.insert(hash, entry);
    }
}
