use crate::core::{node8::BreakthroughNode, Evaluation};

pub mod features;

// Fast heuristic must be iteration-free and should be minimally branched.
// i.e. limits itself to whole-board bitwise operations.
pub fn fast_heuristic(node: &BreakthroughNode) -> Evaluation {
    let (wpop, bpop) = features::popcount(node);
    Evaluation::Heuristic(wpop as i64 - bpop as i64)
}
