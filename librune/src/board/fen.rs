use crate::{
    board::{
        Board, GameState,
        defs::{File, Rank, Square},
        pieces::Pieces,
    },
    defs::Sides,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseFenError {
    NotEnoughParts,
    InvalidBoardPart(String),
    InvalidSideToMovePart(String),
    InvalidCastlingPart(String),
    InvalidEnPassantPart(String),
    InvalidHalfmovePart(String),
    InvalidFullmovePart(String),
}

/// Attempt to parse a FEN string, returns the position as a [`Board`]
///
/// # Errors
/// Returns `Err` if the FEN is invalid
pub fn parse_fen(fen: &str) -> Result<Board, ParseFenError> {
    let mut parts = fen.split_whitespace();

    let piece_placement_str = parts.next().ok_or(ParseFenError::NotEnoughParts)?;
    let mut board = parse_piece_placement(piece_placement_str)?;

    let side_to_move_str = parts.next().ok_or(ParseFenError::NotEnoughParts)?;
    let side_to_move = match side_to_move_str {
        "w" => Sides::WHITE,
        "b" => Sides::BLACK,
        _ => {
            return Err(ParseFenError::InvalidSideToMovePart(
                side_to_move_str.to_string(),
            ));
        }
    };

    let castling_str = parts.next().ok_or(ParseFenError::NotEnoughParts)?;
    let castling_rights = parse_castling_rights(castling_str)?;

    let en_passant_str = parts.next().ok_or(ParseFenError::NotEnoughParts)?;
    let en_passant_target = parse_en_passant_target(en_passant_str)?;

    let halfmove_str = parts.next().ok_or(ParseFenError::NotEnoughParts)?;
    let halfmove_clock = halfmove_str
        .parse::<u16>()
        .map_err(|_| ParseFenError::InvalidHalfmovePart(halfmove_str.to_string()))?;

    let fullmove_str = parts.next().ok_or(ParseFenError::NotEnoughParts)?;
    let fullmove_number = fullmove_str
        .parse::<u16>()
        .map_err(|_| ParseFenError::InvalidFullmovePart(fullmove_str.to_string()))?;

    let game_state = GameState {
        en_passant_target,
        castling_rights,
        halfmove_clock,
        fullmove_number,
        side_to_move,
    };

    board.game_state = game_state;
    dbg!(&board);
    Ok(board)
}

/// Parses the board piece placement section of the FEN string.
///
/// The parsing logic needs to iterate over the string, filling the
/// `bb_pieces` and `bb_occupancy` bitboards.
fn parse_piece_placement(fen_part: &str) -> Result<Board, ParseFenError> {
    let mut board = Board::EMPTY;
    let ranks: Vec<&str> = fen_part.split('/').collect();

    if ranks.len() != 8 {
        return Err(ParseFenError::InvalidBoardPart(format!(
            "Expected 8 ranks, found {}",
            ranks.len()
        )));
    }

    for (i, rank_str) in ranks.iter().enumerate() {
        let rank = 7 - i;
        let mut file: usize = 0;

        for char_piece in rank_str.chars() {
            if char_piece.is_ascii_digit() {
                let empty_squares = unsafe { char_piece.to_digit(10).unwrap_unchecked() } as usize;
                file += empty_squares;
            } else if let Some(piece) = Pieces::match_char(&char_piece.to_ascii_lowercase()) {
                if file >= 8 {
                    return Err(ParseFenError::InvalidBoardPart(format!(
                        "Too many pieces/squares on rank {} ({})",
                        rank + 1,
                        rank_str
                    )));
                }

                let square = Square::from_coords_u8(
                    u8::try_from(file).map_err(|_| {
                        ParseFenError::InvalidBoardPart(format!("Failed to parse {file} as u8"))
                    })?,
                    u8::try_from(rank).map_err(|_| {
                        ParseFenError::InvalidBoardPart(format!("Failed to parse {rank} as u8"))
                    })?,
                );

                let side = if char_piece.is_ascii_uppercase() {
                    Sides::WHITE
                } else {
                    Sides::BLACK
                };

                board.get_piece_bb_mut(side, piece).set(square);
                board.get_occupancy_bb_mut(side).set(square);

                file += 1;
            } else {
                return Err(ParseFenError::InvalidBoardPart(format!(
                    "Invalid piece character: {char_piece}"
                )));
            }
        }

        if file != 8 {
            return Err(ParseFenError::InvalidBoardPart(format!(
                "Rank {} does not have 8 squares: {}",
                rank + 1,
                rank_str
            )));
        }
    }
    Ok(board)
}

/// Parses the castling rights string into a bitmask
fn parse_castling_rights(fen_part: &str) -> Result<u8, ParseFenError> {
    let mut rights: u8 = 0;
    if fen_part == "-" {
        return Ok(rights);
    }

    for c in fen_part.chars() {
        match c {
            'K' => rights |= 1 << 0, // 0b0001
            'Q' => rights |= 1 << 1, // 0b0010
            'k' => rights |= 1 << 2, // 0b0100
            'q' => rights |= 1 << 3, // 0b1000
            _ => return Err(ParseFenError::InvalidCastlingPart(fen_part.to_string())),
        }
    }

    Ok(rights)
}

/// Parses the en passant target square string into an `Option<Square>`.
fn parse_en_passant_target(fen_part: &str) -> Result<Option<Square>, ParseFenError> {
    if fen_part == "-" {
        return Ok(None);
    }

    if fen_part.len() != 2 {
        return Err(ParseFenError::InvalidEnPassantPart(fen_part.to_string()));
    }

    let mut chars = fen_part.chars();

    let file_char = unsafe { chars.next().unwrap_unchecked() };
    let rank_char = unsafe { chars.next().unwrap_unchecked() };

    let file_idx = File::try_from(&file_char).ok();
    let rank_idx = Rank::try_from(&rank_char).ok();

    match (file_idx, rank_idx) {
        (Some(f), Some(r)) => {
            let square = Square::from_coords_u8(f.as_u8(), r.0);

            let target_rank = square.0 / 8;
            if target_rank != 2 && target_rank != 5 {
                return Err(ParseFenError::InvalidEnPassantPart(format!(
                    "En passant target square must be on rank 3 or 6: {fen_part}"
                )));
            }

            Ok(Some(square))
        }
        _ => Err(ParseFenError::InvalidEnPassantPart(fen_part.to_string())),
    }
}
