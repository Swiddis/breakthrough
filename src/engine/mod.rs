pub mod minimax;
pub mod random;
pub mod classic;
pub mod fast_win_check;

use std::{cmp::Ordering, ops::Not};

#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
pub enum Evaluation {
    BlackWinPly(u32),
    WhiteWinPly(u32),
    Heuristic(i64),
}

impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Evaluation::BlackWinPly(n) => match other {
                Evaluation::BlackWinPly(k) => Some(n.cmp(k)),
                Evaluation::WhiteWinPly(_) => Some(Ordering::Less),
                Evaluation::Heuristic(_) => Some(Ordering::Less),
            },
            Evaluation::WhiteWinPly(n) => match other {
                Evaluation::BlackWinPly(_) => Some(Ordering::Greater),
                Evaluation::WhiteWinPly(k) => Some(k.cmp(n)),
                Evaluation::Heuristic(_) => Some(Ordering::Greater),
            },
            Evaluation::Heuristic(n) => match other {
                Evaluation::BlackWinPly(_) => Some(Ordering::Greater),
                Evaluation::WhiteWinPly(_) => Some(Ordering::Less),
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
            Self::BlackWinPly(n) => Self::WhiteWinPly(n),
            Self::WhiteWinPly(n) => Self::BlackWinPly(n),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::engine::Evaluation::*;

    #[test]
    fn test_evaluation_ord() {
        assert!(Heuristic(-1) < Heuristic(1));
        assert!(BlackWinPly(1) < BlackWinPly(2));
        assert!(WhiteWinPly(2) < WhiteWinPly(1));
        assert!(BlackWinPly(1) < WhiteWinPly(1));
        assert!(Heuristic(0) < WhiteWinPly(1));
        assert!(BlackWinPly(1) < Heuristic(0));
    }

    #[test]
    fn test_negation() {
        assert_eq!(!Heuristic(10), Heuristic(-10));
        assert_eq!(!WhiteWinPly(2), BlackWinPly(2));
        assert_eq!(!BlackWinPly(5), WhiteWinPly(5));
    }
}
