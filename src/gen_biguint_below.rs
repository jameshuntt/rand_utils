// crates/shamirs_secret_sharing/src/rand_utils/gen_biguint_below.rs

use num_bigint::BigUint;

use rand::{
    CryptoRng,
    RngCore
};

pub fn gen_biguint_below(
    rng: &mut impl RngCore,
    upper: &BigUint
) -> BigUint {
    let bits = upper.bits();

    loop {
        let mut bytes = vec![
            0u8;
            ((bits + 7) / 8).try_into().unwrap()
        ];
        rng.fill_bytes(&mut bytes);
        let candidate = BigUint::from_bytes_be(&bytes);
        if &candidate < upper {
            return candidate;
        }
    }
}

/// Generate a uniformly random `BigUint` strictly less than `upper`
pub fn gen_biguint_below_2(
    rng: &mut impl RngCore,
    upper: &BigUint
) -> BigUint {
    let bits = upper.bits();
    let byte_len = ((bits + 7) / 8) as usize;

    loop {
        let mut bytes = vec![0u8; byte_len];
        rng.fill_bytes(&mut bytes);
        let candidate = BigUint::from_bytes_be(&bytes);
        if &candidate < upper {
            return candidate;
        }
    }
}

pub fn gen_biguint_below_3(
    rng: &mut (impl RngCore + CryptoRng),
    upper: &BigUint
) -> BigUint {
    let bits = upper.bits();
    let byte_len = ((bits + 7) / 8) as usize;

    loop {
        let mut bytes = vec![0u8; byte_len];
        rng.fill_bytes(&mut bytes);
        let candidate = BigUint::from_bytes_be(&bytes);
        if &candidate < upper {
            return candidate;
        }
    }
}
