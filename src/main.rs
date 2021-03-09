#![feature(exclusive_range_pattern)]

mod chess;
use chess::{Coord, Game};

fn main() {
    let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let coord = Coord::from_algebraic("b1").unwrap();
    let possible_moves = game.get_possible_moves(&coord);
    println!("{:#}", game);
    println!("Possible moves from {} would be:", coord);
    for m in possible_moves {
        println!("\t{:#}", m);
    }
}
