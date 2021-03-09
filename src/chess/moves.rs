use super::{
    helper::coord_offset, helper::is_valid_coord, Color, Coord, CoordOff, Game, Move, Piece,
};
pub const BOARD_DIRECTIONS: &[CoordOff; 4] = &[1, -1, 8, -8];
pub const BOARD_DIRECTIONS_DIAGONAL: &[CoordOff; 8] = &[1, -1, 7, -7, 9, -9, 8, -8];
pub const BOARD_DIRECTIONS_ONLY_DIAGONAL: &[CoordOff; 4] = &[7, -7, 9, -9];
pub const KNIGHT_MOVES: &[CoordOff; 8] = &[
    -2 + -8,
    -2 + 8,
    2 + -8,
    2 + 8,
    16 + 1,
    -16 + 1,
    16 + -1,
    -16 + -1,
];

impl Game {
    pub fn get_possible_moves(&self, c: Coord) -> Vec<Move> {
        let tile = self.board[c as usize]
            .expect("Internal error. tried to get moves of a tile that does not exist");
        let mut moves: Vec<Move> = vec![];
        let mut basic_moves_targets: Vec<Coord> = vec![];

        match tile.1 {
            Piece::Queen => {
                for d in BOARD_DIRECTIONS_ONLY_DIAGONAL {
                    basic_moves_targets.append(&mut self.get_consecutive_capture_tiles(c, *d))
                }
            }
            Piece::King => {
                for d in BOARD_DIRECTIONS_DIAGONAL {
                    basic_moves_targets.push(coord_offset(c, *d));
                }
            }
            Piece::Knight => {
                for d in KNIGHT_MOVES {
                    basic_moves_targets.push(coord_offset(c, *d));
                }
            }
            Piece::Bishop => {
                for d in BOARD_DIRECTIONS_ONLY_DIAGONAL {
                    basic_moves_targets.append(&mut self.get_consecutive_capture_tiles(c, *d))
                }
            }
            Piece::Rook => {
                for d in BOARD_DIRECTIONS {
                    basic_moves_targets.append(&mut self.get_consecutive_capture_tiles(c, *d))
                }
            }
            Piece::Pawn => {
                basic_moves_targets.push(coord_offset(c, tile.0.get_direction()));
                if (8..16).contains(&c) || tile.0 == Color::White {
                    basic_moves_targets.push(coord_offset(c, tile.0.get_direction() * 2));
                }
                if (48..56).contains(&c) || tile.0 == Color::Black {
                    basic_moves_targets.push(coord_offset(c, tile.0.get_direction() * 2));
                }
                // TODO en passent
            }
        }

        let mut basic_moves = basic_moves_targets
            .iter()
            .filter(|t| is_valid_coord(**t))
            .filter(|t| self.can_capture_tile(c, **t))
            .map(|t| Move::Basic(c, *t))
            .collect::<Vec<_>>();

        moves.append(&mut basic_moves);
        return moves;
    }

    pub fn make_move_unchecked(&mut self, m: Move) {
        match m {
            Move::Basic(from, to) => {
                let tile = self.board[from];
                self.board[to] = tile;
                self.board[from] = None;
            }
            Move::Castle(king_from, king_to) => {
                // TODO castling
                todo!();
            }
            Move::EnPassent(from, to) => {
                // TODO en passent
                todo!();
            }
        }
    }

    pub fn can_capture_tile(&self, source: Coord, target: Coord) -> bool {
        return true;
    }

    pub fn get_consecutive_capture_tiles(&self, start: Coord, direction: CoordOff) -> Vec<Coord> {
        let mut cursor = start as CoordOff;
        let mut c = vec![];
        while is_valid_coord(cursor as Coord) {
            cursor += direction;
            if let Some(_) = self.board[cursor as Coord] {
                break;
            }
            c.push(cursor as Coord);
        }
        return c;
    }
}

impl Color {
    pub fn get_direction(&self) -> CoordOff {
        match self {
            Color::Black => -8,
            Color::White => 8,
        }
    }
}
