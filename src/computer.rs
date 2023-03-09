use crate::chess::{Chess, Color, Move};

pub struct BestMove {
    pub m: Option<Move>,
    score: i16,
}

pub fn minimax(chess: &Chess, depth: u8, mut alpha: i16, mut beta: i16) -> BestMove {
    if (depth == 0) || chess.outcome().is_some() {
        BestMove {
            m: None,
            score: chess.evaluate(),
        }
    } else if chess.turn == Color::White {
        let mut best_move = None;
        let mut best_score = i16::MIN;

        for m in chess.moves() {
            let mut copy = *chess;
            copy.perform(m);
            let score = minimax(&copy, depth - 1, alpha, beta).score;
            if score > best_score || best_move.is_none() {
                best_score = score;
                best_move = Some(m);
                if score > alpha {
                    alpha = score;
                }
                if alpha >= beta {
                    break;
                }
            }
        }

        BestMove {
            m: Some(best_move.unwrap()),
            score: best_score,
        }
    } else {
        let mut best_move = None;
        let mut best_score = i16::MAX;

        for m in chess.moves() {
            let mut copy = *chess;
            copy.perform(m);
            let score = minimax(&copy, depth - 1, alpha, beta).score;
            if score < best_score || best_move.is_none() {
                best_score = score;
                best_move = Some(m);
                if score < beta {
                    beta = score;
                }
                if alpha >= beta {
                    break;
                }
            }
        }

        BestMove {
            m: Some(best_move.unwrap()),
            score: best_score,
        }
    }
}
