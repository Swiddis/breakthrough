use crate::core::{move8::BreakthroughMove, node8::BreakthroughNode, Evaluation};

use self::table::TranspositionTable;

pub mod negamax;
pub mod table;
mod zobrist;

pub fn evaluate_with_ttable<'a>(
    node: &BreakthroughNode,
    depth: u32,
    table: &mut TranspositionTable,
) -> (Option<BreakthroughMove>, Evaluation) {
    negamax::negamax(
        node,
        depth,
        Evaluation::BlackWinPly(node.ply),
        Evaluation::WhiteWinPly(node.ply),
        table,
    )
}

pub fn evaluate<'a>(node: &BreakthroughNode, depth: u32) -> (Option<BreakthroughMove>, Evaluation) {
    negamax::negamax(
        node,
        depth,
        Evaluation::BlackWinPly(node.ply),
        Evaluation::WhiteWinPly(node.ply),
        &mut TranspositionTable::new(0),
    )
}
