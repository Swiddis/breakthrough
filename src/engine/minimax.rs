use super::Evaluation;
use crate::game::node::{GameResult, Node, Player};

pub fn get_move<M: Clone, N: Node<M>>(node: &N, depth: usize) -> (Option<M>, Evaluation) {
    if depth == 0 || node.is_terminal() {
        return match node.get_result() {
            GameResult::Undecided => (None, Evaluation::Heuristic(0)),
            GameResult::Win(Player::Black) => (None, Evaluation::BlackWinPly(node.ply())),
            GameResult::Win(Player::White) => (None, Evaluation::WhiteWinPly(node.ply())),
            GameResult::Draw => (None, Evaluation::Heuristic(0)),
        };
    }

    match node.to_play() {
        Player::White => {
            let mut result = (None, Evaluation::BlackWinPly(0));
            for action in node.get_possible_actions().into_iter() {
                let next = node.take_action(&action);
                let (_, eval) = get_move(&next, depth - 1);
                if result.1 < eval {
                    result = (Some(action), eval);
                }
            }

            result
        }
        Player::Black => {
            let mut result = (None, Evaluation::WhiteWinPly(0));
            for action in node.get_possible_actions().into_iter() {
                let next = node.take_action(&action);
                let (_, eval) = get_move(&next, depth - 1);
                if result.1 > eval {
                    result = (Some(action), eval);
                }
            }

            result
        }
    }
}
