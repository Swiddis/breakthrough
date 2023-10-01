use std::{io, str::FromStr};

use clap::{Parser, ValueEnum};
use v2::{core::{node8::BreakthroughNode, move8::BreakthroughMove, Evaluation, Player}, search::{evaluate_with_ttable, table::TranspositionTable}};

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
    Play {
        #[arg(long, default_value = "random")]
        strategy: PlayStrategy,
        #[arg(long, default_value = "8")]
        depth: u32,
    }
}

#[derive(Clone, Debug, ValueEnum)]
enum PlayStrategy {
    Random,
    V2,
}

fn evaluate_iterative_deepening(node: &BreakthroughNode, depth: u32, table: &mut TranspositionTable) -> (BreakthroughMove, Evaluation) {
    for d in 1..depth {
        let eval = evaluate_with_ttable(node, d, table);
        println!("{} {} {:?}", d, eval.0.to_string(), eval.1);
    }
    match node.to_play {
        Player::White => evaluate_with_ttable(node, depth, table),
        Player::Black => {
            let eval = evaluate_with_ttable(node, depth, table);
            (eval.0, -eval.1)
        },
    }
}

fn do_selfplay(strategy: PlayStrategy, depth: u32) {
    match strategy {
        PlayStrategy::Random => todo!(),
        PlayStrategy::V2 => {
            let mut node = BreakthroughNode::default();
            let mut table = TranspositionTable::new(2usize.pow(22));

            println!("{}\n{}", node.fen(), node.to_string());

            while !node.is_terminal() {
                let (action, eval) = evaluate_iterative_deepening(&node, depth, &mut table);
                println!("({}, {:?})", action.to_string(), eval);
                node = node.take_action(&action);
                println!("{}\n{}", node.fen(), node.to_string());
            }
        },
    }
}

fn get_user_action(node: &BreakthroughNode) -> Result<BreakthroughMove, io::Error> {
    let valid_moves = node.get_possible_actions();
    let stdin = io::stdin();
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        match BreakthroughMove::from_str(&buffer.trim()) {
            Ok(m) => if valid_moves.contains(&m) {
                return Ok(m);
            } else {
                println!("Invalid move!");
                continue
            },
            Err(e) => {
                println!("{}", e.to_string());
                continue
            },
        }
    }
}

fn do_play(strategy: PlayStrategy, depth: u32) {
    match strategy {
        PlayStrategy::Random => todo!(),
        PlayStrategy::V2 => {
            let mut node = BreakthroughNode::default();
            let mut table = TranspositionTable::new(2usize.pow(22));

            println!("{}\n{}", node.fen(), node.to_string());

            while !node.is_terminal() {
                let action = get_user_action(&node).expect("Valid input");
                node = node.take_action(&action);
                println!("{}\n{}", node.fen(), node.to_string());

                let (action, eval) = evaluate_iterative_deepening(&node, depth, &mut table);
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
        },
        Commands::Play { strategy, depth } => {
            do_play(strategy, depth);
        }
    }
}
