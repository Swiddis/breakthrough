pub mod random;
pub mod minimax;

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Ord)]
pub enum Evaluation {
    BlackMate(usize),
    WhiteMate(usize),
    Heuristic(i64),
}

impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Evaluation::BlackMate(n) => match other {
                Evaluation::BlackMate(k) => Some(n.cmp(k)),
                Evaluation::WhiteMate(_) => Some(Ordering::Less),
                Evaluation::Heuristic(_) => Some(Ordering::Less),
            },
            Evaluation::WhiteMate(n) => match other {
                Evaluation::BlackMate(_) => Some(Ordering::Greater),
                Evaluation::WhiteMate(k) => Some(k.cmp(n)),
                Evaluation::Heuristic(_) => Some(Ordering::Greater),
            },
            Evaluation::Heuristic(n) => match other {
                Evaluation::BlackMate(_) => Some(Ordering::Greater),
                Evaluation::WhiteMate(_) => Some(Ordering::Less),
                Evaluation::Heuristic(k) => Some(n.cmp(k)),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::engine::Evaluation::*;

    #[test]
    fn test_evaluation_ord() {
        assert!(Heuristic(-1) < Heuristic(1));
        assert!(BlackMate(1) < BlackMate(2));
        assert!(WhiteMate(2) < WhiteMate(1));
        assert!(BlackMate(1) < WhiteMate(1));
        assert!(Heuristic(0) < WhiteMate(1));
        assert!(BlackMate(1) < Heuristic(0));
    }
}
