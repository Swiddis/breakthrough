pub mod game;

use rand::prelude::*;

use crate::game::{breakthrough::BreakthroughNode, node::Node};

fn main() {
    let mut board = BreakthroughNode::default();
    let mut rng = rand::thread_rng();
    println!("{}", board.to_string());

    while !board.is_terminal() {
        println!("\nTo move: {:?}", board.to_play);
        let actions = board.get_possible_actions();
        let action = &actions[rng.gen_range(0..actions.len())];
        println!("Action: {:?}", action);
        board = board.take_action(action);
        println!("{}", board.to_string());
    }

    println!("Result: {:?}", board.get_result());
}
