use super::{
    helper::{get_file, get_rank},
    Move,
};
use super::{Color, Coord, Game};
use std::fmt::Write;

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for y in 0..8 {
            for x in 0..8 {
                board
                    .write_char(match self.board[y * 8 + x] {
                        Some(t) => t.as_fen_char(),
                        None => ' ',
                    })
                    .unwrap();
            }
            board.write_char('\n').unwrap();
        }

        f.write_fmt(format_args!(
            "{}\n{}",
            board,
            format!("active color: {}", self.active_color),
        ))
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Color::White => "White",
                Color::Black => "Black",
            }
        ))
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Move::Basic(from, to) => format!(
                    "{} -> {}",
                    coord_to_algebraic(*from),
                    coord_to_algebraic(*to)
                ),
                Move::Castle(from, to) => format!(
                    "Castle king from {} to {}",
                    coord_to_algebraic(*from),
                    coord_to_algebraic(*to)
                ),
                Move::EnPassent(from, to) => format!(
                    "En passent from {} to {}",
                    coord_to_algebraic(*from),
                    coord_to_algebraic(*to)
                ),
            }
        ))
    }
}

pub fn coord_to_algebraic(c: Coord) -> String {
    let files = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let ranks = &['1', '2', '3', '4', '5', '6', '7', '8'];
    return format!("{}{}", files[get_file(c)], ranks[get_rank(c)]);
}
