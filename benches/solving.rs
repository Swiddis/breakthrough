use std::{
    fs::File,
    io::{self, BufRead},
    path::Path, str::FromStr,
};

use anyhow::anyhow;
use breakthrough::{engine::{Evaluation, classic}, game::{breakthrough::{BreakthroughNode, BreakthroughMove}, node::Node}};
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn read_positions(filename: &str) -> Result<Vec<(BreakthroughNode, Evaluation)>, anyhow::Error> {
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
                },
                Err(_) => {
                    return Err(anyhow!("Invalid move"));
                }
            }
        }
        
        nodes.push((node, eval));
    }

    Ok(nodes)
}

fn solve_mate_in_n(c: &mut Criterion) {
    let data = read_positions("tests/mate-in-n.txt").unwrap();

    let mut group = c.benchmark_group("solve_mate_in_n");
    for pos in 0..data.len() {
        group.bench_with_input(BenchmarkId::from_parameter(pos), &pos, |b, &pos| {
            b.iter(|| {
                let (pos, _eval) = data[pos].clone();
                black_box(classic::get_move(&pos, 6));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, solve_mate_in_n);
criterion_main!(benches);
