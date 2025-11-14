#[cfg(test)]
mod tests {
    use librune::game::{game_state::GameState, history::History};

    #[test]
    fn push() {
        let mut history = History::EMPTY;
        history.push(GameState {
            castling_rights: 0b001,
            ..GameState::EMPTY
        });

        assert_eq!(
            history.get(),
            Some(&GameState {
                castling_rights: 0b001,
                ..GameState::EMPTY
            })
        );
    }

    #[test]
    fn pop() {
        let mut history = History::EMPTY;
        let expected_state = GameState {
            castling_rights: 0b001,
            ..GameState::EMPTY
        };

        history.push(expected_state);

        assert_eq!(history.get(), Some(&expected_state));
        assert_eq!(history.pop(), Some(&expected_state));
        assert_eq!(history.get(), None);
    }
}
