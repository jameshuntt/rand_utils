// crates/shamirs_secret_sharing/src/rand_utils/gen_secret_biguint_below.rs

use honest::types::secure_types::{
    SecureBigUint,
    SecretBigUint
};
use num_bigint::BigUint;
use rand::{CryptoRng, RngCore};
use secrecy::SecretBox;

pub fn gen_secret_biguint_below(
    rng: &mut (impl RngCore + CryptoRng),
    upper: &BigUint
) -> SecretBigUint {
    let bits = upper.bits();
    let byte_len = ((bits + 7) / 8) as usize;

    loop {
        let mut bytes = vec![0u8; byte_len];
        rng.fill_bytes(&mut bytes);
        let candidate = BigUint::from_bytes_be(&bytes);
        if &candidate < upper {
            return SecretBox::new(
                Box::new(
                    SecureBigUint(candidate)
                )
            );
        }
    }
}