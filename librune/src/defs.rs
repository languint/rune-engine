/// Sides of a chessboard, `WHITE`, `BLACK`, or `BOTH`
pub struct Sides {}
impl Sides {
    pub const WHITE: u8 = 0;
    pub const BLACK: u8 = 1;
    pub const BOTH: u8 = 2;
}

pub struct NrOf {}
impl NrOf {
    pub const FILES: u8 = 8;
    pub const RANKS: u8 = 8;
    pub const SQUARES: u8 = 64;

    pub const PIECE_TYPES: u8 = 6;
    pub const SIDES: u8 = 2;
}
