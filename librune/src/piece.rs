#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Pieces {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    None,
}

impl TryFrom<usize> for Pieces {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0..=5 => Ok(unsafe { std::mem::transmute::<usize, Pieces>(value) }),
            _ => Err(format!("Cannot cast {value} as Piece!")),
        }
    }
}
