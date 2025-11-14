use std::ops;

use crate::board::defs::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);
impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);

    /// Creates a [`Bitboard`] with a single bit set at the given [`Square`]
    #[inline]
    #[must_use]
    pub fn from_square(square: Square) -> Self {
        Bitboard(1u64 << square.0)
    }

    /// Checks if a square is set in the [`Bitboard`]
    #[inline]
    #[must_use]
    pub fn is_set(self, square: Square) -> bool {
        (self.0 & (1u64 << square.0)) != 0
    }

    /// Sets a square in the [`Bitboard`]
    #[inline]
    pub fn set(&mut self, square: Square) {
        self.0 |= 1u64 << square.0;
    }

    /// Clears a square in the [`Bitboard`]
    #[inline]
    pub fn clear(&mut self, square: Square) {
        self.0 &= !(1u64 << square.0);
    }

    /// Flips a square in the [`Bitboard`]
    #[inline]
    pub fn flip(&mut self, square: Square) {
        self.0 ^= 1u64 << square.0;
    }

    /// Counts the number of set bits in the [`Bitboard`]
    #[inline]
    #[must_use]
    pub fn pop_count(self) -> u32 {
        self.0.count_ones()
    }
}

impl ops::BitAnd for Bitboard {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl ops::BitOr for Bitboard {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}
