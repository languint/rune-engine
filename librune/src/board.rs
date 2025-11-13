use crate::{board::bitboard::Bitboard, defs::NrOf};

pub mod defs;
pub mod pieces;
pub mod bitboard;

pub struct Board {
    pub bb_pieces: [[Bitboard; NrOf::PIECE_TYPES as usize]; NrOf::SIDES as usize],
    pub bb_occupancy: [Bitboard; NrOf::SIDES as usize],
}