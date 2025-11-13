use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rank(pub u8);

/// All [`Ranks`][`Rank`]
pub const ALL_RANKS: [Rank; 8] = [
    Rank(1),
    Rank(2),
    Rank(3),
    Rank(4),
    Rank(5),
    Rank(6),
    Rank(7),
    Rank(8),
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
        match value {
            '1' => Ok(Rank(1)),
            '2' => Ok(Rank(2)),
            '3' => Ok(Rank(3)),
            '4' => Ok(Rank(4)),
            '5' => Ok(Rank(5)),
            '6' => Ok(Rank(6)),
            '7' => Ok(Rank(7)),
            '8' => Ok(Rank(8)),
            _ => Err(format!("Cannot parse `{value}` as a rank!")),
        }
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
        match value.to_ascii_lowercase() {
            'a' => Ok(File::A),
            'b' => Ok(File::B),
            'c' => Ok(File::C),
            'd' => Ok(File::D),
            'e' => Ok(File::E),
            'f' => Ok(File::F),
            'g' => Ok(File::G),
            'h' => Ok(File::H),
            _ => Err(format!("Cannot parse `{value}` as a file!")),
        }
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
#[rustfmt::skip]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

/// All [`Squares`][`Square`]
pub const ALL_SQUARES: [Square; 64] = {
    let mut arr = [Square::A1; 64];
    let mut i: u8 = 0;
    while i < 64 {
        arr[i as usize] = unsafe { std::mem::transmute::<u8, Square>(i) };
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

        unsafe { Ok(std::mem::transmute::<u8, Square>(value)) }
    }
}

impl From<Square> for u8 {
    fn from(sq: Square) -> Self {
        sq as u8
    }
}

impl Square {
    /// Get the [`File`] associated with this [`Square`]
    #[inline]
    #[must_use]
    pub fn file(self) -> File {
        unsafe { File::from_u8((self as u8) & 7) }
    }

    /// Get the [`Rank`] associated with this [`Square`]
    #[inline]
    #[must_use]
    pub fn rank(self) -> Rank {
        Rank(self as u8 >> 3)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = (b'a' + u8::from(self.file())) as char;
        let rank = (b'1' + self.rank().0) as char;
        write!(f, "{file}{rank}")
    }
}
