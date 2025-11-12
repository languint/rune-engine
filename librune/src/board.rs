use crate::{bitboard::Bitboard, defs::{NrOf, Sides}};

pub struct Board {
    pub bb_pieces: [[Bitboard; NrOf::PIECE_TYPES]; Sides::BOTH],
    pub bb_side: [Bitboard; Sides::BOTH],
}