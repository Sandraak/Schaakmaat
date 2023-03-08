#![feature(test)]

extern crate test;

use test::Bencher;

use schaakmaat::chess::{Chess, Color, Outcome, Piece};
use schaakmaat::computer;
use schaakmaat::pos::Pos;

const MATE_IN_THREE: Chess = Chess {
    board: [
        [
            Some(Piece::BLACK_ROOK),
            None,
            Some(Piece::BLACK_BISHOP),
            Some(Piece::BLACK_QUEEN),
            None,
            Some(Piece::BLACK_KNIGHT),
            None,
            Some(Piece::BLACK_ROOK),
        ],
        [
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_PAWN),
            None,
            None,
            None,
            Some(Piece::WHITE_QUEEN),
            Some(Piece::BLACK_PAWN),
            Some(Piece::WHITE_BISHOP),
        ],
        [
            None,
            None,
            Some(Piece::BLACK_PAWN),
            None,
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_BISHOP),
            None,
            None,
        ],
        [
            None,
            None,
            None,
            None,
            None,
            Some(Piece::WHITE_ROOK),
            None,
            None,
        ],
        [
            None,
            None,
            Some(Piece::BLACK_PAWN),
            Some(Piece::WHITE_PAWN),
            Some(Piece::BLACK_KING),
            None,
            None,
            None,
        ],
        [None, None, None, None, None, None, None, None],
        [
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
            None,
            None,
            None,
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
        ],
        [
            Some(Piece::WHITE_ROOK),
            None,
            None,
            None,
            None,
            None,
            Some(Piece::WHITE_KING),
            None,
        ],
    ],
    turn: Color::White,
    kings: [Pos::new(6, 7), Pos::new(4, 5)],
};

const MATE_IN_TWO: Chess = Chess {
    board: [
        [
            None,
            None,
            None,
            Some(Piece::BLACK_ROOK),
            None,
            None,
            Some(Piece::BLACK_ROOK),
            Some(Piece::BLACK_KING),
        ],
        [
            Some(Piece::BLACK_PAWN),
            None,
            None,
            None,
            None,
            Some(Piece::BLACK_PAWN),
            None,
            Some(Piece::BLACK_PAWN),
        ],
        [
            None,
            None,
            None,
            Some(Piece::BLACK_PAWN),
            None,
            Some(Piece::WHITE_PAWN),
            Some(Piece::BLACK_PAWN),
            None,
        ],
        [
            None,
            None,
            None,
            Some(Piece::WHITE_ROOK),
            None,
            None,
            None,
            None,
        ],
        [
            None,
            None,
            Some(Piece::BLACK_PAWN),
            None,
            Some(Piece::WHITE_BISHOP),
            None,
            None,
            Some(Piece::WHITE_QUEEN),
        ],
        [None, None, None, None, None, None, None, None],
        [
            None,
            Some(Piece::BLACK_QUEEN),
            None,
            None,
            None,
            None,
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
        ],
        [
            None,
            None,
            None,
            None,
            Some(Piece::WHITE_ROOK),
            None,
            Some(Piece::WHITE_KING),
            None,
        ],
    ],
    turn: Color::White,
    kings: [Pos::new(6, 7), Pos::new(7, 0)],
};

const MATE_IN_ONE: Chess = Chess {
    board: [
        [
            Some(Piece::BLACK_ROOK),
            None,
            Some(Piece::BLACK_BISHOP),
            None,
            None,
            Some(Piece::BLACK_ROOK),
            Some(Piece::BLACK_KING),
            None,
        ],
        [
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_PAWN),
            Some(Piece::BLACK_PAWN),
            None,
            None,
            Some(Piece::BLACK_PAWN),
            None,
        ],
        [None, None, None, None, None, None, None, None],
        [
            None,
            None,
            None,
            Some(Piece::BLACK_QUEEN),
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_KNIGHT),
            None,
            Some(Piece::WHITE_QUEEN),
        ],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [
            Some(Piece::WHITE_PAWN),
            None,
            None,
            None,
            None,
            None,
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
        ],
        [
            Some(Piece::BLACK_BISHOP),
            None,
            Some(Piece::WHITE_BISHOP),
            None,
            None,
            Some(Piece::WHITE_ROOK),
            None,
            Some(Piece::WHITE_KING),
        ],
    ],
    turn: Color::White,
    kings: [Pos::new(7, 7), Pos::new(6, 0)],
};

#[bench]
#[ignore]
fn mate_in_three(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut state = MATE_IN_THREE;

        for _ in 0..5 {
            let best_move = computer::minimax(&state, 5).m.unwrap();
            state.perform(best_move);
        }

        assert_eq!(state.outcome(), Some(Outcome::Winner(Color::White)));
    });
}

#[bench]
fn mate_in_two(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut state = MATE_IN_TWO;

        for _ in 0..3 {
            let best_move = computer::minimax(&state, 3).m.unwrap();
            state.perform(best_move);
        }

        assert_eq!(state.outcome(), Some(Outcome::Winner(Color::White)));
    });
}

#[bench]
fn mate_in_one(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut state = MATE_IN_ONE;

        for _ in 0..1 {
            let best_move = computer::minimax(&state, 1).m.unwrap();
            state.perform(best_move);
        }

        assert_eq!(state.outcome(), Some(Outcome::Winner(Color::White)));
    });
}
