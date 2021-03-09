pub mod display;
pub mod fen;
pub mod helper;
pub mod moves;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Pawn,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile(Color, Piece);

// x file, y rank
pub type Coord = usize;
pub type CoordOff = isize;

#[derive(Debug)]
pub enum Move {
    Basic(Coord, Coord),
    Castle(Coord, Coord),
    EnPassent(Coord, Coord),
}

#[derive(Debug)]
pub struct Game {
    board: [Option<Tile>; 64],
    active_color: Color,
    castling_avail: [bool; 4], // KQkq
    move_count: u16,
    en_passent_target: Option<Coord>,
    moves_since_capture: u16,
}
