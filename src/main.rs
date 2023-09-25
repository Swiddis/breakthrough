pub mod game;
pub mod engine;

use clap::{Parser, ValueEnum};

use crate::{game::{breakthrough::BreakthroughNode, node::Node}, engine::{minimax::get_move, random}};

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
        },
        PlayStrategy::Minimax => {
            let mut node = BreakthroughNode::default();

            while !node.is_terminal() {
                println!("{}\nTo play: {:?}", node.to_string(), node.to_play());
                let (action, eval) = get_move(&node, 6);
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
