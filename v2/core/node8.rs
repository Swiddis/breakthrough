use super::{constants8::*, move8::BreakthroughMove, GameResult, Player};
/**
 * 8x8-specific bitboards
 */
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreakthroughNode {
    pub bitboard_black: u64,
    pub bitboard_white: u64,
    pub to_play: Player,
    pub ply: u32,
}

impl BreakthroughNode {
    fn get_moves_white(&self) -> Vec<BreakthroughMove> {
        let nonempty_squares = self.bitboard_black | self.bitboard_white;
        // Straight-line: shift by 8 and filter to only empty destinations
        let straight_line = (self.bitboard_white >> 8) & (!nonempty_squares);
        // Diagonal right: filter out right column, shift by 7, filter friendly
        let diag_right = ((self.bitboard_white & !EDGE_RIGHT) >> 7) & !self.bitboard_white;
        // Diagonal left: filter out left column, shift by 9, filter friendly
        let diag_left = ((self.bitboard_white & !EDGE_LEFT) >> 9) & !self.bitboard_white;
        // Collect moves by destination
        let mut moves = Vec::with_capacity(32);
        for i in 0..64 {
            if straight_line & (1 << i) > 0 {
                moves.push(BreakthroughMove(i + 8, i))
            }
            if diag_right & (1 << i) > 0 {
                moves.push(BreakthroughMove(i + 7, i))
            }
            if diag_left & (1 << i) > 0 {
                moves.push(BreakthroughMove(i + 9, i))
            }
        }
        moves
    }

    fn get_moves_black(&self) -> Vec<BreakthroughMove> {
        let nonempty_squares = self.bitboard_black | self.bitboard_white;
        // Straight-line: shift by 8 and filter to only empty destinations
        let straight_line = (self.bitboard_black << 8) & (!nonempty_squares);
        // Diagonal right: filter the right column, shift by 9, filter friendly
        let diag_right = ((self.bitboard_black & !EDGE_RIGHT) << 9) & !self.bitboard_black;
        // Diagonal left: filter out left column, shift by 7, filter friendly
        let diag_left = ((self.bitboard_black & !EDGE_LEFT) << 7) & !self.bitboard_black;
        // Collect moves by destination
        let mut moves = Vec::with_capacity(32);
        for i in 0..64 {
            if straight_line & (1 << i) > 0 {
                moves.push(BreakthroughMove(i - 8, i))
            }
            if diag_right & (1 << i) > 0 {
                moves.push(BreakthroughMove(i - 9, i))
            }
            if diag_left & (1 << i) > 0 {
                moves.push(BreakthroughMove(i - 7, i))
            }
        }
        moves
    }

    pub fn get_possible_actions(&self) -> Vec<BreakthroughMove> {
        match self.to_play {
            Player::White => self.get_moves_white(),
            Player::Black => self.get_moves_black(),
        }
    }

    pub fn get_result(&self) -> GameResult {
        match (
            self.bitboard_white & BLACK_FIRST_ROW,
            self.bitboard_black & WHITE_FIRST_ROW,
        ) {
            (0, 0) => GameResult::Undecided,
            (_, 0) => GameResult::Win(Player::White),
            (0, _) => GameResult::Win(Player::Black),
            (_, _) => GameResult::Undecided,
        }
    }

    pub fn is_terminal(&self) -> bool {
        ((self.bitboard_white & BLACK_FIRST_ROW) > 0)
            || ((self.bitboard_black & WHITE_FIRST_ROW) > 0)
    }

    pub fn take_action(&self, action: &BreakthroughMove) -> Self {
        let (start, end) = (1 << action.0, 1 << action.1);
        match self.to_play {
            Player::Black => Self {
                bitboard_black: (self.bitboard_black & !start) | end,
                bitboard_white: self.bitboard_white & !end,
                to_play: Player::White,
                ply: self.ply + 1,
            },
            Player::White => Self {
                bitboard_black: self.bitboard_black & !end,
                bitboard_white: (self.bitboard_white & !start) | end,
                to_play: Player::Black,
                ply: self.ply + 1,
            },
        }
    }
}

impl Default for BreakthroughNode {
    fn default() -> Self {
        Self {
            bitboard_black: BLACK_START,
            bitboard_white: WHITE_START,
            to_play: Player::White,
            ply: 0,
        }
    }
}

impl Hash for BreakthroughNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bitboard_black.hash(state);
        self.bitboard_white.hash(state);
        self.to_play.hash(state);
    }
}

impl ToString for BreakthroughNode {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in 0..8 {
            for col in 0..8 {
                let idx = row * 8 + col;
                if self.bitboard_black & (1 << idx) > 0 {
                    result.push('B');
                } else if self.bitboard_white & (1 << idx) > 0 {
                    result.push('W');
                } else {
                    result.push('.');
                }
                if col < 7 {
                    result.push(' ');
                }
            }
            if row < 7 {
                result.push('\n');
            }
        }
        result
    }
}
