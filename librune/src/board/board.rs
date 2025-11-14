use crate::{board::bitboard::Bitboard, defs::NrOf};

/// A chess board, containing [`Bitboards`][`Bitboard`] for pieces and occupancy
pub struct Board {
    pub bb_pieces: [Bitboard; NrOf::PIECE_TYPES],
    pub bb_occupancy: [Bitboard; NrOf::SIDES],
}
impl Board {
    pub const EMPTY: Board = Board {
        bb_pieces: [Bitboard::EMPTY; NrOf::PIECE_TYPES],
        bb_occupancy: [Bitboard::EMPTY; NrOf::SIDES],
    };
}
