use crate::{eval::weights::Weights, movegen::defs::Move};

pub type Bitboard = u64;
pub type Piece = usize;
pub type Square = usize;
pub type Side = usize;

pub struct NrOf;
impl NrOf {
    pub const PIECE_TYPES: usize = 6;
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
    pub const SIDES: usize = 2;
    pub const CASTLING_PERMISSIONS: usize = 16;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum Sides {
    WHITE,
    BLACK,
    BOTH,
}

pub struct Castling;
impl Castling {
    pub const WK: u8 = 1;
    pub const WQ: u8 = 2;
    pub const BK: u8 = 4;
    pub const BQ: u8 = 8;
    pub const ALL: u8 = 15;
}

#[derive(Debug, Clone, Copy)]
pub struct GameState {
    pub active_side: Sides,
    pub castling: u8,
    pub half_move_clock: u8,
    pub en_passant: Option<Square>,
    pub fullmove_number: u16,
    pub zobrist_key: u64,
    pub phase_value: i16,
    pub psqt_value: [Weights; Sides::BOTH as usize],
    pub next_move: Move,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            active_side: Sides::WHITE,
            castling: 0,
            en_passant: None,
            half_move_clock: 0,
            fullmove_number: 0,
            zobrist_key: 0,
            phase_value: 0,
            psqt_value: [Weights(0, 0); Sides::BOTH as usize],
            next_move: Move::new(0),
        }
    }

    pub fn clear(&mut self) {
        self.active_side = Sides::WHITE;
        self.castling = 0;
        self.en_passant = None;
        self.half_move_clock = 0;
        self.fullmove_number = 0;
        self.zobrist_key = 0;
        self.phase_value = 0;
        self.psqt_value = [Weights(0, 0); Sides::BOTH as usize];
        self.next_move = Move::new(0);
    }

    #[must_use]
    pub fn castling_to_string(&self) -> String {
        let mut s: String = String::new();
        let c = self.castling;

        s += if c & Castling::WK > 0 { "K" } else { "" };
        s += if c & Castling::WQ > 0 { "Q" } else { "" };
        s += if c & Castling::BK > 0 { "k" } else { "" };
        s += if c & Castling::BQ > 0 { "q" } else { "" };

        if s.is_empty() {
            s = String::from("-");
        }

        s
    }
}

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const EMPTY: u64 = 0;

pub const MAX_GAME_MOVES: usize = 2048;
pub const MAX_LEGAL_MOVES: usize = 255;
pub const MAX_DEPTH_PLY: i8 = 125;
pub const MAX_MOVE_RULE: u8 = 100; //50/75
