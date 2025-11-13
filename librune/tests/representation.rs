#[cfg(test)]
mod tests {
    use librune::{
        board::defs::{ALL_RANKS, File, Rank, Square},
        defs::NrOf,
    };

    #[test]
    fn rank_from_char_valid() {
        for (i, c) in ['1', '2', '3', '4', '5', '6', '7', '8'].iter().enumerate() {
            let rank = Rank::try_from(c).unwrap();
            assert_eq!(rank.0, i as u8);
        }
    }

    #[test]
    fn rank_from_char_invalid() {
        assert!(Rank::try_from(&'0').is_err());
        assert!(Rank::try_from(&'9').is_err());
        assert!(Rank::try_from(&'a').is_err());
    }

    #[test]
    fn rank_all_ranks_correct() {
        for i in 0..NrOf::RANKS {
            assert_eq!(ALL_RANKS[i as usize].0, i);
        }
    }

    #[test]
    fn file_u8_conversion() {
        for i in 0..NrOf::FILES {
            let file = File::try_from(i).unwrap();
            assert_eq!(file.as_u8(), i);
        }
        assert!(File::try_from(NrOf::FILES).is_err());
    }

    #[test]
    fn file_from_char_valid() {
        for (i, c) in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].iter().enumerate() {
            let file = File::try_from(c).unwrap();
            assert_eq!(file.as_u8(), i as u8);
        }

        for (i, c) in ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'].iter().enumerate() {
            let file = File::try_from(c).unwrap();
            assert_eq!(file.as_u8(), i as u8);
        }
    }

    #[test]
    fn file_from_char_invalid() {
        assert!(File::try_from(&'i').is_err());
        assert!(File::try_from(&'0').is_err());
        assert!(File::try_from(&'Z').is_err());
    }

    #[test]
    fn file_display() {
        assert_eq!(format!("{}", File::A), "a");
        assert_eq!(format!("{}", File::H), "h");
    }

    #[test]
    fn square_u8_conversion() {
        for i in 0..NrOf::SQUARES {
            let square = Square::try_from(i).unwrap();
            assert_eq!(u8::from(square), i);
        }

        assert!(Square::try_from(NrOf::SQUARES).is_err());
        assert!(Square::try_from(200).is_err());
    }

    #[test]
    fn square_coords_round_trip() {
        let sq_a1 = Square(0);
        assert_eq!(sq_a1.file(), File::A);
        assert_eq!(sq_a1.rank().0, 0);

        let sq_h8 = Square(63);
        assert_eq!(sq_h8.file(), File::H);
        assert_eq!(sq_h8.rank().0, 7);

        let sq_d4 = Square(27);
        assert_eq!(sq_d4.file(), File::D);
        assert_eq!(sq_d4.rank().0, 3);
    }

    #[test]
    fn square_from_coords() {
        let file_d = File::D;
        let rank_4 = Rank(3);
        let square = Square::from_coords(&file_d, &rank_4);

        assert_eq!(u8::from(square), 27);
    }

    #[test]
    fn square_display() {
        assert_eq!(format!("{}", Square(0)), "a1");
        assert_eq!(format!("{}", Square(63)), "h8");
        assert_eq!(format!("{}", Square(27)), "d4");
        assert_eq!(format!("{}", Square(8)), "a2");
        assert_eq!(format!("{}", Square(55)), "h7");
    }
}
