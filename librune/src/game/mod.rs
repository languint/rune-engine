use crate::game::{
    fen::{Fen, FenParsingError},
    gamestate::GameState,
    history::History,
};

pub mod defs;
pub mod fen;
pub mod gamestate;
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

    #[inline]
    pub fn from_fen(fen: &str) -> Result<Self, FenParsingError> {
        Fen::parse(fen)
    }
}
