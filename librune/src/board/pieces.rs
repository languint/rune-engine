/// The chess pieces
#[repr(u8)]
pub enum Pieces {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl TryFrom<&char> for Pieces {
    type Error = String;
    /// Attempt to construct a [`Piece`][Pieces] from a `char`
    ///
    /// Not case sensitive, `p` or `P` will both be pawns
    ///
    /// # Errors
    /// Returns `Err` if the piece couldn't be parsed
    #[inline]
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'p' => Ok(Pieces::Pawn),
            'n' => Ok(Pieces::Knight),
            'b' => Ok(Pieces::Bishop),
            'r' => Ok(Pieces::Rook),
            'q' => Ok(Pieces::Queen),
            'k' => Ok(Pieces::King),
            _ => Err(format!("Cannot parse `{value}` as a piece!")),
        }
    }
}
