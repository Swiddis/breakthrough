use v2::{evaluation, core::{Evaluation, Player}, search};

mod common;

#[test]
fn mate_in_n_heuristic_correctness() {
    let dataset = common::read_positions("tests/data/mate-in-n.txt").unwrap();
    let (mut correct, mut incorrect) = (0, 0);

    for (node, expect_eval, _) in dataset.iter() {
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

#[test]
fn mate_in_n_negamax_correctness() {
    let dataset = common::read_positions("tests/data/mate-in-n.txt").unwrap();
    
    for (node, expect_eval, depth) in dataset.iter().filter(|x| x.2 <= 4).take(50) {
        let actual_eval = match node.to_play {
            Player::White => search::negamax(node, *depth),
            Player::Black => !search::negamax(node, *depth),
        };
        if expect_eval != &actual_eval {
            eprintln!("{:?}\n{}\n{}", node, node.fen(), node.to_string());
        }
        assert_eq!(expect_eval, &actual_eval);
    }
}
