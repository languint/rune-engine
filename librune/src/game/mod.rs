use crate::game::{
    fen::{Fen, FenParsingError},
    game_state::GameState,
    history::History,
};

pub mod defs;
pub mod fen;
pub mod game_move;
pub mod game_state;
pub mod history;

pub struct Game {
    pub history: History,
    pub game_state: GameState,
}
impl Game {
    pub const EMPTY: Game = Game {
        history: History::EMPTY,
        game_state: GameState::EMPTY,
    };

    /// Attempt to create a new game from a FEN string
    /// 
    /// # Errors
    /// Returns `Err` if the FEN is invalid
    #[inline]
    pub fn from_fen(fen: &str) -> Result<Self, FenParsingError> {
        Fen::parse(fen)
    }
}
