use crate::{
    board::{bitboard::Bitboard, defs::Piece},
    defs::NrOf,
};

/// A chess board, containing [`Bitboards`][`Bitboard`] for pieces and occupancy
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
    pub fn occupancy(&self) -> Bitboard {
        self.bb_occupancy[0] | self.bb_occupancy[1]
    }
}
