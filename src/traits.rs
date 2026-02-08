// crates/shamirs_secret_sharing/src/rand_utils/traits.rs

use honest::types::secure_types::{
    SecretBigUint,
    SecureBigInt,
    SecureBigUint
};
use num_bigint::{BigInt, BigUint};
use rand::{CryptoRng, RngCore};
use secrecy::SecretBox;

pub trait BigUintGenerator {
    fn gen_biguint_below(&mut self, upper: &BigUint) -> BigUint;
    fn gen_biguint_range(&mut self, low: &BigUint, high: &BigUint) -> BigUint;
}

pub trait SecureBigUintGenerator {
    fn gen_secret_biguint_below(&mut self, upper: &BigUint) -> SecretBox<SecureBigUint>;
    fn gen_secret_biguint_range(&mut self, low: &BigUint, high: &BigUint) -> SecretBox<SecureBigUint>;
}

pub trait SecureBigUintGenerator2 {
    fn gen_secret_biguint_below(&mut self, upper: &BigUint) -> SecretBigUint;
    fn gen_secret_biguint_range(&mut self, low: &BigUint, high: &BigUint) -> SecretBigUint;
}

pub trait SecureBigIntGenerator {
    fn gen_secret_bigint_range(&mut self, low: &BigInt, high: &BigInt) -> SecretBox<SecureBigInt>;
}

pub trait RandomProvider: RngCore + CryptoRng + Send + Sync + 'static {}
impl<T: RngCore + CryptoRng + Send + Sync + 'static> RandomProvider for T {}


pub trait FieldRng {
    fn gen_field_element(&mut self, prime: &BigUint) -> BigUint;
}
