#[cfg(test)]
mod tests {
    use librune::board::{bitboard::Bitboard, defs::Square};

    #[test]
    fn representation() {
        let sq = Square::try_from(0u8).expect("Expected try_from to succeed");
        assert_eq!(sq, Square::A1);
        assert_eq!(sq.file(), 0);
        assert_eq!(sq.rank(), 0);
        assert_eq!(sq.to_string(), "a1");
        assert_eq!(Bitboard::from_square(sq), Bitboard(1u64));

        let sq = Square::try_from(63u8).expect("Expected try_from to succeed");
        assert_eq!(sq, Square::H8);
        assert_eq!(sq.file(), 7);
        assert_eq!(sq.rank(), 7);
        assert_eq!(sq.to_string(), "h8");
        assert_eq!(
            Bitboard::from_square(sq),
            Bitboard(1u64 << 63)
        );
    }
}
