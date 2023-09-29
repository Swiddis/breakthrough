use crate::game::{
    breakthrough::{BreakthroughMove, BreakthroughNode},
    node::{GameResult, Node, Player},
};

use super::Evaluation::{self, *};

use std::{
    cmp::{max, min, Ord, Ordering},
    collections::HashMap,
};

type TTable<'a> = HashMap<BreakthroughNode, Evaluation>;

fn fast_return(
    node: &BreakthroughNode,
    depth: u32,
    alpha: Evaluation,
    beta: Evaluation,
    ttable: &TTable,
) -> Option<Evaluation> {
    if depth == 0 || node.is_terminal() {
        return match node.get_result() {
            GameResult::Undecided => Some(Heuristic(0)),
            GameResult::Win(Player::Black) => Some(Evaluation::BlackWinPly(node.ply())),
            GameResult::Win(Player::White) => Some(Evaluation::WhiteWinPly(node.ply())),
            GameResult::Draw => Some(Heuristic(0)),
        };
    }
    match alpha {
        WhiteWinPly(n) => if node.ply() > n {
            return Some(alpha);
        },
        _ => {}
    };
    match beta {
        BlackWinPly(n) => if node.ply() > n {
            return Some(beta);
        },
        _ => {}
    };
    match ttable.get(node) {
        None => None,
        Some(Heuristic(_)) => None,
        Some(eval) => Some(*eval),
    }
}

fn min_win_moves(node: &BreakthroughNode) -> (u32, u32) {
    let (white_bb, black_bb) = node.bitboards();
    (white_bb.leading_zeros() / 8, black_bb.trailing_zeros() / 8)
}

fn early_cut(node: &BreakthroughNode, depth: u32) -> bool {
    let (white_min, black_min) = min_win_moves(node);
    match node.to_play() {
        Player::White => white_min * 2 + 1 > depth && black_min * 2 > depth,
        Player::Black => black_min * 2 + 1 > depth && white_min * 2 > depth,
    }
}

fn order_actions(node: &BreakthroughNode, actions: &mut Vec<BreakthroughMove>) {
    let (white_min, black_min) = min_win_moves(node);
    // If white is attacking, prioritize moves on the black side (end)
    match white_min.cmp(&black_min) {
        Ordering::Less => {
            // White is attacking, prioritize squares near the black side
            actions.sort_by_key(|a| a.1);
        }
        Ordering::Greater => {
            // Black is attacking, prioritize squares near the white side
            actions.sort_by_key(|a| u8::MAX - a.1);
        }
        Ordering::Equal => match node.to_play() {
            Player::White => {
                // White has the initiative and can attack
                actions.sort_by_key(|a| a.1);
            }
            Player::Black => {
                // Black has the initiative
                actions.sort_by_key(|a| u8::MAX - a.1);
            }
        },
    }
}

fn alpha_beta(
    node: &BreakthroughNode,
    depth: u32,
    alpha: Evaluation,
    beta: Evaluation,
    ttable: &mut TTable,
) -> Evaluation {
    match fast_return(node, depth, alpha, beta, ttable) {
        None => (),
        Some(eval) => {
            return eval;
        }
    };

    if early_cut(node, depth) {
        return Heuristic(0);
    }

    let mut actions = node.get_possible_actions();
    order_actions(node, &mut actions);

    let (mut alpha, mut beta) = (alpha, beta);
    let mut result: Evaluation;
    match node.to_play() {
        Player::White => {
            result = Evaluation::BlackWinPly(0);
            for action in actions.into_iter() {
                let next = node.take_action(&action);
                result = max(result, alpha_beta(&next, depth - 1, alpha, beta, ttable));
                if result > beta {
                    break;
                }
                alpha = max(alpha, result);
            }
        }
        Player::Black => {
            result = Evaluation::WhiteWinPly(0);
            for action in actions.into_iter() {
                let next = node.take_action(&action);
                result = min(result, alpha_beta(&next, depth - 1, alpha, beta, ttable));
                if result < alpha {
                    break;
                }
                beta = min(beta, result);
            }
        }
    };

    ttable.insert(node.clone(), result);
    result
}

pub fn get_move(node: &BreakthroughNode, depth: u32) -> Evaluation {
    let mut ttable: TTable = HashMap::new();
    alpha_beta(node, depth, BlackWinPly(0), WhiteWinPly(0), &mut ttable)
}
