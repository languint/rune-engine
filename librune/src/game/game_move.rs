use crate::board::defs::Square;

pub struct MoveFlags;
impl MoveFlags {
    pub const NONE: u8 = 0b0000;
    pub const CAPTURE: u8 = 0b0001;
    pub const CHECK: u8 = 0b0010;
    pub const CASTLING: u8 = 0b0100;
}

struct Shifts;
impl Shifts {
    pub const SOURCE: u16 = 10;
    pub const DESTINATION: u16 = 4;
}

/// A chess move
///
/// 0bSSSSSSDDDDDDFFFF
/// `S` - Source [`Square`]
///
/// `D` - Destination [`Square`]
///
/// `F` - [`MoveFlags`]
pub struct GameMove(pub u16);
impl GameMove {
    pub const NULL: GameMove = GameMove(0);

    #[must_use]
    pub fn from_parts(source: Square, destination: Square, flags: u8) -> GameMove {
        GameMove(
            (u16::from(source.0)) << Shifts::SOURCE
                | (u16::from(destination.0)) << Shifts::DESTINATION
                | u16::from(flags) ,
        )
    }

    #[must_use]
    pub fn source(&self) -> Square {
        Square((self.0 >> Shifts::SOURCE) as u8)
    }
}
