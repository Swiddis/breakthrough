pub mod engine;
pub mod game;

use clap::{Parser, ValueEnum};
use engine::Evaluation;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    engine::{classic, fast_win_check, minimax, random},
    game::{
        breakthrough::BreakthroughMove,
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

fn output_setup(winner: Player, to_play: Player, depth: u32, moves: &Vec<BreakthroughMove>) {
    println!(
        "{} {} {} {}",
        if winner == Player::White { "w" } else { "b" },
        if to_play == Player::White { "w" } else { "b" },
        depth,
        moves
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn evaluate_all_moves(
    node: &BreakthroughNode,
    depth: u32,
    rng: &mut ThreadRng,
) -> Vec<(BreakthroughMove, Evaluation)> {
    let mut actions = node.get_possible_actions();
    actions.shuffle(rng);
    let result = actions
        .iter()
        .map(|a| {
            let next = node.take_action(a);
            (a.clone(), fast_win_check::evaluate(&next, depth))
        })
        .collect();
    result
}

fn find_endgames(
    min_depth: u32,
    max_depth: u32,
    node: &BreakthroughNode,
    moves: &Vec<BreakthroughMove>,
    evals: &Vec<(BreakthroughMove, Evaluation)>,
) -> usize {
    let mut count = 0;
    let mut moves = moves.clone();
    for eval in evals.iter() {
        match eval.1 {
            Evaluation::BlackWinPly(n) => if min_depth <= n - node.ply() && n - node.ply() <= max_depth {
                moves.push(eval.0.clone());
                output_setup(Player::Black, node.to_play(), n - node.ply(), &moves);
                moves.pop();               
            },
            Evaluation::WhiteWinPly(n) => if min_depth <= n - node.ply() && n - node.ply() <= max_depth {
                moves.push(eval.0.clone());
                output_setup(Player::White, node.to_play(), n - node.ply(), &moves);
                moves.pop();
            },
            Evaluation::Heuristic(_) => continue,
        }
        count += 1;
    }
    count
}

fn generate_endgames(min_depth: u32, max_depth: u32, n: usize) {
    let mut count = 0;
    let mut rng = rand::thread_rng();

    while count < n {
        let mut node = BreakthroughNode::default();
        let mut history: Vec<BreakthroughNode> = vec![];
        let mut moves: Vec<BreakthroughMove> = Vec::new();

        while !node.is_terminal() {
            let evaluated_actions = evaluate_all_moves(&node, max_depth + 1, &mut rng);
            count += find_endgames(min_depth, max_depth, &node, &moves, &evaluated_actions);
            let next_move = match node.to_play() {
                Player::White => evaluated_actions.iter().max_by_key(|a| a.1).unwrap(),
                Player::Black => evaluated_actions.iter().min_by_key(|a| a.1).unwrap(),
            };

            moves.push(next_move.0.clone());
            history.push(node.clone());
            node = node.take_action(&next_move.0);
            eprintln!();
            eprintln!("{}", node.to_string());
        }
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Selfplay { strategy } => selfplay(strategy),
        Commands::GenerateEndgame {
            min_depth,
            max_depth,
            n,
        } => {
            generate_endgames(min_depth, max_depth, n as usize);
        }
    }
}
