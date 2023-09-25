use crate::game::node::{Node, GameResult, Player};
use super::Evaluation;

pub fn get_move<M: Clone, N: Node<M>>(node: &N, depth: usize) -> (Option<M>, Evaluation) {
    if depth == 0 || node.is_terminal() {
        return match node.get_result() {
            GameResult::Undecided => (None, Evaluation::Heuristic(0)),
            GameResult::Win(Player::Black) => (None, Evaluation::BlackMate(0)),
            GameResult::Win(Player::White) => (None, Evaluation::WhiteMate(0)),
            GameResult::Draw => (None, Evaluation::Heuristic(0)),
        };
    }
;
    match node.to_play() {
        Player::White => {
            let mut result = (None, Evaluation::BlackMate(0));
            for action in node.get_possible_actions().into_iter() {
                let next = node.take_action(&action);
                let (_, eval) = get_move(&next, depth - 1);
                if result.1 < eval {
                    result = (Some(action), eval);
                }
            }

            match result.1 {
                Evaluation::WhiteMate(n) => (result.0, Evaluation::WhiteMate(n + 1)),
                _ => result
            }
        },
        Player::Black => {
            let mut result = (None, Evaluation::WhiteMate(0));
            for action in node.get_possible_actions().into_iter() {
                let next = node.take_action(&action);
                let (_, eval) = get_move(&next, depth - 1);
                if result.1 > eval {
                    result = (Some(action), eval);
                }
            }

            match result.1 {
                Evaluation::BlackMate(n) => (result.0, Evaluation::BlackMate(n + 1)),
                _ => result
            }
        },
    }
}
