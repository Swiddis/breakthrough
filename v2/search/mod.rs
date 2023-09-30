use std::cmp::max;

use crate::{
    core::{
        constants8::{BLACK_START, WHITE_START},
        node8::BreakthroughNode,
        Evaluation, GameResult, Player, move8::BreakthroughMove,
    },
    evaluation::fast_heuristic,
};

// Attempt to evaluate the current node
// Return negative if the game is over since the current player has already lost
// Otherwise return positive
fn evaluate_result(node: &BreakthroughNode) -> Evaluation {
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

// Moves sorted by priority and filtered to remove obvious losing moves
fn get_prioritized_actions(node: &BreakthroughNode) -> Vec<BreakthroughMove> {
    node.get_possible_actions()
}

fn negamax(node: &BreakthroughNode, depth: u32, alpha: Evaluation, beta: Evaluation) -> Evaluation {
    if node.is_terminal() || depth == 0 {
        return evaluate_result(node);
    }
    if let Some(eval) = fast_win(node) {
        return eval;
    }

    let (mut alpha, beta) = (alpha, beta);
    let mut value = Evaluation::BlackWinPly(node.ply);
    let actions = get_prioritized_actions(node);
    for action in actions.iter() {
        let child = node.take_action(action);
        value = max(value, -negamax(&child, depth - 1, -beta, -alpha));
        alpha = max(alpha, value);
        if alpha >= beta {
            break;
        }
    }

    value
}

pub fn evaluate(node: &BreakthroughNode, depth: u32) -> Evaluation {
    negamax(node, depth, Evaluation::BlackWinPly(node.ply), Evaluation::WhiteWinPly(node.ply))
}
