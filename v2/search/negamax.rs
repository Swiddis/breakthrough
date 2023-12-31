use std::cmp::max;

use crate::{
    core::{
        constants8::{BLACK_FIRST_ROW, BLACK_START, WHITE_FIRST_ROW, WHITE_START},
        move8::BreakthroughMove,
        node8::BreakthroughNode,
        Evaluation, GameResult, Player,
    },
    evaluation::fast_heuristic,
    search::table::TranspositionTable,
};

// Attempt to evaluate the current node
// Return negative if the game is over since the current player has already lost
// Otherwise return positive
fn evaluate_result(node: &BreakthroughNode) -> Evaluation {
    match node.get_result() {
        GameResult::Win(Player::White) => Evaluation::BlackWinPly(node.ply),
        GameResult::Win(Player::Black) => Evaluation::BlackWinPly(node.ply),
        GameResult::Undecided => match node.to_play {
            Player::White => fast_heuristic(node),
            Player::Black => -fast_heuristic(node),
        },
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
fn get_filtered_actions(node: &BreakthroughNode) -> Vec<BreakthroughMove> {
    let actions: Vec<BreakthroughMove> = node
        .get_possible_actions()
        .into_iter()
        .filter(|action| {
            // If opponent is threatening a win, we can only recapture
            match node.to_play {
                Player::White => {
                    if node.bitboard_black & WHITE_START == 0 {
                        true
                    } else {
                        (1 << action.1) & node.bitboard_black & WHITE_START > 0
                    }
                }
                Player::Black => {
                    if node.bitboard_white & BLACK_START == 0 {
                        true
                    } else {
                        (1 << action.1) & node.bitboard_white & BLACK_START > 0
                    }
                }
            }
        })
        .collect();

    actions
}

fn guess_priority(
    source_square: u64,
    target_square: u64,
    opp_start: u64,
    opp_side: u64,
    self_base: u64,
    any_bitboard: u64,
) -> i32 {
    if target_square & opp_start > 0 {
        // Priority 0: Almost-winning moves
        0
    } else if target_square & any_bitboard > 0 {
        // Priority 1: Captures
        1
    } else if target_square & opp_side > 0 {
        // Priority 2: Entering the opponent's side of the board
        2
    } else if (source_square & self_base) > 0 {
        // Avoid moving pieces from the start row unless necessary
        100
    } else {
        50
    }
}

fn prioritize_actions(node: &BreakthroughNode, actions: &mut [BreakthroughMove]) {
    actions.sort_unstable_by_key(|action| {
        let (opp_start, opp_side, self_base) = match node.to_play {
            Player::White => (
                BLACK_START,
                BLACK_START | (BLACK_START << 16),
                WHITE_FIRST_ROW,
            ),
            Player::Black => (
                WHITE_START,
                WHITE_START | (WHITE_START >> 16),
                BLACK_FIRST_ROW,
            ),
        };
        // Prioritize seen nodes before new ones
        guess_priority(
            1 << action.0,
            1 << action.1,
            opp_start,
            opp_side,
            self_base,
            node.bitboard_white | node.bitboard_black,
        )
    });
}

pub fn negamax(
    node: &BreakthroughNode,
    depth: u32,
    alpha: Evaluation,
    beta: Evaluation,
    table: &mut TranspositionTable,
) -> (Option<BreakthroughMove>, Evaluation) {
    if node.is_terminal() || depth == 0 {
        return (None, evaluate_result(node));
    }

    // If we don't need a move, try non-move search pruning
    // TODO we should actually store/compute the move and return it here
    if let Some(eval) = fast_win(node) {
        return (None, eval);
    }

    if let Some(entry) = table.get(node, depth) {
        return (None, entry.2);
    }

    let mut actions = get_filtered_actions(node);
    if actions.is_empty() {
        // If there's no reasonable actions, the opponent wins in the next turn
        // Add 2 since lose state is on our next turn
        return (None, Evaluation::BlackWinPly(node.ply + 2));
    }
    // Skip expensive prioritization of near-leaf nodes
    if depth > 1 {
        prioritize_actions(node, &mut actions);
    }

    let (mut alpha, beta) = (alpha, beta);
    let mut value = (None, Evaluation::BlackWinPly(node.ply));
    for action in actions.iter() {
        let child = node.take_action(action);
        let eval = negamax(
            &child,
            depth - 1,
            -beta,
            -alpha,
            table,
        );
        if -eval.1 > value.1 {
            value = (Some(action), -eval.1);
        }
        alpha = max(alpha, value.1);
        if alpha >= beta {
            break;
        }
    }

    table.put((node.clone(), depth, value.1));
    (value.0.cloned(), value.1)
}
