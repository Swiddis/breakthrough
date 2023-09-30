use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    str::FromStr,
};

use v2::core::{move8::BreakthroughMove, node8::BreakthroughNode, Evaluation};

use anyhow::anyhow;

pub fn read_positions(
    filename: &str,
) -> Result<Vec<(BreakthroughNode, Evaluation, u32)>, anyhow::Error> {
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
        let depth = depth.parse::<u32>()?;
        let eval = match winner {
            "w" => Evaluation::WhiteWinPly(moves.len() as u32 + depth - 1),
            "b" => Evaluation::BlackWinPly(moves.len() as u32 + depth - 1),
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

        nodes.push((node, eval, depth));
    }

    Ok(nodes)
}
