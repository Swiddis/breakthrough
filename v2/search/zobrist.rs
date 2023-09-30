use std::sync::OnceLock;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;

static ZOBRIST_KEYS: OnceLock<[u64; 192]> = OnceLock::new();

pub fn zobrist_keys() -> &'static [u64; 192] {
    ZOBRIST_KEYS.get_or_init(|| {
        let mut rng = ChaCha12Rng::seed_from_u64(3141592653589793238);
        let mut result = [0u64; 192];
        (0..192).for_each(|i| result[i] = rng.gen());
        result
    })
}
