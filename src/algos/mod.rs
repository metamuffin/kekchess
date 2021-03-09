use crate::chess::{Game, Move};

pub mod minimax;
pub mod random;

pub fn algo_move(name: &str, game: &Game) -> Result<Move, String> {
    match name {
        "minimax" => Ok(minimax::minimax_move(game)),
        "random" => Ok(random::random_move(game)),
        _ => Err(format!("Unknown algo: {:?}", name)),
    }
}
