use crate::game::node::Node;

use super::Evaluation;

use rand::prelude::*;

pub fn get_move<M: Clone, N: Node<M>>(node: &N) -> (Option<M>, Evaluation) {
    if node.is_terminal() {
        return (None, Evaluation::Heuristic(0));
    }
    let mut rng = thread_rng();
    let actions = node.get_possible_actions();
    let action = actions[rng.gen_range(0..actions.len())].clone();
    (Some(action), Evaluation::Heuristic(0))
}
