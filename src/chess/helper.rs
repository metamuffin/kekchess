use super::{Coord, CoordOff};

#[inline]
pub fn is_valid_coord(c: Coord) -> bool {
    c < 64 && c > 0
}

#[inline]
pub fn coord_offset(c: Coord, o: CoordOff) -> Coord {
    (c as CoordOff + o) as Coord
}

#[inline(always)]
pub fn get_file(c: Coord) -> usize {
    (c % 8) as usize
}
#[inline(always)]
pub fn get_rank(c: Coord) -> usize {
    (c / 8) as usize
}
#[inline(always)]
pub fn from_file_rank(file: usize, rank: usize) -> Coord {
    (rank * 8 + file) as Coord
}