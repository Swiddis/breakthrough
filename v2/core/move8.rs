use std::{str::FromStr, io::{self, ErrorKind}};

/**
 * Primitives for handling moves on an 8x8 board
 */

// Player, start, end
#[derive(Clone, Debug, PartialEq)]
pub struct BreakthroughMove(pub u8, pub u8);

impl BreakthroughMove {
    fn encode_square(square: u8) -> String {
        format!("{}{}", (square % 8 + b'a') as char, 8 - square / 8)
    }
}

impl ToString for BreakthroughMove {
    fn to_string(&self) -> String {
        format!(
            "{}{}",
            Self::encode_square(self.0),
            Self::encode_square(self.1)
        )
    }
}

impl FromStr for BreakthroughMove {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(io::Error::new(ErrorKind::InvalidInput, "must be length 4"));
        }
        let bytes: &[u8] = s.as_bytes();
        let start: u8 =
            if b'a' <= bytes[0] && b'h' >= bytes[0] && b'1' <= bytes[1] && b'8' >= bytes[1] {
                (b'8' - bytes[1]) * 8 + (bytes[0] - b'a')
            } else {
                return Err(io::Error::new(ErrorKind::InvalidInput, "first square is invalid"));
            };
        let end: u8 =
            if b'a' <= bytes[2] && b'h' >= bytes[2] && b'1' <= bytes[3] && b'8' >= bytes[3] {
                (b'8' - bytes[3]) * 8 + (bytes[2] - b'a')
            } else {
                return Err(io::Error::new(ErrorKind::InvalidInput, "second square is invalid"));
            };
        Ok(BreakthroughMove(start, end))
    }
}
