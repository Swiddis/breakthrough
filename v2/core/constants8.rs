/**
 * Bitboard constants for 8x8
 */

/*
Places: top-to-bottom, left-to-right.
We view it from White's perspective, so white is on the bottom.

i.e. square = 1 << n with n described by:

 0  1  2  3  4  5  6  7  |  B  B  B  B  B  B  B  B
 8  9 10 11 12 13 14 15  |  B  B  B  B  B  B  B  B
16 17 18 19 20 21 22 23  |  .  .  .  .  .  .  .  .
24 25 26 27 28 29 30 31  |  .  .  .  .  .  .  .  .
32 33 34 35 36 37 38 39  |  .  .  .  .  .  .  .  .
40 41 42 43 44 45 46 47  |  .  .  .  .  .  .  .  .
48 49 50 51 52 53 54 55  |  W  W  W  W  W  W  W  W
56 57 58 59 60 61 62 63  |  W  W  W  W  W  W  W  W
 */

pub const WHITE_FIRST_ROW: u64 = 0xff << 56;
pub const BLACK_FIRST_ROW: u64 = 0xff;
pub const WHITE_START: u64 = 0xffff << 48;
pub const BLACK_START: u64 = 0xffff;
pub const EDGE_RIGHT: u64 = 0x8080808080808080;
pub const EDGE_LEFT: u64 = 0x0101010101010101;
