use crate::{defs::NrOf, game::game_state::GameState};

pub struct History {
    game_states: [GameState; NrOf::MAX_HISTORY],
    count: usize,
}
impl History {
    #[allow(clippy::large_stack_arrays)]
    pub const EMPTY: History = History {
        game_states: [GameState::EMPTY; NrOf::MAX_HISTORY],
        count: 0,
    };
}

impl History {
    pub fn push(&mut self, gs: GameState) {
        self.game_states[self.count] = gs;
        self.count += 1;
    }

    #[must_use]
    pub fn pop(&mut self) -> Option<&GameState> {
        if let Some(index) = self.count.checked_sub(1) {
            let last_state = self.game_states.get(index);
            self.count = index;
            return last_state;
        }
        None
    }

    #[must_use]
    pub fn get(&self) -> Option<&GameState> {
        if let Some(index) = self.count.checked_sub(1) {
            self.game_states.get(index)
        } else {
            None
        }
    }
}
