use clap::{Parser, ValueEnum};
use rand::Rng;
use v2::{core::node8::BreakthroughNode, search::{evaluate_with_ttable, table::TranspositionTable}};

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
        #[arg(long, default_value = "8")]
        depth: u32,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum PlayStrategy {
    Random,
    V2,
}

fn do_selfplay(strategy: PlayStrategy, depth: u32) {
    match strategy {
        PlayStrategy::Random => todo!(),
        PlayStrategy::V2 => {
            let mut node = BreakthroughNode::default();
            let mut ttable = TranspositionTable::new(2usize.pow(22));

            println!("{}\n{}", node.fen(), node.to_string());

            while !node.is_terminal() {
                let (action, eval) = evaluate_with_ttable(&node, depth, &mut ttable);
                println!("({}, {:?})", action.to_string(), eval);
                node = node.take_action(&action);
                println!("{}\n{}", node.fen(), node.to_string());
            }
        },
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Selfplay { strategy, depth } => {
            do_selfplay(strategy, depth);
        }
    }
}
