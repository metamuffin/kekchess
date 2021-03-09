#![feature(exclusive_range_pattern)]

mod chess;
use chess::{Coord, Game};

fn main() {
    let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    println!("{:#}", game);

    let possible_moves = game.get_all_possible_moves();
    println!("All possible moves are:");
    for m in possible_moves {
        println!("\t{:#}", m);
    }
}
