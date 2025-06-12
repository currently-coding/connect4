use board::Board;
mod engine;

mod board;

fn main() {
    println!("Hello, world!");
    let mut board = Board::new();
    board.print();
    println!("{:064b}", board.board[0]);
    board.print();
}
