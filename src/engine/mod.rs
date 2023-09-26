pub mod minimax;
pub mod random;
pub mod classic;

use std::{cmp::Ordering, ops::Not};

#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
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

impl Not for Evaluation {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Heuristic(n) => Self::Heuristic(-n),
            Self::BlackMate(n) => Self::WhiteMate(n),
            Self::WhiteMate(n) => Self::BlackMate(n),
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

    #[test]
    fn test_negation() {
        assert_eq!(!Heuristic(10), Heuristic(-10));
        assert_eq!(!WhiteMate(2), BlackMate(2));
        assert_eq!(!BlackMate(5), WhiteMate(5));
    }
}
