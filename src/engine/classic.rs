// Minimax + AB Pruning + Transposition Table

use std::cmp::{max, min};
use std::collections::HashMap;
use std::hash::Hash;

use crate::game::node::{GameResult, Node, Player};
use crate::game::*;

use super::Evaluation;

const CENTRAL_SQUARES: u64 = 0x0000393939390000;
const WHITE_SIDE: u64 = 0xffffffff00000000;
const BLACK_SIDE: u64 = 0x00000000ffffffff;

fn evaluate<N: Node<M> + Clone, M>(node: &N) -> Evaluation {
    let (white_bits, black_bits) = node.bitboards();
    let (white_material, black_material) = (
        white_bits.count_ones() as i64,
        black_bits.count_ones() as i64,
    );

    let (white_central, black_central) = (
        (white_bits & CENTRAL_SQUARES).count_ones() as i64,
        (black_bits & CENTRAL_SQUARES).count_ones() as i64,
    );

    let (white_in_black, black_in_white) = (
        (white_bits & BLACK_SIDE).count_ones() as i64,
        (black_bits & WHITE_SIDE).count_ones() as i64,
    );

    Evaluation::Heuristic(
        1000 * (white_material - black_material)
            + 150 * (white_central - black_central)
            + 350 * (white_in_black - black_in_white),
    )
}

// Count sharp features of a node
fn sharpness<N: Node<M>, M>(node: &N) -> u32 {
    let (white_bits, black_bits) = node.bitboards();

    // Nodes in their opponent's starting area are sharp
    let (black_in_white, white_in_black) = (white_bits & BLACK_START, black_bits & WHITE_START);
    // Nodes in tension are sharp
    // let ()

    return black_in_white.count_ones() + white_in_black.count_ones();
}

pub fn alpha_beta<M: Clone, N: Node<M> + Hash + Eq + Clone>(
    node: &N,
    depth: i32,
    alpha: Evaluation,
    beta: Evaluation,
    transposition_table: &mut HashMap<N, (Option<M>, Evaluation)>,
) -> (Option<M>, Evaluation) {
    if node.is_terminal() {
        return match node.get_result() {
            GameResult::Undecided => (None, evaluate(node)),
            GameResult::Win(Player::Black) => (None, Evaluation::BlackWinPly(node.ply())),
            GameResult::Win(Player::White) => (None, Evaluation::WhiteWinPly(node.ply())),
            GameResult::Draw => (None, Evaluation::Heuristic(0)),
        };
    }

    if transposition_table.contains_key(node) {
        return transposition_table.get(node).unwrap().clone();
    }

    let curr_sharpness = sharpness(node);
    if depth <= 0 && curr_sharpness == 0 {
        return (None, evaluate(node));
    }

    let (mut alpha, mut beta) = (alpha, beta);
    let mut children: Vec<(M, N)> = node
        .get_possible_actions()
        .into_iter()
        .map(|action| (action.clone(), node.take_action(&action)))
        .collect();
    children.sort_by_cached_key(|pos| (sharpness(&pos.1), if node.to_play() == Player::White {
        !evaluate(&pos.1)
    } else {
        evaluate(&pos.1)
    }));
    let mut result = match node.to_play() {
        Player::Black => (None, Evaluation::WhiteWinPly(node.ply())),
        Player::White => (None, Evaluation::BlackWinPly(node.ply())),
    };

    match node.to_play() {
        Player::White => {
            for child in children.into_iter() {
                if depth < 0 && sharpness(&child.1) >= curr_sharpness {
                    continue;
                }

                let (_, eval) = alpha_beta(&child.1, depth - 1, alpha, beta, transposition_table);
                if result.1 < eval {
                    result = (Some(child.0), eval);
                }
                if eval > beta {
                    break;
                }
                alpha = max(alpha, eval);
            }

            transposition_table.insert(node.clone(), result.clone());
        }
        Player::Black => {
            for action in node.get_possible_actions().into_iter() {
                let next = node.take_action(&action);
                if depth < 0 && sharpness(&next) >= curr_sharpness {
                    continue;
                }

                let (_, eval) = alpha_beta(&next, depth - 1, alpha, beta, transposition_table);
                if result.1 > eval {
                    result = (Some(action), eval);
                }
                if eval < alpha {
                    break;
                }
                beta = min(beta, eval);
            }

            transposition_table.insert(node.clone(), result.clone());
        }
    }
    result
}

pub fn get_move<M: Clone, N: Node<M> + Hash + Eq + Clone>(node: &N, depth: i32) -> (Option<M>, Evaluation) {
    let mut tt: HashMap<N, (Option<M>, Evaluation)> = HashMap::new();

    alpha_beta(
        node,
        depth,
        Evaluation::BlackWinPly(0),
        Evaluation::WhiteWinPly(0),
        &mut tt
    )
}
