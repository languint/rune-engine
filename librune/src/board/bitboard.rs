use crate::board::defs::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);
impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);

    /// Creates a [`Bitboard`] with a single bit set at the given [`Square`]
    #[inline]
    #[must_use]
    pub fn from_square(square: Square) -> Self {
        Bitboard(1u64 << u8::from(square))
    }

    /// Checks if a square is set in the [`Bitboard`]
    #[inline]
    #[must_use]
    pub fn is_set(self, square: Square) -> bool {
        (self.0 & (1u64 << u8::from(square))) != 0
    }

    /// Sets a square in the [`Bitboard`]
    #[inline]
    pub fn set(&mut self, square: Square) {
        self.0 |= 1u64 << u8::from(square);
    }

    /// Clears a square in the [`Bitboard`]
    #[inline]
    pub fn clear(&mut self, square: Square) {
        self.0 &= !(1u64 << u8::from(square));
    }

    /// Flips a square in the [`Bitboard`]
    #[inline]
    pub fn flip(&mut self, square: Square) {
        self.0 ^= 1u64 << u8::from(square);
    }

    /// Counts the number of set bits in the [`Bitboard`]
    #[inline]
    #[must_use]
    pub fn pop_count(self) -> u32 {
        self.0.count_ones()
    }
}
