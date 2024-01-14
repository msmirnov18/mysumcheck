// Importing finite fields and polynomials from arkworks
use ark_ff::fields::{Fp64, MontBackend, MontConfig};
use ark_ff::BigInt;
use ark_ff::Zero;
use ark_poly::polynomial::univariate;
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial, SparseTerm, Term},
    DenseMVPolynomial, Polynomial,
};
use std::ops::Add;
use std::ops::Deref;
use std::ops::Mul;

use ark_ff::UniformRand;
use rand::prelude::*;
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};

// Instantiating the finite field F_{101}
#[derive(MontConfig)]
#[modulus = "101"]
#[generator = "2"]
pub struct FieldConfig;
pub type FF = Fp64<MontBackend<FieldConfig, 1>>;

use crate::protocol::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_raise_to_power() {
        assert_eq!(raise_to_power(FF::from(2), 5), FF::from(32));
    }

    #[test]
    fn test_univariate_hypercube_evaluate() {
        let testpoly = univariate::SparsePolynomial::from_coefficients_vec(vec![
            (0, FF::from(5)),
            (2, FF::from(1)),
        ]);
        assert_eq!(univariate_hypercube_evaluate(testpoly), FF::from(11));
    }
}
