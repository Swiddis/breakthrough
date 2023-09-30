use std::str::FromStr;

/**
 * Primitives for handling moves on an 8x8 board
 */

// Player, start, end
#[derive(Clone, Debug, PartialEq)]
pub struct BreakthroughMove(pub u8, pub u8);

impl BreakthroughMove {
    fn encode_square(square: u8) -> String {
        format!("{}{}", (square % 8 + ('a' as u8)) as char, 8 - square / 8)
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(());
        }
        let bytes: &[u8] = s.as_bytes();
        let start: u8;
        let end: u8;
        let (a, h, one, eight) = ('a' as u8, 'h' as u8, '1' as u8, '8' as u8);
        if a <= bytes[0] && h >= bytes[0] && one <= bytes[1] && eight >= bytes[1] {
            start = (eight - bytes[1]) * 8 + (bytes[0] - a)
        } else {
            return Err(());
        }
        if a <= bytes[2] && h >= bytes[2] && one <= bytes[3] && eight >= bytes[3] {
            end = (eight - bytes[3]) * 8 + (bytes[2] - a)
        } else {
            return Err(());
        }
        Ok(BreakthroughMove(start, end))
    }
}
