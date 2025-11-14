/// Sides of a chessboard, `WHITE`, `BLACK`, or `BOTH`
pub struct Sides {}
impl Sides {
    pub const WHITE: u8 = 0;
    pub const BLACK: u8 = 1;
    pub const BOTH: u8 = 2;
}

pub struct NrOf {}
impl NrOf {
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
    pub const SQUARES: usize = 64;

    pub const PIECE_TYPES: usize = 6;
    pub const SIDES: usize = 2;
}
