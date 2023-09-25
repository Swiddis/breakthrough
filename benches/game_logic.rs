use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use breakthrough::game::{breakthrough::BreakthroughNode, node::Node};

fn generate_playout(rng: &mut ChaCha8Rng) -> Vec<BreakthroughNode> {
    let mut board = BreakthroughNode::default();
    let mut result = vec![board.clone()];
    while !board.is_terminal() {
        let actions = board.get_possible_actions();
        let action = &actions[rng.gen_range(0..actions.len())];
        board = board.take_action(action);
        result.push(board.clone());
    }
    result
}

fn sample_node_dataset() -> Vec<BreakthroughNode> {
    let mut rng = ChaCha8Rng::seed_from_u64(1);
    let mut dataset: Vec<BreakthroughNode> = (0..1000)
        .map(|_| generate_playout(&mut rng))
        .flatten()
        .filter(|node| !node.is_terminal())
        .collect();
    dataset.shuffle(&mut rng);
    dataset
}

fn playout_benchmark(c: &mut Criterion) {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    c.bench_function("generate_playout", |b| {
        b.iter(|| generate_playout(&mut rng))
    });
}

fn move_generation_benchmark(c: &mut Criterion) {
    let nodes: Vec<BreakthroughNode> = sample_node_dataset();
    let mut node_iter = nodes.iter().cycle();
    c.bench_function("generate_moves", |b| {
        b.iter(|| {
            let node = node_iter.next().unwrap();
            black_box(node.get_possible_actions());
        })
    });
}

criterion_group!(benches, playout_benchmark, move_generation_benchmark);
criterion_main!(benches);
