use board::Board;
use engine::Engine;
mod engine;

mod board;

fn main() {
    let mut board = Board::new();
    let mut p1 = Engine::new(21);
    let mut p2 = Engine::new(10);
    loop {
        p1.make_move(&mut board);
        board.print();
        if board.game_over() || true {
            break;
        }
        p2.make_move(&mut board);
        board.print();
        if board.game_over() || true {
            break;
        }
    }
    println!("Summary:");
    println!("\tVisited: {}", p1.visited_counter);
    println!("\tPruned:  {}", p1.prune_counter);
    println!("Exiting..");
}
