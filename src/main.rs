use clap::{Parser, ValueEnum};
use rand::Rng;
use v2::core::node8::BreakthroughNode;

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
    GenerateEndgame {
        #[arg(long)]
        min_depth: u32,
        #[arg(long)]
        max_depth: u32,
        #[arg(short)]
        n: u32,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum PlayStrategy {
    Random,
    Minimax,
    Classic,
}
fn main() {
    let mut rng = rand::thread_rng();
    let mut board = BreakthroughNode::default();

    while !board.is_terminal() {
        println!("{}", board.fen());
        let actions = board.get_possible_actions();
        let choice = actions[rng.gen_range(0..actions.len())].clone();
        board = board.take_action(&choice);
    }
    println!("{}", board.fen());
}
