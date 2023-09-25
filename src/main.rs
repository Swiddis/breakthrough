pub mod game;
pub mod engine;

use clap::{Parser, ValueEnum};
use rand::prelude::*;

use crate::{game::{breakthrough::BreakthroughNode, node::Node}, engine::minimax::get_move};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    Selfplay {
        #[arg(long, default_value = "random")]
        strategy: PlayStrategy,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum PlayStrategy {
    Random,
    Minimax,
}

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

fn selfplay(strategy: PlayStrategy) {
    match strategy {
        PlayStrategy::Random => {
            let playout = generate_playout();

            for node in playout.into_iter() {
                println!("{}", node.to_string());
                if node.is_terminal() {
                    println!("\nResult: {:?}", node.get_result());
                } else {
                    println!("\nTo move: {:?}", node.to_play());
                }
            }
        },
        PlayStrategy::Minimax => {
            let mut node = BreakthroughNode::default();

            while !node.is_terminal() {
                println!("{}\nTo play: {:?}", node.to_string(), node.to_play());
                let (action, eval) = get_move(&node, 6);
                println!("{:?}\n", eval);

                node = node.take_action(&action.unwrap());
            }
        }
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Selfplay { strategy } => selfplay(strategy),
    }
}
