use super::{Color, Coord, Game, Move, Piece, Tile};

impl Game {
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut g = Self {
            board: [None; 64],
            move_count: 0,
            moves_since_capture: 0,
            castling_avail: [false; 4],
            active_color: Color::White,
            en_passent_target: None,
        };

        for (fi, field) in fen.split(" ").enumerate() {
            match fi {
                0 => {
                    for (rank, rr) in field
                        .split("/")
                        .enumerate()
                        .map(|(rank, v)| (7 - rank as i8, v))
                    {
                        let mut x = 0;
                        for c in rr.chars() {
                            match c {
                                '1' => x += 1,
                                '2' => x += 2,
                                '3' => x += 3,
                                '4' => x += 4,
                                '5' => x += 5,
                                '6' => x += 6,
                                '7' => x += 7,
                                '8' => x += 8,
                                _ => {
                                    if x > 8 || rank < 0 {
                                        return Err(format!(
                                            "FEN field 'pieces' uses a board thats bigger than 8x8"
                                        ));
                                    }
                                    g.board[Coord(x, rank as i8).index()] =
                                        Some(Tile::from_fen_char(c)?);
                                    x += 1;
                                }
                            }
                        }
                    }
                }
                1 => match field {
                    "b" => g.active_color = Color::Black,
                    "w" => g.active_color = Color::White,
                    _ => {
                        return Err(format!(
                            "FEN field 'active color' contains unrecognised color: {:?}",
                            field
                        ))
                    }
                },
                2 => {
                    if field != "-" {
                        for c in field.chars() {
                            let ca = match c {
                                'K' => 0,
                                'Q' => 1,
                                'k' => 2,
                                'q' => 3,
                                _ => return Err(format!("FEN field 'castling availibility' contains invalid character: {:?}", c))
                            };
                            g.castling_avail[ca] = true;
                        }
                    }
                }
                3 => {
                    if field != "-" {
                        g.en_passent_target = Some(Coord::from_algebraic(field)?)
                    }
                }
                4 => {
                    if let Ok(n) = field.parse::<u16>() {
                        g.moves_since_capture = n
                    } else {
                        return Err(format!(
                            "FEN field 'halfmove clock' contains invalid number: {:?}",
                            field
                        ));
                    }
                }
                5 => {
                    if let Ok(n) = field.parse::<u16>() {
                        g.move_count = n
                    } else {
                        return Err(format!(
                            "FEN field 'fullmove clock' contains invalid number: {:?}",
                            field
                        ));
                    }
                }
                _ => {
                    return Err(format!(
                        "FEN has too many fields. {:?} was not expected.",
                        field
                    ))
                }
            }
        }
        Ok(g)
    }
    pub fn to_fen(&self) -> String {
        let mut output = String::new();
        let mut empty_count: Option<usize> = None;
        for rank in 0..8 {
            if let Some(n) = empty_count {
                output += format!("{}", n).as_str();
                empty_count = None
            }
            if rank != 0 {
                output += "/"
            }
            for file in 0..8 {
                let tile = self.board[Coord(file, rank).index()];
                match tile {
                    Some(t) => {
                        if let Some(n) = empty_count {
                            output += format!("{}", n).as_str();
                            empty_count = None
                        }
                        output += format!("{}", t.as_fen_char()).as_str();
                    }
                    None => match empty_count {
                        None => empty_count = Some(1),
                        Some(n) => empty_count = Some(n + 1),
                    },
                }
            }
        }
        output += " ";
        output += format!("{}", self.active_color.as_fen_color()).as_str();
        output += " ";
        {
            let mut r = String::from("");
            if self.castling_avail[0] {
                r += "K";
            }
            if self.castling_avail[1] {
                r += "Q";
            }
            if self.castling_avail[2] {
                r += "k";
            }
            if self.castling_avail[3] {
                r += "q";
            }
            if r.len() == 0 {
                r += "-"
            }
            output += r.as_str();
        }
        output += " ";
        let temp = match &self.en_passent_target {
            None => String::from("-"),
            Some(t) => t.to_algebraic(),
        };
        output += temp.as_str();
        output += format!(" {} {}", self.moves_since_capture, self.move_count).as_str();
        return output;
    }
}

impl Color {
    pub fn as_fen_color(&self) -> char {
        match self {
            Color::Black => 'b',
            Color::White => 'w',
        }
    }
}

impl Tile {
    pub fn from_fen_char(s: char) -> Result<Self, String> {
        Ok(match s {
            'K' => Tile(Color::White, Piece::King),
            'k' => Tile(Color::Black, Piece::King),

            'Q' => Tile(Color::White, Piece::Queen),
            'q' => Tile(Color::Black, Piece::Queen),

            'B' => Tile(Color::White, Piece::Bishop),
            'b' => Tile(Color::Black, Piece::Bishop),

            'N' => Tile(Color::White, Piece::Knight),
            'n' => Tile(Color::Black, Piece::Knight),

            'R' => Tile(Color::White, Piece::Rook),
            'r' => Tile(Color::Black, Piece::Rook),

            'P' => Tile(Color::White, Piece::Pawn),
            'p' => Tile(Color::Black, Piece::Pawn),
            _ => return Err(format!("FEN tile character is invalid: {:?}", s)),
        })
    }
    pub fn as_fen_char(&self) -> char {
        let mut c = match self.1 {
            Piece::King => 'k',
            Piece::Queen => 'q',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Pawn => 'p',
        };
        if self.0 == Color::White {
            c = c.to_uppercase().to_string().chars().nth(0).unwrap();
        }
        return c;
    }
}

impl Coord {
    pub fn from_algebraic(s: &str) -> Result<Coord, String> {
        if s.len() != 2 {
            return Err(format!("Algebraic Notation has invalid length: {:?}", s));
        }
        let x = match s.chars().nth(0).unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => {
                return Err(format!(
                    "Algebraic Notation contains invalid rank letter: {:?}",
                    s
                ))
            }
        };
        let y = match s.chars().nth(1).unwrap() {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => {
                return Err(format!(
                    "Algebraic Notation contains invalid file number: {:?}",
                    s
                ))
            }
        };
        return Ok(Coord(x, y));
    }
    pub fn to_algebraic(&self) -> String {
        let files = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let ranks = &['1', '2', '3', '4', '5', '6', '7', '8'];
        return format!("{}{}", files[self.file_index()], ranks[self.rank_index()]);
    }
}

impl Move {
    pub fn serialize(&self) -> String {
        format!(
            "{}",
            match self {
                Move::Basic(from, to) => format!("b,{}-{}", from, to),
                Move::Castle(color, side) => format!("c,{},{}", color, side),
                Move::EnPassent(from, to) => format!("e,{}-{}", from, to),
                Move::PawnPromotion(from, to, a) => format!("p,{}-{},{}", from, to, a),
            }
        )
    }
    pub fn deserialize(s: &str) -> Self {
        todo!()
    }
}
