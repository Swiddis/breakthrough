use std::cmp::max;

use crate::{
    core::{
        constants8::{BLACK_START, WHITE_START},
        node8::BreakthroughNode,
        Evaluation, GameResult, Player, move8::BreakthroughMove,
    },
    evaluation::fast_heuristic,
};

pub mod table;

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
    // We detect captures lazily, a destination on any piece is a capture
    let any_bitboard = node.bitboard_white | node.bitboard_black;

    let actions = node.get_possible_actions();

    let mut actions: Vec<BreakthroughMove> = actions.into_iter().filter(|action| {
        // If opponent is threatening a win, we can only recapture
        match node.to_play {
            Player::White => if node.bitboard_black & WHITE_START == 0 {
                true
            } else {
                (1 << action.1) & node.bitboard_black & WHITE_START > 0
            },
            Player::Black => if node.bitboard_white & BLACK_START == 0 {
                true
            } else {
                (1 << action.1) & node.bitboard_white & BLACK_START > 0 
            }
        }
    }).collect();

    actions.sort_unstable_by_key(|action| {
        let target_square = 1 << action.1;
        let (opp_start, opp_side) = match node.to_play {
            Player::White => (BLACK_START, BLACK_START | (BLACK_START << 16)),
            Player::Black => (WHITE_START, WHITE_START | (WHITE_START >> 16))
        };
        if target_square & opp_start > 0 {
            // Priority 0: Almost-winning moves
            0
        } else if target_square & any_bitboard > 0 {
            // Priority 1: Captures
            1
        } else if target_square & opp_side > 0 {
            // Priority 2: Entering the opponent's side of the board
            2
        } else {
            3
        }
    });

    actions
}

fn negamax(node: &BreakthroughNode, depth: u32, alpha: Evaluation, beta: Evaluation) -> Evaluation {
    if node.is_terminal() || depth == 0 {
        return evaluate_result(node);
    }
    if let Some(eval) = fast_win(node) {
        return eval;
    }

    let actions = get_prioritized_actions(node);
    if actions.is_empty() {
        // If there's no reasonable actions, the opponent wins in the next turn
        // Add 2 since lose state is on our next turn
        return Evaluation::BlackWinPly(node.ply + 2);
    }

    let (mut alpha, beta) = (alpha, beta);
    let mut value = Evaluation::BlackWinPly(node.ply);
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
