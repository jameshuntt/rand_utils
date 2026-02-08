#![allow(unused)]

// crates/shamirs_secret_sharing/src/rand_utils/implementation.rs

use honest::types::secure_types::SecureBigUint;
use num_bigint::BigUint;
use rand::{rngs::OsRng, TryRngCore};
use secrecy::SecretBox;

use crate::traits::{
    BigUintGenerator,
    SecureBigUintGenerator
};

impl BigUintGenerator for OsRng {
    fn gen_biguint_below(&mut self, upper: &BigUint) -> BigUint {
        let bits = upper.bits();
        let byte_len = ((bits + 7) / 8) as usize;

        loop {
            let mut bytes = vec![0u8; byte_len];
            self.try_fill_bytes(&mut bytes);
            let candidate = BigUint::from_bytes_be(&bytes);
            if &candidate < upper {
                return candidate;
            }
        }
    }

    fn gen_biguint_range(&mut self, low: &BigUint, high: &BigUint) -> BigUint {
        assert!(low < high);
        let range = high - low;
        self.gen_biguint_below(&range) + low
    }
}

impl SecureBigUintGenerator for OsRng {
    fn gen_secret_biguint_below(&mut self, upper: &BigUint) -> SecretBox<SecureBigUint> {
        let value = self.gen_biguint_below(upper);
        SecretBox::new(Box::new(SecureBigUint(value)))
    }

    fn gen_secret_biguint_range(&mut self, low: &BigUint, high: &BigUint) -> SecretBox<SecureBigUint> {
        let value = self.gen_biguint_range(low, high);
        SecretBox::new(Box::new(SecureBigUint(value)))
    }
}
