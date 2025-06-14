use std::io::{self, Write};

use crate::board::Board;

pub struct Human {}

impl Human {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_move(&self, board: &Board) -> u8 {
        board.print();
        print!("Choose column: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately

        // Read input as String, then directly parse it into u8
        let col: u8 = io::stdin()
            .lines()
            .next()
            .expect("Failed to read line")
            .expect("Failed to read input")
            .trim()
            .parse()
            .expect("Failed to parse input as u8");
        col
    }
}
