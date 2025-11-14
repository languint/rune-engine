pub mod bitboard;
pub mod defs;

use crate::{
    board::{bitboard::Bitboard, defs::Piece},
    defs::{NrOf, Sides},
};

/// A chess board, containing [`Bitboards`][`Bitboard`] for pieces and occupancy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub bb_pieces: [Bitboard; NrOf::PIECE_TYPES],
    pub bb_occupancy: [Bitboard; NrOf::SIDES],
    pub piece_list: [Option<Piece>; NrOf::SQUARES],
}
impl Board {
    pub const EMPTY: Board = Board {
        bb_pieces: [Bitboard::EMPTY; NrOf::PIECE_TYPES],
        bb_occupancy: [Bitboard::EMPTY; NrOf::SIDES],
        piece_list: [None; NrOf::SQUARES],
    };
}

impl Board {
    /// Get the [`Bitboard`] of a [`Side`][`Sides`]
    #[inline]
    #[must_use]
    pub fn occupancy_side(&self, side: u8) -> Bitboard {
        self.bb_occupancy[side as usize]
    }

    /// Get the combined [`Bitboard`] of both sides
    #[inline]
    #[must_use]
    pub fn occupancy(&self) -> Bitboard {
        self.occupancy_side(Sides::WHITE) | self.occupancy_side(Sides::BLACK)
    }
}
