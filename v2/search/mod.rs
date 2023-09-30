use crate::{
    core::{
        constants8::{BLACK_START, WHITE_START},
        node8::BreakthroughNode,
        Evaluation, GameResult, Player,
    },
    evaluation::fast_heuristic,
};

// Attempt to evaluate the current node
// Return negative if the game is over since the current player has already lost
// Otherwise return positive
fn evaluate(node: &BreakthroughNode) -> Evaluation {
    match node.get_result() {
        GameResult::Win(Player::White) => Evaluation::BlackWinPly(node.ply),
        GameResult::Win(Player::Black) => Evaluation::BlackWinPly(node.ply),
        GameResult::Undecided => fast_heuristic(node),
    }
}

// Check if the node is immediately winnable
// Return positive since the current player is taking the action
fn fast_win(node: &BreakthroughNode) -> Option<Evaluation> {
    match node.to_play {
        Player::White => {
            if node.bitboard_white & BLACK_START > 0 {
                Some(Evaluation::WhiteWinPly(node.ply + 1))
            } else {
                None
            }
        }
        Player::Black => {
            if node.bitboard_black & WHITE_START > 0 {
                Some(Evaluation::WhiteWinPly(node.ply + 1))
            } else {
                None
            }
        }
    }
}

pub fn negamax(node: &BreakthroughNode, depth: u32) -> Evaluation {
    if node.is_terminal() || depth == 0 {
        return evaluate(node);
    }
    if let Some(eval) = fast_win(node) {
        return eval;
    }

    let best_eval = node.get_possible_actions().iter().map(|action| {
        let child = node.take_action(action);
        -negamax(&child, depth - 1)
    }).max();

    best_eval.unwrap()
}
