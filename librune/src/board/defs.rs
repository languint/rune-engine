use core::fmt;

use crate::board::bitboard::Bitboard;

/// A square on a chess board
///
/// `A1` -> `0`
/// `H8` -> `63`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(pub u8);

impl Square {
    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);

    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);

    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);

    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);

    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);

    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);

    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);

    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);

    #[allow(clippy::cast_possible_truncation)]
    pub const ALL: [Square; 64] = {
        let mut arr = [Square(0); 64];
        let mut i = 0;
        while i < 64 {
            arr[i] = Square(i as u8);
            i += 1;
        }
        arr
    };

    #[inline]
    #[must_use]
    pub const fn rank(self) -> u8 {
        self.0 / 8
    }

    #[inline]
    #[must_use]
    pub const fn file(self) -> u8 {
        self.0 % 8
    }

    #[inline]
    #[must_use]
    pub const fn file_char(self) -> char {
        (b'a' + self.file()) as char
    }

    #[inline]
    #[must_use]
    pub const fn rank_char(self) -> char {
        (b'1' + self.rank()) as char
    }

    #[inline]
    #[must_use]
    pub const fn bb_mask(self) -> Bitboard {
        Bitboard(1u64 << self.0)
    }
}

impl From<Square> for u8 {
    #[inline]
    fn from(square: Square) -> Self {
        square.0
    }
}

impl From<u8> for Square {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{}{}", self.file_char(), self.rank_char()))
    }
}

/// A file on the chessboard, `A`->`H`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct File(pub u8);

impl File {
    pub const A: File = File(0);
    pub const B: File = File(1);
    pub const C: File = File(2);
    pub const D: File = File(3);
    pub const E: File = File(4);
    pub const F: File = File(5);
    pub const G: File = File(6);
    pub const H: File = File(7);
}

/// A rank on the chessboard, `0`->`7` (`R1=0`...`R8=7`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rank(pub u8);

impl Rank {
    pub const R1: Rank = Rank(0);
    pub const R2: Rank = Rank(1);
    pub const R3: Rank = Rank(2);
    pub const R4: Rank = Rank(3);
    pub const R5: Rank = Rank(4);
    pub const R6: Rank = Rank(5);
    pub const R7: Rank = Rank(6);
    pub const R8: Rank = Rank(7);
}

/// A chess piece, kings, queens, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece(pub u8);
impl Piece {
    pub const KING: u8 = 0;
    pub const QUEEN: u8 = 1;
    pub const ROOK: u8 = 2;
    pub const KNIGHT: u8 = 3;
    pub const BISHOP: u8 = 4;
    pub const PAWN: u8 = 5;

    pub const ALL: [u8; 6] = [
        Self::KING,
        Self::QUEEN,
        Self::ROOK,
        Self::KNIGHT,
        Self::BISHOP,
        Self::PAWN,
    ];
}
