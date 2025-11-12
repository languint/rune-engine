#[cfg(test)]
mod piece_tests {
    use librune::{defs::NrOf, piece::Pieces};

    #[test]
    fn piece_from_usize() {
        for i in 0..NrOf::PIECE_TYPES {
            let piece = Pieces::try_from(i);
            assert!(piece.is_ok());
        }
    }

    #[test]
    fn piece_to_usize() {
        for i in 0..NrOf::PIECE_TYPES {
            let piece = Pieces::try_from(i);
            assert!(piece.is_ok(), "Failed to cast usize {} to a valid Piece", i);
            assert_eq!(piece.unwrap() as usize, i)
        }
    }
}
