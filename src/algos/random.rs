use rand::Rng;

use crate::chess::{Game, Move};

pub fn random_move(game: &Game) -> Move {
    let moves = game.get_all_possible_moves();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..moves.len());
    return moves[index].clone();
}