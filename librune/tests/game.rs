#[cfg(test)]
mod game {
    use librune::{
        defs::{Castling, GameState, Sides},
        eval::weights::Weights,
        movegen::defs::Move,
    };

    #[test]
    fn test_game_state_new_defaults() {
        let gs = GameState::new();
        assert_eq!(gs.active_side, Sides::WHITE);
        assert_eq!(gs.castling, 0);
        assert_eq!(gs.en_passant, None);
        assert_eq!(gs.half_move_clock, 0);
        assert_eq!(gs.fullmove_number, 0);
        assert_eq!(gs.zobrist_key, 0);
        assert_eq!(gs.phase_value, 0);
        assert_eq!(gs.psqt_value, [Weights(0, 0); Sides::BOTH as usize]);
    }

    #[test]
    fn test_game_state_clear() {
        let mut gs = GameState {
            active_side: Sides::BLACK,
            castling: Castling::ALL,
            en_passant: Some(10),
            half_move_clock: 50,
            fullmove_number: 10,
            zobrist_key: 12345,
            phase_value: 100,
            psqt_value: [Weights(10, 20); Sides::BOTH as usize],
            next_move: Move::new(1),
        };

        gs.clear();
        let default_gs = GameState::new();

        assert_eq!(gs.active_side, default_gs.active_side);
        assert_eq!(gs.castling, default_gs.castling);
        assert_eq!(gs.en_passant, default_gs.en_passant);
        assert_eq!(gs.half_move_clock, default_gs.half_move_clock);
        assert_eq!(gs.fullmove_number, default_gs.fullmove_number);
        assert_eq!(gs.zobrist_key, default_gs.zobrist_key);
        assert_eq!(gs.phase_value, default_gs.phase_value);
        assert_eq!(gs.psqt_value, default_gs.psqt_value);
    }
}
