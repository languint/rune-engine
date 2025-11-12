use crate::{bitboard::Bitboard, defs::NrOf};

pub struct Board {
    pub bb_pieces: [[Bitboard; NrOf::PIECE_TYPES]]
}