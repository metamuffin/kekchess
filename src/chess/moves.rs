use super::{Color, Coord, Game, Move, Piece};

pub const BOARD_DIRECTIONS: &[Coord; 4] = &[Coord(0, 1), Coord(0, -1), Coord(1, 0), Coord(-1, 0)];
pub const BOARD_DIRECTIONS_DIAGONAL: &[Coord; 8] = &[
    Coord(1, 1),
    Coord(1, -1),
    Coord(-1, 1),
    Coord(-1, -1),
    Coord(0, 1),
    Coord(0, -1),
    Coord(1, 0),
    Coord(-1, 0),
];
pub const BOARD_DIRECTIONS_ONLY_DIAGONAL: &[Coord; 4] =
    &[Coord(1, 1), Coord(1, -1), Coord(-1, 1), Coord(-1, -1)];
pub const KNIGHT_MOVES: &[Coord; 8] = &[
    Coord(2, 1),
    Coord(2, -1),
    Coord(-2, 1),
    Coord(-2, -1),
    Coord(1, 2),
    Coord(-1, 2),
    Coord(1, -2),
    Coord(-1, -2),
];

impl Game {
    pub fn get_all_possible_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        for file in 0..8 {
            for rank in 0..8 {
                let c = Coord(file, rank);
                if let Some(tile) = self.board[c.index()] {
                    if tile.0 == self.active_color {
                        moves.append(&mut self.get_possible_moves(&c))
                    }
                }
            }
        }
        return moves;
    }

    pub fn get_possible_moves(&self, c: &Coord) -> Vec<Move> {
        let tile = self.board[c.index()]
            .expect("Internal error. tried to get moves of a tile that does not exist");
        let mut moves: Vec<Move> = vec![];
        let mut basic_moves_targets: Vec<Coord> = vec![];

        match tile.1 {
            Piece::Queen => {
                for d in BOARD_DIRECTIONS_ONLY_DIAGONAL {
                    basic_moves_targets.append(&mut self.get_consecutive_capture_tiles(c, d))
                }
            }
            Piece::King => {
                for d in BOARD_DIRECTIONS_DIAGONAL {
                    basic_moves_targets.push(c.offset(d));
                }
            }
            Piece::Knight => {
                for d in KNIGHT_MOVES {
                    basic_moves_targets.push(c.offset(d));
                }
            }
            Piece::Bishop => {
                for d in BOARD_DIRECTIONS_ONLY_DIAGONAL {
                    basic_moves_targets.append(&mut self.get_consecutive_capture_tiles(c, d))
                }
            }
            Piece::Rook => {
                for d in BOARD_DIRECTIONS {
                    basic_moves_targets.append(&mut self.get_consecutive_capture_tiles(c, d))
                }
            }
            Piece::Pawn => {
                basic_moves_targets.push(c.offset(&tile.0.get_direction()));
                if c.1 == 7 && tile.0 == Color::White {
                    basic_moves_targets.push(c.offset(&tile.0.get_direction().mul(2)));
                }
                if c.1 == 1 && tile.0 == Color::Black {
                    basic_moves_targets.push(c.offset(&tile.0.get_direction().mul(2)));
                }
                // TODO en passent
            }
        }
        let mut basic_moves = basic_moves_targets
            .iter()
            .filter(|t| t.is_valid())
            .filter(|t| self.can_capture_tile(c, t))
            .map(|t| Move::Basic(c.clone(), t.clone()))
            .collect::<Vec<_>>();

        moves.append(&mut basic_moves);
        return moves;
    }

    pub fn make_move_unchecked(&mut self, m: Move) {
        match m {
            Move::Basic(from, to) => {
                let tile = self.board[from.index()];
                self.board[to.index()] = tile;
                self.board[from.index()] = None;
            }
            Move::Castle(king_from, king_to) => {
                // TODO castling
                todo!();
            }
            Move::EnPassent(from, to) => {
                // TODO en passent
                todo!();
            }
            Move::PawnPromotion(from, to, a) => {
                todo!();
            }
        }
    }

    pub fn can_capture_tile(&self, source: &Coord, target: &Coord) -> bool {
        let source_tile =
            self.board[source.index()].expect("Nothing is not able to capture anything");
        let target_tile = self.board[target.index()];
        return match target_tile {
            None => true,
            Some(t) => {
                if t.0 == source_tile.0 || t.1 == Piece::King {
                    return false;
                }
                // TODO
                return true;
            }
        };
    }

    pub fn get_consecutive_capture_tiles(&self, start: &Coord, direction: &Coord) -> Vec<Coord> {
        let mut cursor = start.clone();
        let mut c = vec![];
        loop {
            cursor.offset_in_place(direction);
            c.push(cursor.clone());
            if !cursor.is_valid() {
                break;
            }
            if let Some(_) = self.board[cursor.index()] {
                break;
            }
        }
        return c;
    }
}

impl Color {
    pub fn get_direction(&self) -> Coord {
        match self {
            Color::Black => Coord(0, -1),
            Color::White => Coord(0, 1),
        }
    }
}
