use super::{Color, Coord, Game};
use super::{Move, Piece};
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
                Move::Basic(from, to) => format!("{} -> {}", from, to),
                Move::Castle(from, to) => format!("Castle king from {} to {}", from, to),
                Move::EnPassent(from, to) => format!("En passent from {} to {}", from, to),
                Move::PawnPromotion(from, to, a) => {
                    format!("{} -> {}, pawn promoted to {}", from, to, a)
                }
            }
        ))
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Piece::King => "king",
                Piece::Queen => "queen",
                Piece::Knight => "knight",
                Piece::Bishop => "bishop",
                Piece::Rook => "rook",
                Piece::Pawn => "pawn",
            }
        ))
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let files = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let ranks = &['1', '2', '3', '4', '5', '6', '7', '8'];
        f.write_fmt(format_args!(
            "{}{}",
            files[self.file_index()],
            ranks[self.rank_index()]
        ))
    }
}
