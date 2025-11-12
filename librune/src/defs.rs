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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum Sides {
    WHITE,
    BLACK,
    BOTH,
}
