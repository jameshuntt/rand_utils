// crates/shamirs_secret_sharing/src/rand_utils/gen_secret_biguint_range.rs

use num_bigint::BigUint;

use rand::{
    CryptoRng,
    RngCore
};

use secrecy::{
    ExposeSecret,
    SecretBox
};

use crate::gen_secret_biguint_below::gen_secret_biguint_below;
use honest::types::secure_types::{
    SecretBigUint,
    SecureBigUint
};

pub fn gen_secret_biguint_range(
    rng: &mut (impl RngCore + CryptoRng),
    low: &BigUint,
    high: &BigUint
) -> SecretBigUint {
    assert!(low < high, "low must be less than high");
    let range = high - low;
    let r = gen_secret_biguint_below(
        rng,
        &range
    );
    let unwrapped = &r.expose_secret().0 + low;

    SecretBox::new(
        Box::new(
            SecureBigUint(unwrapped)
        )
    )
}
