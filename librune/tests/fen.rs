#[cfg(test)]
mod tests {
    use librune::{
        board::{
            bitboard::Bitboard,
            defs::{STARTING_FEN, Square},
            fen::parse_fen,
            pieces::{ALL_PIECES, Pieces},
        },
        defs::Sides,
    };

    #[test]
    fn starting_pos_valid() {
        let parse = parse_fen(STARTING_FEN).expect("Starting FEN should always be valid!");

        assert_eq!(
            parse.get_occupancy_bb(Sides::WHITE) | parse.get_occupancy_bb(Sides::BLACK),
            Bitboard(0xFFFF_0000_0000_FFFF)
        );

        assert_eq!(
            parse.get_piece_bb(Sides::WHITE, Pieces::Pawn),
            Bitboard(0x0000_0000_0000_FF00)
        );
        assert_eq!(
            parse.get_piece_bb(Sides::BLACK, Pieces::Pawn),
            Bitboard(0x00FF_0000_0000_0000)
        );

        assert_eq!(
            parse.get_piece_bb(Sides::WHITE, Pieces::Rook),
            Bitboard(0x0000_0000_0000_0081)
        );
        assert_eq!(
            parse.get_piece_bb(Sides::BLACK, Pieces::Rook),
            Bitboard(0x8100_0000_0000_0000)
        );

        assert_eq!(
            parse.get_piece_bb(Sides::WHITE, Pieces::Knight),
            Bitboard(0x0000_0000_0000_0042)
        );
        assert_eq!(
            parse.get_piece_bb(Sides::BLACK, Pieces::Knight),
            Bitboard(0x4200_0000_0000_0000)
        );

        assert_eq!(
            parse.get_piece_bb(Sides::WHITE, Pieces::Bishop),
            Bitboard(0x0000_0000_0000_0024)
        );
        assert_eq!(
            parse.get_piece_bb(Sides::BLACK, Pieces::Bishop),
            Bitboard(0x2400_0000_0000_0000)
        );

        assert_eq!(
            parse.get_piece_bb(Sides::WHITE, Pieces::Queen),
            Bitboard(0x0000_0000_0000_0008)
        );
        assert_eq!(
            parse.get_piece_bb(Sides::BLACK, Pieces::Queen),
            Bitboard(0x0800_0000_0000_0000)
        );

        assert_eq!(
            parse.get_piece_bb(Sides::WHITE, Pieces::King),
            Bitboard(0x0000_0000_0000_0010)
        );
        assert_eq!(
            parse.get_piece_bb(Sides::BLACK, Pieces::King),
            Bitboard(0x1000_0000_0000_0000)
        );
    }

    #[test]
    fn side_to_move_white() {
        let parse = parse_fen("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();
        assert_eq!(parse.game_state.side_to_move, Sides::WHITE);
    }

    #[test]
    fn side_to_move_black() {
        let parse = parse_fen("8/8/8/8/8/8/8/8 b - - 0 1").unwrap();
        assert_eq!(parse.game_state.side_to_move, Sides::BLACK);
    }

    #[test]
    fn castling_rights_parsed() {
        let parse = parse_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        assert_eq!(parse.game_state.castling_rights, 0b1111);
    }

    #[test]
    fn no_castling_rights() {
        let parse = parse_fen("r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1").unwrap();
        assert_eq!(parse.game_state.castling_rights, 0b0);
    }

    #[test]
    fn en_passant_square_parsed() {
        let parse = parse_fen("8/8/8/3pP3/8/8/8/8 w - d6 0 2").unwrap();
        assert_eq!(parse.game_state.en_passant_target, Some(Square(43)));
    }

    #[test]
    fn invalid_fen_rejected() {
        assert!(parse_fen("8/8/8/8/8/8/8/9 w - - 0 1").is_err());
        assert!(parse_fen("8/8/8/8/8/8/8/8 x - - 0 1").is_err());
        assert!(parse_fen("8/8/8/8/8/8/8/8 w - -").is_err());
    }

    #[test]
    fn empty_board_valid() {
        let parse = parse_fen("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();
        for side in [Sides::WHITE, Sides::BLACK] {
            for piece in ALL_PIECES {
                assert_eq!(parse.get_piece_bb(side, piece), Bitboard(0));
            }
        }
    }
}
