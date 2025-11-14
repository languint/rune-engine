/// The castling rights of a chess board, represented by 4 bits (`0b0000`)
pub struct CastlingRights;
impl CastlingRights {
    pub const WK: u8 = 0b0001;
    pub const WQ: u8 = 0b0010;

    pub const BK: u8 = 0b0100;
    pub const BQ: u8 = 0b1000;

    pub const NONE: u8 = 0b0000;
    pub const ALL: u8 = 0b1111;
}
