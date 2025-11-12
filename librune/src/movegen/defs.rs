use crate::defs::{Piece, Square};

const MOVE_MASK: usize = 0x00_00_00_00_00_FF_FF_FF;

pub struct Shifts {}
impl Shifts {
    pub const PIECE: usize = 0;
    pub const FROM_SQ: usize = 3;
    pub const TO_SQ: usize = 9;
    pub const CAPTURE: usize = 15;
    pub const PROMOTION: usize = 18;
    pub const EN_PASSANT: usize = 21;
    pub const DOUBLE_STEP: usize = 22;
    pub const CASTLING: usize = 23;
    pub const SORTSCORE: usize = 24;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move(usize);

#[allow(clippy::cast_possible_truncation)]
impl Move {
    #[must_use]
    pub fn new(data: usize) -> Self {
        Self(data)
    }

    #[must_use]
    pub fn piece(&self) -> Piece {
        ((self.0 >> Shifts::PIECE as u64) & 0x7) as Piece
    }

    #[must_use]
    pub fn from(&self) -> Square {
        ((self.0 >> Shifts::FROM_SQ as u64) & 0x3F) as Square
    }

    #[must_use]
    pub fn to(&self) -> Square {
        ((self.0 >> Shifts::TO_SQ as u64) & 0x3F) as Square
    }

    #[must_use]
    pub fn captured(&self) -> Piece {
        ((self.0 >> Shifts::CAPTURE as u64) & 0x7) as Piece
    }

    #[must_use]
    pub fn promoted(&self) -> Piece {
        ((self.0 >> Shifts::PROMOTION as u64) & 0x7) as Piece
    }

    #[must_use]
    pub fn en_passant(&self) -> bool {
        ((self.0 >> Shifts::EN_PASSANT as u64) & 0x1) as u8 == 1
    }

    #[must_use]
    pub fn double_step(&self) -> bool {
        ((self.0 >> Shifts::DOUBLE_STEP as u64) & 0x1) as u8 == 1
    }

    #[must_use]
    pub fn castling(&self) -> bool {
        ((self.0 >> Shifts::CASTLING as u64) & 0x1) as u8 == 1
    }

    #[must_use]
    pub fn get_sort_score(self) -> u32 {
        ((self.0 >> Shifts::SORTSCORE as u64) & 0xFFFF_FFFF) as u32
    }

    pub fn set_sort_score(&mut self, value: u32) {
        let mask: usize = 0xFFFF_FFFF << Shifts::SORTSCORE;
        let v: usize = (value as usize) << Shifts::SORTSCORE;
        self.0 = (self.0 & !mask) | v;
    }

    #[must_use]
    pub fn to_short_move(self) -> ShortMove {
        ShortMove::new((self.0 & MOVE_MASK) as u32)
    }

    #[must_use]
    pub fn get_move(&self) -> u32 {
        (self.0 & MOVE_MASK) as u32
    }
}

pub struct ShortMove(u32);
impl ShortMove {
    #[must_use]
    pub fn new(data: u32) -> Self {
        Self(data)
    }

    #[must_use]
    pub fn get_move(&self) -> u32 {
        self.0
    }
}
