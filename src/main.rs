#![feature(exclusive_range_pattern)]


mod chess;
use chess::{Game, display::coord_to_algebraic, fen::coord_from_algebraic};

fn main() {
    let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let coord = coord_from_algebraic("b1").unwrap();
    let possible_moves = game.get_possible_moves(coord);
    println!("{:#}", game);
    println!("Possible moves from {} would be:", coord_to_algebraic(coord));
    for m in possible_moves {
        println!("\t{:#}", m);
    }
}
