use littlewing::common::*;
use littlewing::bitboard::BitwiseOperations;

#[deriving(Copy, PartialEq)]
pub enum MoveCategory {
    QuietMove,
    DoublePawnPush,
    KingCastle,
    QueenCastle,
    Capture,
    EnPassant,
    NullMove,
    KnightPromotion = 8,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    KnightPromotionCapture,
    BishopPromotionCapture,
    RookPromotionCapture,
    QueenPromotionCapture
}

pub struct Move {
    from: Square,
    to: Square,
    _type: MoveType // FIXME
}

impl Move {
    pub fn new(f: Square, t: Square, mt: MoveType) -> Move {
        Move {
            from: f,
            to: t,
            _type: mt
        }
    }
}

pub type Moves = Vec<Move>;

pub trait MovesOperations {
    fn add_moves(&mut self, mut targets: Bitboard, dir: uint, mt: MoveType);
}

impl MovesOperations for Moves {
    fn add_moves(&mut self, mut targets: Bitboard, dir: uint, mt: MoveType) {
        while targets != 0 {
            let to = targets.ffs();
            let from = to - dir;
            let m = Move::new(from, to, mt);
            self.push(m);
            targets.reset(to);
        }
    }
}
