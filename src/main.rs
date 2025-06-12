use board::Board;
use engine::Engine;
mod engine;

mod board;

fn main() {
    let mut board = Board::new();
    let mut p1 = Engine::new(7);
    let mut p2 = Engine::new(7);
    loop {
        p1.make_move(&mut board);
        board.print();
        if board.game_over() {
            println!("P1 won!");
            break;
        }
        p2.make_move(&mut board);
        board.print();
        if board.game_over() {
            println!("P2 won!");
            break;
        }
    }
    println!("Summary:");
    println!("\tP1:");
    println!("\t\tVisited: {}", p1.visited_counter);
    println!("\t\tPruned:  {}", p1.prune_counter);
    println!("\tP2:");
    println!("\t\tVisited: {}", p2.visited_counter);
    println!("\t\tPruned:  {}", p2.prune_counter);
    println!("Exiting..");
}
