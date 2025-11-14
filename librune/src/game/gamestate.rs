use crate::{board::{Board, defs::Square}, defs::Sides, game::defs::CastlingRights};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameState {
    /// The chess [`Board`]
    pub board: Board,
    /// The side to move
    pub side_to_move: u8,
    /// The target square for en-passant
    pub en_passant_square: Option<Square>,
    /// The castling rights for the current position
    pub castling_rights: u8,
    /// The number of halfmoves since the last pawn advance, or capture
    pub halfmove_clock: u16,
    /// The number of fullmoves, starts at `1` and is incremented after black's turn
    pub fullmove_count: u16,
}
impl GameState {
    pub const EMPTY: GameState = GameState {
        board: Board::EMPTY,
        side_to_move: Sides::WHITE,
        en_passant_square: None,
        castling_rights: CastlingRights::ALL,
        fullmove_count: 1,
        halfmove_clock: 0,
    };
}
