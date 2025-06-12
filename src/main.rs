use board::Board;
use engine::Engine;
use human::Human;
mod engine;
mod human;

mod board;

fn main() {
    let mut board = Board::new();
    let mut p1 = Engine::new(10);
    // let mut p2 = Engine::new(25);
    let p2 = Human::new();
    // p1.make_move(&mut board);
    let mut m;
    loop {
        m = p1.get_move(&mut board);
        board.make_move(m);
        board.print();
        if board.game_over() {
            println!("P1 won!");
            break;
        }
        m = p2.get_move();
        board.make_move(m);
        board.print();
        if board.game_over() {
            println!("P2 won!");
            break;
        } else if board.occupancy() >= 0b0111111011111101111110111111011111101111110111111u64 {
            // TODO: find out what tha num is
            println!("DRAW!");
            break;
        }
    }
    println!("Summary:");
    println!("\tP1:");
    println!("\t\tVisited: {}", p1.visited_counter);
    println!("\t\tPruned:  {}", p1.prune_counter);
    println!("\t\tTT-table:{}", p1.tt_counter);
    // println!("\tP2:");
    // println!("\t\tVisited: {}", p2.visited_counter);
    // println!("\t\tPruned:  {}", p2.prune_counter);
    // println!("\t\tTT-table:{}", p2.tt_counter);
    println!("Exiting..");
}
