mod ttentry;
use board::Board;
use engine::Engine;
use human::Human;
mod engine;
mod human;

mod board;

fn main() {
    let mut board = Board::new();
    // board.print();
    let mut p1 = Engine::new(22);
    // let mut p2 = Engine::new(18);
    // let mut p2 = Human::new();
    let mut m;
    m = p1.get_move(&mut board);
    // board.make_move(m);
    // loop {
    //     m = p1.get_move(&mut board);
    //     board.make_move(m);
    //     if board.game_over(board.active) {
    //         println!("P1 won!");
    //         break;
    //     }
    //     m = p2.get_move(&mut board);
    //     board.make_move(m);
    //     if board.game_over(board.active) {
    //         println!("P2 won!");
    //         break;
    //     } else if board.draw() {
    //         println!("DRAW!");
    //         break;
    //     }
    // }
    // board.print();
    println!("Summary:");
    println!("\tP1:");
    println!("\t\tVisited: {}", p1.visited_counter);
    println!("\t\tPruned:  {}", p1.prune_counter);
    println!("\t\tTT-table:{}", p1.tt_counter);
    // println!("\tP2:");
    // println!("\t\tVisited: {}", p2.visited_counter);
    // println!("\t\tPruned:  {}", p2.prune_counter);
    // println!("\t\tTT-table access:{}", p2.tt_counter);
    // println!("\t\tTT-table size(now):{}", p2.seen.len());
    // println!("Exiting..");
}
