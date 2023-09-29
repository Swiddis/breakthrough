pub mod engine;
pub mod game;

use clap::{Parser, ValueEnum};
use rand::Rng;

use crate::{
    engine::{classic, minimax, random, fast_win_check},
    game::{breakthrough::BreakthroughMove, breakthrough::BreakthroughNode, node::Node},
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
    GenerateEndgame {
        #[arg(long)]
        min_depth: u32,
        #[arg(long)]
        max_depth: u32,
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
                let action = action.unwrap();
                println!("{:?}\n{}", eval, action.to_string());

                node = node.take_action(&action);
            }

            println!("{}", node.to_string());
        }
    }
}

fn generate_endgame(min_depth: u32, max_depth: u32) {
    let mut node = BreakthroughNode::default();
    let mut moves: Vec<BreakthroughMove> = Vec::new();
    let mut history: Vec<BreakthroughNode> = Vec::new();
    let mut rng = rand::thread_rng();

    loop {
        // Play until a game ends
        println!("Searching for terminal node");
        while !node.is_terminal() {
            let actions = node.get_possible_actions();
            let action = &actions[rng.gen_range(0..actions.len())];
            node = node.take_action(action);
            moves.push(action.clone());
            history.push(node.clone());
        }
        // Rollback min_depth moves
        let cutoff = history.len() - (min_depth as usize);
        moves.truncate(cutoff);
        history.truncate(cutoff);
        node = history[history.len() - 1].clone();
        println!("{:?}\n{}", node.to_play(), node.to_string());
        // Is it a puzzle?
        println!("Checking potential puzzle depth");
        match fast_win_check::get_move(&node, max_depth) {
            // If it's not an endgame, generate another random line
            engine::Evaluation::Heuristic(_) => {
                println!("Not an endgame -- Continuing");
                continue;
            },
            // If we can win early, we could be at a dead end, retry
            engine::Evaluation::WhiteWinPly(n) => {
                let depth = (n as i32) - (cutoff as i32);
                if depth < min_depth as i32 {
                    println!("Early win found -- Retrying");
                    generate_endgame(min_depth, max_depth);
                    return;
                }
                print!("White {} ", depth);
                break;
            }
            engine::Evaluation::BlackWinPly(n) => {
                let depth = n - (cutoff as u32);
                if depth < min_depth {
                    println!("Early win found -- Retrying");
                    generate_endgame(min_depth, max_depth);
                    return;
                }
                print!("Black {} ", depth);
                break;
            }
        }
    }
    for action in moves.into_iter() {
        print!("{} ", action.to_string());
    }
    println!();
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Selfplay { strategy } => selfplay(strategy),
        Commands::GenerateEndgame {
            min_depth,
            max_depth,
        } => generate_endgame(min_depth, max_depth),
    }
}
