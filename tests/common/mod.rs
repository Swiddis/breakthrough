use std::{path::Path, fs::File, str::FromStr, io::{self, BufRead}};

use v2::core::{node8::BreakthroughNode, Evaluation, move8::BreakthroughMove};

use anyhow::anyhow;

pub fn read_positions(filename: &str) -> Result<Vec<(BreakthroughNode, Evaluation)>, anyhow::Error> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    let mut nodes = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let mut parts = line.split_ascii_whitespace();
        let (winner, _to_play, depth, moves) = (
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.collect::<Vec<&str>>(),
        );
        let eval = match winner {
            "w" => Evaluation::WhiteWinPly(moves.len() as u32 + depth.parse::<u32>()?),
            "b" => Evaluation::BlackWinPly(moves.len() as u32 + depth.parse::<u32>()?),
            _ => return Err(anyhow!("Invalid eval winner")),
        };
        let mut node = BreakthroughNode::default();
        for step in moves {
            let parsed = BreakthroughMove::from_str(step);
            match parsed {
                Ok(action) => {
                    node = node.take_action(&action);
                }
                Err(_) => {
                    return Err(anyhow!("Invalid move"));
                }
            }
        }

        nodes.push((node, eval));
    }

    Ok(nodes)
}
