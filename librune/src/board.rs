use std::sync::Arc;

use crate::{
    defs::{Bitboard, EMPTY, GameState, NrOf, Piece, Sides},
    piece::Pieces,
    zobrist::ZobristRandoms,
};

pub struct Board {
    pub bb_pieces: [[Bitboard; NrOf::PIECE_TYPES]; NrOf::SIDES],
    pub bb_occupancy: [Bitboard; NrOf::SIDES],
    pub game_state: GameState,
    pub history: Vec<GameState>,
    pub piece_list: [Piece; NrOf::SQUARES],
    zobrist_randoms: Arc<ZobristRandoms>,
}

impl Board {
    #[must_use]
    pub fn get_pieces(&self, side: Sides, piece: Pieces) -> Bitboard {
        self.bb_pieces[side as usize][piece as usize]
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    #[must_use]
    pub fn new() -> Self {
        Self {
            bb_pieces: [[EMPTY; NrOf::PIECE_TYPES]; Sides::BOTH as usize],
            bb_occupancy: [EMPTY; Sides::BOTH as usize],
            game_state: GameState::new(),
            history: vec![GameState::new()],
            piece_list: [Pieces::None as usize; NrOf::SQUARES],
            zobrist_randoms: Arc::new(ZobristRandoms::new()),
        }
    }
}
