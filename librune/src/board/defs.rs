use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rank(pub u8);

/// All [`Ranks`][`Rank`]
pub const ALL_RANKS: [Rank; 8] = [
    Rank(0),
    Rank(1),
    Rank(2),
    Rank(3),
    Rank(4),
    Rank(5),
    Rank(6),
    Rank(7),
];

impl TryFrom<&char> for Rank {
    type Error = String;

    /// Attempt to construct a [`Rank`] from a `char`.
    ///
    /// Accepts digits `'1'` through `'8'`.
    ///
    /// # Errors
    /// Returns `Err` if the rank couldn't be parsed.
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        if let Some(digit) = value.to_digit(10)
            && (1..=8).contains(&digit)
        {
            return Ok(Rank(
                u8::try_from(digit)
                    .map_err(|e| format!("Failed to cast `{digit}` as a u8: {e}!"))?
                    - 1,
            ));
        }
        Err(format!("Cannot parse `{value}` as a rank!"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

/// All [`Files`][`File`]
pub const ALL_FILES: [File; 8] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

impl TryFrom<&char> for File {
    type Error = String;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        let lower = value.to_ascii_lowercase();

        if ('a'..='h').contains(&lower) {
            let index = lower as u8 - b'a';
            return Ok(unsafe { std::mem::transmute::<u8, File>(index) });
        }

        Err(format!("Cannot parse `{value}` as a file!"))
    }
}

impl TryFrom<u8> for File {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 7 {
            return Err(format!("Cannot parse {value} as a file!"));
        }

        Ok(unsafe { std::mem::transmute::<u8, File>(value) })
    }
}

impl From<File> for u8 {
    fn from(file: File) -> Self {
        file as u8
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (*self as u8 + b'a') as char)
    }
}

impl File {
    /// Transmute a `u8` as a [`File`]
    ///
    /// # Safety
    /// This function is safe as long us `value` is less than `8`
    #[must_use]
    #[inline]
    pub unsafe fn from_u8(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }

    /// Get the `u8` representation of a [`File`]
    #[must_use]
    #[inline]
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// A chess square using little endian mappings.
///
/// `0` -> `A1`
/// `63` -> `H8`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Square(pub u8);

/// All [`Squares`][`Square`]
pub const ALL_SQUARES: [Square; 64] = {
    let mut arr = [Square(0); 64];
    let mut i: u8 = 0;
    while i < 64 {
        arr[i as usize] = Square(i);
        i += 1;
    }
    arr
};

impl TryFrom<u8> for Square {
    type Error = String;
    /// Attempt to create a [`Square`] from a `u8`
    ///
    /// # Errors
    /// Returns `Err` if the value is greater than 63
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 63 {
            return Err(format!("Cannot parse `{value}` as a square!"));
        }

        Ok(Square(value))
    }
}

impl From<Square> for u8 {
    fn from(sq: Square) -> Self {
        sq.0
    }
}

impl Square {
    /// Get the [`File`] associated with this Square
    #[inline]
    #[must_use]
    pub fn file(self) -> File {
        unsafe { File::from_u8(self.0 & 7) }
    }

    /// Get the [`Rank`] associated with this Square
    #[inline]
    #[must_use]
    pub fn rank(self) -> Rank {
        Rank(self.0 >> 3)
    }

    /// Create a [`Square`] from `file` and `rank` coordinates
    #[inline]
    #[must_use]
    pub fn from_coords(file: &File, rank: &Rank) -> Self {
        let index = (rank.0 << 3) | file.as_u8();
        Square(index)
    }

    /// Create a [`Square`] from `file` and `rank` coordinates
    #[inline]
    #[must_use]
    pub fn from_coords_u8(file: u8, rank: u8) -> Self {
        let index = (rank << 3) | file;
        Square(index)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = (b'a' + u8::from(self.file())) as char;

        let rank = (b'1' + self.rank().0) as char;
        write!(f, "{file}{rank}")
    }
}

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w kqKQ - 0 1";
