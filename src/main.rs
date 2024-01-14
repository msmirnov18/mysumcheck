#![allow(dead_code, unused_imports, unused_variables)]

mod protocol;
mod tests;

use protocol::degree_in_one_variable;
use protocol::hypercube;
use protocol::run_sumcheck_protocol;

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

fn main() {
    // // Polynomial from Thaler's book
    // let input_polynomial = SparsePolynomial::from_coefficients_vec(
    //     3,
    //     vec![
    //         (FF::from(2), SparseTerm::new(vec![(0, 3)])),
    //         (FF::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
    //         (FF::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
    //     ],
    // );

    // Polynomial in 4 variables
    let input_polynomial = SparsePolynomial::from_coefficients_vec(
        4,
        vec![
            (FF::from(2), SparseTerm::new(vec![(0, 4)])),
            (FF::from(3), SparseTerm::new(vec![(1, 4)])),
            (FF::from(5), SparseTerm::new(vec![(2, 4)])),
            (FF::from(7), SparseTerm::new(vec![(3, 4)])),
            (FF::from(17), SparseTerm::new(vec![(1, 1)])),
            (
                FF::from(11),
                SparseTerm::new(vec![(0, 1), (1, 1), (2, 1), (3, 1)]),
            ),
        ],
    );

    // // One variable polynomial
    // let input_polynomial = SparsePolynomial::from_coefficients_vec(
    //     1,
    //     vec![
    //         (FF::from(2), SparseTerm::new(vec![(0, 3)])),
    //         (FF::from(1), SparseTerm::new(vec![(0, 1)])),
    //         (FF::from(1), SparseTerm::new(vec![(0, 5)])),
    //     ],
    // );

    run_sumcheck_protocol(input_polynomial.clone());

    println!("");
    println!("{:?}", degree_in_one_variable(input_polynomial.clone(), 2));
}
