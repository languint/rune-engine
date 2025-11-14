use crate::{
    board::{Board, defs::Square},
    defs::Sides,
    game::{Game, defs::CastlingRights, gamestate::GameState},
};

pub enum FenParsingError {
    NotEnoughParts,
    InvalidSideToMove(String),
    InvalidCastlingPart(String),
    InvalidEnPassantSquare(String),
    InvalidFullmoveCount(String),
    InvalidHalfmoveClock(String),
}

pub const FEN_PARTS: usize = 6;

pub struct Fen;
impl Fen {
    /// Parse a FEN string into a [`Game`]
    ///
    /// # Errors
    /// Returns `Err` if the FEN is invalid
    pub fn parse(fen: &str) -> Result<Game, FenParsingError> {
        let parts: Vec<&str> = fen.split_ascii_whitespace().collect();

        if parts.len() != FEN_PARTS {
            return Err(FenParsingError::NotEnoughParts);
        }

        let mut game = Game::EMPTY;

        let side_to_move = match parts[1] {
            "w" => Sides::WHITE,
            "b" => Sides::BLACK,
            _ => {
                return Err(FenParsingError::InvalidSideToMove(format!(
                    "{} is not a valid side to move",
                    parts[1]
                )));
            }
        };

        let castling_rights = Self::parse_castling(parts[2])?;
        let en_passant_square = Self::parse_en_passant(parts[3])?;

        let fullmove_count = parts[4].parse::<u16>().map_err(|_| {
            FenParsingError::InvalidFullmoveCount(format!(
                "{} cannot be a fullmove count",
                parts[4]
            ))
        })?;

        let halfmove_clock = parts[5].parse::<u16>().map_err(|_| {
            FenParsingError::InvalidHalfmoveClock(format!(
                "{} cannot be a halfmove clock",
                parts[5]
            ))
        })?;

        game.history.push(GameState {
            board: Board::EMPTY,
            castling_rights,
            en_passant_square,
            fullmove_count,
            side_to_move,
            halfmove_clock,
        });

        Ok(game)
    }

    /// Parse the en passant square of a FEN string
    ///
    /// # Errors
    /// Returns `Err` if the target square is invalid
    fn parse_en_passant(en_passant: &str) -> Result<Option<Square>, FenParsingError> {
        if en_passant == "-" {
            return Ok(None);
        }

        Ok(Some(Square::try_from(en_passant).map_err(|_| {
            FenParsingError::InvalidEnPassantSquare(format!(
                "Cannot parse {en_passant} as a square"
            ))
        })?))
    }

    /// Parse the castling part of a FEN string
    ///
    /// # Errors
    /// Returns `Err` if a char other than `-` or `KQkq` is present
    fn parse_castling(castling: &str) -> Result<u8, FenParsingError> {
        let mut rights = CastlingRights::NONE;
        if castling == "-" {
            return Ok(rights);
        }

        for c in castling.chars() {
            match c {
                'K' => rights |= CastlingRights::WK,
                'Q' => rights |= CastlingRights::WQ,
                'k' => rights |= CastlingRights::BK,
                'q' => rights |= CastlingRights::BQ,
                _ => {
                    return Err(FenParsingError::InvalidCastlingPart(format!(
                        "{c} is not a valid castling char!"
                    )));
                }
            }
        }

        Ok(rights)
    }
}
