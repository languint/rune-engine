use crate::{defs::{Bitboard, NrOf, Sides}, piece::Pieces};

pub struct Board {
    pub bb_pieces: [[Bitboard; NrOf::PIECE_TYPES]; NrOf::SIDES],
    pub bb_occupancy: [Bitboard; NrOf::SIDES],
}

impl Board {
    #[must_use]
    pub fn get_pieces(&self, side: Sides, piece: Pieces) -> Bitboard {
        self.bb_pieces[side as usize][piece as usize]
    }
}
