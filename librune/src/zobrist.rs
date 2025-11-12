use crate::defs::{EMPTY, NrOf, Piece, Side, Sides, Square};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

type PieceRandoms = [[[u64; NrOf::SQUARES]; NrOf::PIECE_TYPES]; Sides::BOTH as usize];
type CastlingRandoms = [u64; NrOf::CASTLING_PERMISSIONS];
type SideRandoms = [u64; Sides::BOTH as usize];
type EpRandoms = [u64; NrOf::SQUARES + 1];

pub type ZobristKey = u64;

const RNG_SEED: [u8; 32] = [125; 32];

pub struct ZobristRandoms {
    pieces: PieceRandoms,
    castling: CastlingRandoms,
    sides: SideRandoms,
    en_passant: EpRandoms,
}

impl Default for ZobristRandoms {
    fn default() -> Self {
        Self::new()
    }
}

impl ZobristRandoms {
    #[must_use]
    pub fn new() -> Self {
        let mut random = ChaChaRng::from_seed(RNG_SEED);
        let mut zobrist_randoms = Self {
            pieces: [[[EMPTY; NrOf::SQUARES]; NrOf::PIECE_TYPES]; Sides::BOTH as usize],
            castling: [EMPTY; NrOf::CASTLING_PERMISSIONS],
            sides: [EMPTY; Sides::BOTH as usize],
            en_passant: [EMPTY; NrOf::SQUARES + 1],
        };

        zobrist_randoms.pieces.iter_mut().for_each(|side| {
            for piece in side.iter_mut() {
                for square in piece.iter_mut() {
                    *square = random.random::<u64>();
                }
            }
        });

        zobrist_randoms
            .castling
            .iter_mut()
            .for_each(|permission| *permission = random.random::<u64>());

        zobrist_randoms
            .sides
            .iter_mut()
            .for_each(|side| *side = random.random::<u64>());

        zobrist_randoms
            .en_passant
            .iter_mut()
            .for_each(|ep| *ep = random.random::<u64>());

        zobrist_randoms
    }

    #[must_use]
    pub fn piece(&self, side: Side, piece: Piece, square: Square) -> ZobristKey {
        self.pieces[side][piece][square]
    }

    #[must_use]
    pub fn castling(&self, castling_permissions: u8) -> ZobristKey {
        self.castling[castling_permissions as usize]
    }

    #[must_use]
    pub fn side(&self, side: Side) -> u64 {
        self.sides[side]
    }

    #[must_use]
    pub fn en_passant(&self, en_passant: Option<u8>) -> ZobristKey {
        match en_passant {
            Some(ep) => self.en_passant[ep as usize],
            None => self.en_passant[NrOf::SQUARES],
        }
    }
}
