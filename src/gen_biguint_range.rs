// crates/shamirs_secret_sharing/src/rand_utils/gen_biguint_range.rs

use num_bigint::BigUint;
use rand::{CryptoRng, RngCore};

use crate::gen_biguint_below::gen_biguint_below;

/// Generates a uniformly random `BigUint` in the range [low, high).
pub fn gen_biguint_range(
    rng: &mut (impl RngCore + CryptoRng),
    low: &BigUint,
    high: &BigUint
) -> BigUint {
    assert!(low < high, "low must be less than high");
    let range = high - low;
    let rand = gen_biguint_below(rng, &range);

    rand + low
}
