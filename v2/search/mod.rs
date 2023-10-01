use crate::core::{move8::BreakthroughMove, node8::BreakthroughNode, Evaluation};

use self::table::TranspositionTable;

pub mod negamax;
pub mod table;
mod zobrist;

pub fn evaluate_with_ttable(
    node: &BreakthroughNode,
    depth: u32,
    table: &mut TranspositionTable,
) -> (BreakthroughMove, Evaluation) {
    let eval = negamax::negamax(
        node,
        depth,
        Evaluation::BlackWinPly(node.ply),
        Evaluation::WhiteWinPly(node.ply),
        table,
        true,
    );
    match eval.0 {
        Some(e) => (e, eval.1),
        None => (node.get_possible_actions()[0].clone(), eval.1),
    }
}

pub fn evaluate(node: &BreakthroughNode, depth: u32) -> (BreakthroughMove, Evaluation) {
    let eval = negamax::negamax(
        node,
        depth,
        Evaluation::BlackWinPly(node.ply),
        Evaluation::WhiteWinPly(node.ply),
        &mut TranspositionTable::new(0),
        true,
    );
    match eval.0 {
        Some(e) => (e, eval.1),
        None => (node.get_possible_actions()[0].clone(), eval.1),
    }
}
