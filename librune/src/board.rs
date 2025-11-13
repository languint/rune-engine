use crate::{
    board::{bitboard::Bitboard, defs::Square, pieces::Pieces},
    defs::{NrOf, Sides},
};

pub mod bitboard;
pub mod defs;
pub mod fen;
pub mod pieces;

/// Contains transient game state information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameState {
    /// The square that can be targeted for an en passant capture on the next move.
    /// `None` if no en passant target is available.
    pub en_passant_target: Option<Square>,

    /// Bitmask representing the castling rights.
    /// (0: K, 1: Q, 2: k, 3: q)
    pub castling_rights: u8,

    /// The number of halfmoves since the last pawn advance or piece capture.
    /// Used for the fifty-move rule.
    pub halfmove_clock: u16,

    /// The number of the full move. It starts at 1, and is incremented after Black's move.
    pub fullmove_number: u16,

    /// The [`Side`][`Sides`] whose turn it is to move.
    pub side_to_move: u8,
}

#[derive(Debug)]
/// A chess board
pub struct Board {
    pub bb_pieces: [[Bitboard; NrOf::PIECE_TYPES as usize]; NrOf::SIDES as usize],
    pub bb_occupancy: [Bitboard; NrOf::SIDES as usize],
    pub game_state: GameState,
    // pub history: History,
}

impl Board {
    const DEFAULT_GAME_STATE: GameState = GameState {
        en_passant_target: None,
        castling_rights: 0,
        halfmove_clock: 0,
        fullmove_number: 1,
        side_to_move: Sides::WHITE,
    };

    pub const EMPTY: Board = Board {
        bb_occupancy: [Bitboard::EMPTY; 2],
        bb_pieces: [[Bitboard::EMPTY; NrOf::PIECE_TYPES as usize]; NrOf::SIDES as usize],
        game_state: Self::DEFAULT_GAME_STATE,
    };
}

impl Board {
    #[inline]
    #[must_use]
    pub fn get_piece_bb(&self, side: u8, piece: Pieces) -> Bitboard {
        self.bb_pieces[side as usize][piece as usize]
    }

    #[inline]
    #[must_use]
    pub fn get_occupancy_bb(&self, side: u8) -> Bitboard {
        self.bb_occupancy[side as usize]
    }

    #[inline]
    pub fn get_piece_bb_mut(&mut self, side: u8, piece: Pieces) -> &mut Bitboard {
        &mut self.bb_pieces[side as usize][piece as usize]
    }

    #[inline]
    pub fn get_occupancy_bb_mut(&mut self, side: u8) -> &mut Bitboard {
        &mut self.bb_occupancy[side as usize]
    }
}
