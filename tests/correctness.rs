use v2::{evaluation, core::Evaluation};

mod common;

#[test]
fn mate_in_n_heuristic_correctness() {
    let dataset = common::read_positions("tests/data/mate-in-n.txt").unwrap();
    let (mut correct, mut incorrect) = (0, 0);

    for (node, expect_eval) in dataset.iter() {
        let actual_eval = evaluation::fast_heuristic(node);
        match expect_eval {
            Evaluation::BlackWinPly(_) => if actual_eval < Evaluation::Heuristic(0) {
                correct += 1;
            } else {
                incorrect += 1;
            },
            Evaluation::WhiteWinPly(_) => if actual_eval > Evaluation::Heuristic(0) {
                correct += 1;
            } else {
                incorrect += 1;
            },
            Evaluation::Heuristic(_) => {},
        }
    }

    eprintln!("(correct={}, incorrect={})", correct, incorrect);
    assert!(correct > incorrect);
}
