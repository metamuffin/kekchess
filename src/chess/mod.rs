pub mod coord;
pub mod display;
pub mod fen;
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

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    Normal,
    Draw,
    Stalemate,
    Check(Color),
    Checkmate(Color),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile(Color, Piece);

// x file, y rank
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coord(pub i8, pub i8);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move {
    Basic(Coord, Coord),
    Castle(Color, bool),
    EnPassent(Coord, Coord),
    PawnPromotion(Coord, Coord, Piece),
}

#[derive(Debug, Clone)]
pub struct Game {
    board: [Option<Tile>; 64],
    active_color: Color,
    castling_avail: [bool; 4], // KQkq
    move_count: u16,
    en_passent_target: Option<Coord>,
    moves_since_capture: u16,
}
