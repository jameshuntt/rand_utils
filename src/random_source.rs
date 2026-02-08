// crates/shamirs_secret_sharing/src/rand_utils/random_source.rs

use rand::{RngCore, CryptoRng};
use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use secrecy::SecretBox;

use crate::{
    traits::{
        BigUintGenerator,
        SecureBigIntGenerator,
        SecureBigUintGenerator,
        SecureBigUintGenerator2
    },
};
use honest::types::secure_types::{
    SecretBigUint,
    SecureBigInt,
    SecureBigUint
};

// === RandomSource Wrapper ===

pub struct RandomSource<R: RngCore>(pub R);

impl<R: RngCore + CryptoRng> BigUintGenerator for RandomSource<R> {
    fn gen_biguint_below(&mut self, upper: &BigUint) -> BigUint {
        let bits = upper.bits();
        let byte_len = ((bits + 7) / 8) as usize;
        loop {
            let mut bytes = vec![0u8; byte_len];
            self.0.fill_bytes(&mut bytes);
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

impl<R: RngCore + CryptoRng> SecureBigUintGenerator for RandomSource<R> {
    fn gen_secret_biguint_below(&mut self, upper: &BigUint) -> SecretBox<SecureBigUint> {
        let result = self.gen_biguint_below(upper);
        SecretBox::new(Box::new(SecureBigUint(result)))
    }

    fn gen_secret_biguint_range(&mut self, low: &BigUint, high: &BigUint) -> SecretBox<SecureBigUint> {
        let result = self.gen_biguint_range(low, high);
        SecretBox::new(Box::new(SecureBigUint(result)))
    }
}

impl<R: RngCore + CryptoRng> SecureBigUintGenerator2 for RandomSource<R> {
    fn gen_secret_biguint_below(&mut self, upper: &BigUint) -> SecretBigUint {
        SecretBox::new(Box::new(SecureBigUint(self.gen_biguint_below(upper))))
    }

    fn gen_secret_biguint_range(&mut self, low: &BigUint, high: &BigUint) -> SecretBigUint {
        SecretBox::new(Box::new(SecureBigUint(self.gen_biguint_range(low, high))))
    }
}

impl<R: RngCore + CryptoRng> SecureBigIntGenerator for RandomSource<R> {
    fn gen_secret_bigint_range(&mut self, low: &BigInt, high: &BigInt) -> SecretBox<SecureBigInt> {
        assert!(low < high);
        let range = high - low;
        let bits = range.bits();
        let byte_len = ((bits + 7) / 8) as usize;
        loop {
            let mut bytes = vec![0u8; byte_len];
            self.0.fill_bytes(&mut bytes);
            let candidate = BigInt::from_signed_bytes_be(&bytes);
            if &candidate >= &BigInt::zero() && &candidate < &range {
                return SecretBox::new(Box::new(SecureBigInt(candidate + low)));
            }
        }
    }
}
