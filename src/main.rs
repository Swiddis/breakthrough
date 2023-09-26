pub mod engine;
pub mod game;

use clap::{Parser, ValueEnum};

use crate::{
    engine::{classic, minimax, random},
    game::{
        breakthrough::BreakthroughNode,
        node::{Node, Player},
    },
};

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
    Classic,
}

fn selfplay(strategy: PlayStrategy) {
    match strategy {
        PlayStrategy::Random => {
            let mut node = BreakthroughNode::default();

            while !node.is_terminal() {
                println!("{}\nTo play: {:?}", node.to_string(), node.to_play());
                let (action, eval) = random::get_move(&node);
                println!("{:?}\n", eval);

                node = node.take_action(&action.unwrap());
            }

            println!("{}", node.to_string());
        }
        PlayStrategy::Minimax => {
            let mut node = BreakthroughNode::default();

            while !node.is_terminal() {
                println!("{}\nTo play: {:?}", node.to_string(), node.to_play());
                let (action, eval) = minimax::get_move(&node, 6);
                println!("{:?}\n", eval);

                node = node.take_action(&action.unwrap());
            }

            println!("{}", node.to_string());
        }
        PlayStrategy::Classic => {
            let mut node = BreakthroughNode::default();

            while !node.is_terminal() {
                println!("{}\nTo play: {:?}", node.to_string(), node.to_play());
                let (action, eval) = classic::get_move(&node, 5);
                println!("{:?}\n", eval);

                node = node.take_action(&action.unwrap());
            }

            println!("{}", node.to_string());
        }
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Selfplay { strategy } => selfplay(strategy),
    }
}
