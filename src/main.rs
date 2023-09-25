pub mod game;

use rand::prelude::*;

use crate::game::{breakthrough::BreakthroughNode, node::Node};

fn generate_playout() -> Vec<BreakthroughNode> {
    let mut rng = rand::thread_rng();
    let mut board = BreakthroughNode::default();
    let mut result = vec![board.clone()];
    while !board.is_terminal() {
        let actions = board.get_possible_actions();
        let action = &actions[rng.gen_range(0..actions.len())];
        board = board.take_action(action);
        result.push(board.clone());
    }
    result
}

fn main() {
    let playout = generate_playout();

    for node in playout.into_iter() {
        println!("{}", node.to_string());
        println!("\nTo move: {:?}", node.to_play);
    }
}
