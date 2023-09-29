use super::{
    node::{GameResult, Node, Player},
    BLACK_FIRST_ROW, BLACK_START, EDGE_LEFT, EDGE_RIGHT, WHITE_FIRST_ROW, WHITE_START,
};

use std::hash::Hash;

/*
Indices: top-to-bottom, left-to-right.
We view it from White's perspective, so white is on the bottom.

 0  1  2  3  4  5  6  7  |  B  B  B  B  B  B  B  B
 8  9 10 11 12 13 14 15  |  B  B  B  B  B  B  B  B
16 17 18 19 20 21 22 23  |  .  .  .  .  .  .  .  .
24 25 26 27 28 29 30 31  |  .  .  .  .  .  .  .  .
32 33 34 35 36 37 38 39  |  .  .  .  .  .  .  .  .
40 41 42 43 44 45 46 47  |  .  .  .  .  .  .  .  .
48 49 50 51 52 53 54 55  |  W  W  W  W  W  W  W  W
56 57 58 59 60 61 62 63  |  W  W  W  W  W  W  W  W
 */

// Player, start, end
#[derive(Clone, Debug)]
pub struct BreakthroughMove(Player, u64, u64);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreakthroughNode {
    bitboard_black: u64,
    bitboard_white: u64,
    to_play: Player,
    ply: u32,
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
                moves.push(BreakthroughMove(Player::White, i + 8, i))
            }
            if diag_right & (1 << i) > 0 {
                moves.push(BreakthroughMove(Player::White, i + 7, i))
            }
            if diag_left & (1 << i) > 0 {
                moves.push(BreakthroughMove(Player::White, i + 9, i))
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
                moves.push(BreakthroughMove(Player::Black, i - 8, i))
            }
            if diag_right & (1 << i) > 0 {
                moves.push(BreakthroughMove(Player::Black, i - 9, i))
            }
            if diag_left & (1 << i) > 0 {
                moves.push(BreakthroughMove(Player::Black, i - 7, i))
            }
        }
        moves
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

impl Node<BreakthroughMove> for BreakthroughNode {
    fn get_possible_actions(&self) -> Vec<BreakthroughMove> {
        match self.to_play {
            Player::White => self.get_moves_white(),
            Player::Black => self.get_moves_black(),
        }
    }

    fn get_result(&self) -> GameResult {
        match (
            self.bitboard_white & BLACK_FIRST_ROW,
            self.bitboard_black & WHITE_FIRST_ROW,
        ) {
            (0, 0) => GameResult::Undecided,
            (_, 0) => GameResult::Win(Player::White),
            (0, _) => GameResult::Win(Player::Black),
            (_, _) => GameResult::Draw,
        }
    }

    fn is_terminal(&self) -> bool {
        ((self.bitboard_white & BLACK_FIRST_ROW) > 0)
            || ((self.bitboard_black & WHITE_FIRST_ROW) > 0)
    }

    fn take_action(&self, action: &BreakthroughMove) -> Self {
        let (start, end) = (1 << action.1, 1 << action.2);
        match action.0 {
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

    fn to_play(&self) -> Player {
        self.to_play.clone()
    }

    fn ply(&self) -> u32 {
        self.ply
    }

    fn bitboards(&self) -> (u64, u64) {
        (self.bitboard_white, self.bitboard_black)
    }
}
