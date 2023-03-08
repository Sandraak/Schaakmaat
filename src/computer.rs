use crate::chess::{Chess, Move};

pub struct BestMove {
    pub m: Option<Move>,
    score: i16,
}

pub fn minimax(chess: &Chess, depth: u8) -> BestMove {
    if (depth == 0) || chess.outcome().is_some() {
        BestMove {
            m: None,
            score: chess.evaluate(),
        }
    } else {
        let mut best_move = None;
        let mut best_score = None;

        for m in chess.moves() {
            let mut copy = *chess;
            copy.perform(m);
            let score = minimax(&copy, depth - 1).score;
            if chess.turn.improves(score, best_score) {
                best_score = Some(score);
                best_move = Some(m);
            }
        }

        BestMove {
            m: Some(best_move.unwrap()),
            score: best_score.unwrap(),
        }
    }
}
