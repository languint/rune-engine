#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Pieces {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}