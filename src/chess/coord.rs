use core::panic;

use super::Coord;

impl Coord {
    pub fn index(&self) -> usize {
        if !self.is_valid() {
            panic!("{:?}", self)
        }
        (self.0 + self.1 * 8) as usize
    }
    pub fn offset(&self, off: &Coord) -> Self {
        Coord(self.0 + off.0, self.1 + off.1)
    }
    pub fn offset_in_place(&mut self, off: &Coord) {
        self.0 = self.0 + off.0;
        self.1 = self.1 + off.1;
    }
    pub fn is_valid(&self) -> bool {
        self.0 >= 0 && self.0 < 8 && self.1 >= 0 && self.1 < 8
    }

    #[inline]
    pub fn rank_index(&self) -> usize {
        self.1 as usize
    }

    #[inline]
    pub fn file_index(&self) -> usize {
        self.0 as usize
    }
    pub fn mul(&mut self, fac: i8) -> Self {
        Self(self.0 * fac, self.1 * fac)
    }
}
