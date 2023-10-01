use crate::core::{node8::BreakthroughNode, Evaluation, constants8::{WHITE_START, BLACK_START}};

const CENTER: u64 = 0x00003c3c3c3c0000;
const WHITE_SIDE: u64 = WHITE_START | (WHITE_START >> 16);
const BLACK_SIDE: u64 = BLACK_START | (BLACK_START >> 16);

// Fast heuristic must be iteration-free and should be minimally branched.
// i.e. limits itself to whole-board bitwise operations.
pub fn fast_heuristic(node: &BreakthroughNode) -> Evaluation {
    let (wpop, bpop) = (
        node.bitboard_white.count_ones() as i64 * 1000,
        node.bitboard_black.count_ones() as i64 * 1000,
    );
    let (w_center, b_center) = (
        (node.bitboard_white & CENTER).count_ones() as i64 * 400,
        (node.bitboard_black & CENTER).count_ones() as i64 * 400,
    );
    let (w_attack, b_attack) = (
        (node.bitboard_white & BLACK_SIDE).count_ones() as i64 * 750,
        (node.bitboard_black & WHITE_SIDE).count_ones() as i64 * 750,
    );
    Evaluation::Heuristic((wpop + w_center + w_attack) - (bpop + b_center + b_attack))
}
