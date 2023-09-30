use crate::core::node8::BreakthroughNode;

pub fn popcount(node: &BreakthroughNode) -> (u32, u32) {
    (
        node.bitboard_white.count_ones(),
        node.bitboard_black.count_ones(),
    )
}
