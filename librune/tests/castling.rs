#[cfg(test)]
mod castling {
    use librune::defs::{Castling, GameState, castling_rights_from_string};

    #[test]
    fn test_castling_from_string_all() {
        let castling = castling_rights_from_string(String::from("KQkq"));
        assert_eq!(castling, Castling::ALL);
    }

    #[test]
    fn test_castling_from_string_none() {
        let castling = castling_rights_from_string(String::from("-"));
        assert_eq!(castling, Castling::NONE);
    }

    #[test]
    fn test_castling_from_string_partial_single() {
        let castling = castling_rights_from_string(String::from("Q"));
        assert_eq!(castling, Castling::WQ);
    }

    #[test]
    fn test_castling_from_string_partial_invalid() {
        let castling = castling_rights_from_string(String::from("Kk"));
        assert_eq!(castling, Castling::WK | Castling::BK);
    }

    #[test]
    fn test_castling_to_string() {
        let gs_none = GameState {
            castling: 0,
            ..GameState::new()
        };
        assert_eq!(gs_none.castling_to_string(), "-");

        let gs_all = GameState {
            castling: Castling::ALL,
            ..GameState::new()
        };
        assert_eq!(gs_all.castling_to_string(), "KQkq");

        let gs_wk = GameState {
            castling: Castling::WK,
            ..GameState::new()
        };
        assert_eq!(gs_wk.castling_to_string(), "K");

        let gs_wq_bk = GameState {
            castling: Castling::WQ | Castling::BK,
            ..GameState::new()
        };
        assert_eq!(gs_wq_bk.castling_to_string(), "Qk");

        let gs_kq = GameState {
            castling: Castling::WK | Castling::WQ,
            ..GameState::new()
        };
        assert_eq!(gs_kq.castling_to_string(), "KQ");
    }
}
