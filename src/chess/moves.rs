use super::{Color, Coord, Game, GameState, Move, Piece, Tile};

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

    pub fn get_all_possible_moves_unchecked(&self) -> Vec<Move> {
        let mut moves = vec![];
        for file in 0..8 {
            for rank in 0..8 {
                let c = Coord(file, rank);
                if let Some(tile) = self.board[c.index()] {
                    if tile.0 == self.active_color {
                        moves.append(&mut self.get_possible_moves_unchecked(&c))
                    }
                }
            }
        }
        return moves;
    }

    pub fn get_possible_moves(&self, c: &Coord) -> Vec<Move> {
        return self
            .get_possible_moves_unchecked(c)
            .into_iter()
            // .inspect(|m| println!("{} {}", self.move_results_in_check(m), m))
            .filter(|m| !self.move_results_in_check(m))
            .collect::<Vec<Move>>();
    }

    pub fn get_possible_moves_unchecked(&self, c: &Coord) -> Vec<Move> {
        let tile = match self.board[c.index()] {
            Some(t) => t,
            None => return vec![],
        };
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
                if c.1 == 1 && tile.0 == Color::White {
                    basic_moves_targets.push(c.offset(&tile.0.get_direction().mul(2)));
                }
                if c.1 == 7 && tile.0 == Color::Black {
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

    pub fn move_results_in_check(&self, m: &Move) -> bool {
        let mut branch = self.clone();
        branch.make_move_unchecked(m);
        return branch.is_check().is_some();
    }

    pub fn make_move(&mut self, m: &Move) -> Result<GameState, String> {
        if self.move_results_in_check(m) {
            return Err(format!("move will result in check"));
        }
        let possible_moves = self.get_possible_moves(&m.get_source_coord());
        if !possible_moves.iter().any(|mc| *mc == *m) {
            return Err(format!(
                "move is not part of the set of all possible moves."
            ));
        }
        self.make_move_unchecked(m);
        Ok(self.state())
    }

    pub fn make_move_unchecked(&mut self, m: &Move) {
        let capture = match m {
            Move::Basic(from, to) => {
                let tile = self.board[from.index()];
                let capture = self.board[to.index()].is_some();
                self.board[to.index()] = tile;
                self.board[from.index()] = None;
                capture
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
        };
        if capture {
            self.moves_since_capture = 0;
        }
        if self.active_color == Color::White {
            self.move_count += 1;
            if !capture {
                self.moves_since_capture += 1;
            }
        }
        self.active_color = self.active_color.opponent();
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

    pub fn state(&self) -> GameState {
        let check_now = self.is_check();
        let check_next = self
            .get_all_possible_moves()
            .iter()
            .map(|m| {
                let mut branch = self.clone();
                branch.make_move_unchecked(m);
                branch.is_check()
            })
            .any(|a| a.contains(&self.active_color));

        return match (check_now.is_some(), check_next) {
            (false, true) => GameState::Stalemate,
            (true, true) => GameState::Checkmate(self.active_color),
            (true, false) => GameState::Check(check_now.unwrap()),
            (false, false) => GameState::Normal,
        };
    }

    pub fn is_check(&self) -> Option<Color> {
        for col in &[Color::White, Color::Black] {
            let check = self.get_all_possible_moves_unchecked().iter().any(|m| {
                match m.get_capture_target() {
                    None => false,
                    Some(v) => match self.board[v.index()] {
                        Some(t) => t == Tile(col.clone(), Piece::King),
                        None => false,
                    },
                }
            });
            if check {
                return Some(col.clone());
            }
        }
        None
    }
}

impl Color {
    pub fn get_direction(&self) -> Coord {
        match self {
            Color::Black => Coord(0, -1),
            Color::White => Coord(0, 1),
        }
    }
    pub fn opponent(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl Move {
    pub fn get_source_coord(&self) -> Coord {
        match self {
            Move::Basic(a, _) => a.clone(),
            Move::Castle(color, side) => match color {
                Color::White => Coord(4, 0),
                Color::Black => Coord(4, 7),
            },
            Move::EnPassent(a, _) => a.clone(),
            Move::PawnPromotion(a, _, _) => a.clone(),
        }
    }
    pub fn get_capture_target(&self) -> Option<Coord> {
        match self {
            Move::Basic(_, a) => Some(a.clone()),
            Move::Castle(_, _) => None,
            Move::EnPassent(_, a) => Some(a.clone()),
            Move::PawnPromotion(_, a, _) => Some(a.clone()),
        }
    }
}
