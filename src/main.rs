use schaakmaat::chess::{Chess, Outcome, Color, Piece};
use schaakmaat::computer;
use schaakmaat::pos::Pos;

const MATE_IN_THREE: Chess = Chess {
    board: [
        [
            Some(Piece::WHITE_QUEEN),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        [
            None,
            None,
            None,
            None,
            None,
            Some(Piece::BLACK_PAWN),
            None,
            None,
        ],
        [
            None,
            None,
            None,
            None,
            None,
            Some(Piece::WHITE_PAWN),
            None,
            Some(Piece::BLACK_PAWN),
        ],
        [
            None,
            None,
            None,
            None,
            None,
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_PAWN),
            Some(Piece::WHITE_KNIGHT),
        ],
        [
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::WHITE_PAWN),
            Some(Piece::BLACK_KING),
        ],
        [
            None,
            None,
            None,
            None,
            Some(Piece::WHITE_KNIGHT),
            None,
            Some(Piece::WHITE_ROOK),
            Some(Piece::BLACK_PAWN),
        ],
        [
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::WHITE_PAWN),
        ],
        [
            None,
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
    kings: [Pos::new(6, 7), Pos::new(7, 4)],
};

fn main() {
    // let mut state = MATE_IN_THREE;

    // println!("{state}");

    // let best_move = computer::minimax(&state, 5).m.unwrap();
    // state.perform(best_move);
    // println!("move: {:?} best score: {:?}", best_move, state.evaluate());
    // println!("{state}");

    // let best_move = computer::minimax(&state, 5).m.unwrap();
    // state.perform(best_move);
    // println!("move: {:?} best score: {:?}", best_move, state.evaluate()); 
    // println!("{state}");

    // let best_move = computer::minimax(&state, 5).m.unwrap();
    // state.perform(best_move);
    // println!("move: {:?} best score: {:?}", best_move, state.evaluate());
    // println!("{state}");

    // let best_move = computer::minimax(&state, 5).m.unwrap();
    // state.perform(best_move);
    // println!("move: {:?} best score: {:?}", best_move, state.evaluate());
    // println!("{state}");

    // let best_move = computer::minimax(&state, 5).m.unwrap();
    // state.perform(best_move);
    // println!("move: {:?} best score: {:?}", best_move, state.evaluate());
    // println!("{state}");

    let mut state = Chess::new();

    println!("{state}");

    while let Some(m) = computer::minimax(&state, 1).m {
        state.perform(m);
        println!("{state}");
    }

    match state.outcome().unwrap() {
        Outcome::Winner(color) => println!("{color} wins!"),
        Outcome::Stalemate => println!("it's a stalemate!"),
    }
}
