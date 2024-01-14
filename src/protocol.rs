#![allow(dead_code, unused_imports, unused_variables)]

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

use ark_ff::Field;

use ark_ff::UniformRand;
use rand::prelude::*;
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};

// Creating a struct for the prover
struct Prover<FF: Field> {
    polynomial: SparsePolynomial<FF, SparseTerm>,
}

// Creating a struct for the verifier
struct Verifier<FF: Field> {
    polynomial: SparsePolynomial<FF, SparseTerm>,
}

impl<FF: Field + std::convert::From<i32>> Prover<FF> {
    fn message(&self, random_challenges: &Vec<FF>) -> univariate::SparsePolynomial<FF> {
        let mut output = univariate::SparsePolynomial::zero();

        if random_challenges.len() == self.polynomial.num_vars - 1 {
            output = output.add(polynomial_partial_evaluate(
                self.polynomial.clone(),
                random_challenges.len(),
                random_challenges.clone(),
                vec![],
            ));
            output
        } else {
            // TODO: use usize instead of i32 in hypercube to get rid of try_into().unwrap()
            for item in hypercube(
                (self.polynomial.num_vars - random_challenges.len() - 1)
                    .try_into()
                    .unwrap(),
            ) {
                output = output.add(polynomial_partial_evaluate(
                    self.polynomial.clone(),
                    random_challenges.len(),
                    random_challenges.clone(),
                    item,
                ));
            }
            output
        }
    }
}

impl<FF: Field + std::convert::From<i32>> Verifier<FF> {
    fn message(&self) -> FF {
        let rng = &mut ChaCha20Rng::from_entropy();
        FF::rand(rng)
    }
}

// Defining the boolean hypercube.
pub fn hypercube<FF: Field + std::convert::From<i32>>(size: i32) -> Vec<Vec<FF>> {
    assert!(size >= 0, "Negative input in hypercube");
    if size == 0 {
        vec![]
    } else if size == 1 {
        vec![vec![FF::from(0)], vec![FF::from(1)]]
    } else {
        let mut output: Vec<Vec<FF>> = Vec::new();
        for item in hypercube(size - 1) {
            output.push([item.clone(), vec![FF::from(0)]].concat());
            output.push([item.clone(), vec![FF::from(1)]].concat());
        }
        output
    }
}

// Evaluating a multivariate polynomial at the boolean hypercube.
fn hypercube_evaluation<FF: Field + std::convert::From<i32>>(
    polynomial: SparsePolynomial<FF, SparseTerm>,
) -> FF {
    let mut output = FF::from(0);
    for point in hypercube(polynomial.num_vars().try_into().unwrap()) {
        output += polynomial.evaluate(&point);
    }
    output
}

// Raising a field element into a non-negative power.
// Negative powers are not implemented, as we are not going to need them.
// This is probably a standard function but I could not find it.
pub fn raise_to_power<FF: Field + std::convert::From<i32>>(input: FF, power: usize) -> FF {
    if power == 0 {
        FF::from(1)
    } else if power == 1 {
        input
    } else {
        let mut output = FF::from(1);
        for i in 0..power {
            output *= input
        }
        output
    }
}

// Evaluate a monomial, i.e. an instance of SparseTerm, at a point.
// To avoid any potential confusion: x^2*y^3 is a monomial, but 2*x^2*y^3 is not.
fn monomial_evaluate<FF: Field + std::convert::From<i32>>(
    monomial: SparseTerm,
    coordinates: Vec<FF>,
) -> FF {
    let mut output = FF::from(1);
    for i in 0..monomial.vars().len() {
        output *= raise_to_power(coordinates[monomial.vars()[i]], monomial.powers()[i]);
    }
    output
}

// Evaluate all variables but one in a monomial, i.e. in an instance of SparseTerm.
// As input we get the index of the "missing coordinate", i.e. the coordinate that is not evaluated,
// and two lists coordinates_part_1 and coordinates_part_2 that specify the values of the coordinates
// that come before and after the "missing coordinate".
// As output we get a univariate polynomial.
fn monomial_partial_evaluate<FF: Field + std::convert::From<i32>>(
    monomial: SparseTerm,
    missing_coordinate: usize,
    coordinates_part_1: Vec<FF>,
    coordinates_part_2: Vec<FF>,
) -> univariate::SparsePolynomial<FF> {
    let mut coefficient = FF::from(1);
    let mut power: usize = 0;

    for i in 0..monomial.vars().len() {
        if monomial.vars()[i] < missing_coordinate {
            coefficient *=
                raise_to_power(coordinates_part_1[monomial.vars()[i]], monomial.powers()[i]);
        } else if monomial.vars()[i] == missing_coordinate {
            power = monomial.powers()[i];
        } else {
            coefficient *= raise_to_power(
                coordinates_part_2[monomial.vars()[i] - missing_coordinate - 1],
                monomial.powers()[i],
            );
        }
    }
    // This does not cause a bug, as we are only creating a polynomial from one monomial
    univariate::SparsePolynomial::from_coefficients_vec(vec![(
        power.try_into().unwrap(),
        FF::from(coefficient),
    )])
}

// Evaluate all variables but one in a multivariate polynomial, as required in the sumcheck protocol.
// The notation is the same as in the function monomial_partial_evaluate above.
fn polynomial_partial_evaluate<FF: Field + std::convert::From<i32>>(
    polynomial: SparsePolynomial<FF, SparseTerm>,
    missing_coordinate: usize,
    coordinates_part_1: Vec<FF>,
    coordinates_part_2: Vec<FF>,
) -> univariate::SparsePolynomial<FF> {
    let mut output = univariate::SparsePolynomial::zero();

    for item in polynomial.terms() {
        output = output.add(
            monomial_partial_evaluate(
                item.1.clone(),
                missing_coordinate.clone(),
                coordinates_part_1.clone(),
                coordinates_part_2.clone(),
            )
            .mul(&univariate::SparsePolynomial::from_coefficients_vec(vec![
                (0, item.0.clone()),
            ])),
        )
    }
    output
}

// Degree of a multivariate polynomial in one variable, called missing_coordinate as in the sumcheck.
pub fn degree_in_one_variable<FF: Field + std::convert::From<i32>>(
    polynomial: SparsePolynomial<FF, SparseTerm>,
    missing_coordinate: usize,
) -> usize {
    let mut coordinates_part_1: Vec<FF> = Vec::new();
    let mut coordinates_part_2: Vec<FF> = Vec::new();
    for i in 0..missing_coordinate {
        coordinates_part_1.push(FF::from(1))
    }
    for i in missing_coordinate + 1..polynomial.num_vars {
        coordinates_part_2.push(FF::from(1))
    }

    polynomial_partial_evaluate(
        polynomial,
        missing_coordinate,
        coordinates_part_1,
        coordinates_part_2,
    )
    .degree()
}

pub fn univariate_hypercube_evaluate<FF: Field + std::convert::From<i32>>(
    polynomial: univariate::SparsePolynomial<FF>,
) -> FF {
    polynomial.evaluate(&FF::from(0)) + polynomial.evaluate(&FF::from(1))
}

pub fn run_sumcheck_protocol<FF: Field + std::convert::From<i32>>(
    input_polynomial: SparsePolynomial<FF, SparseTerm>,
) {
    let verbose = true;

    if verbose {
        println!("Input polynomial is {:?}", input_polynomial);
        println!("");
    }

    let prover = Prover {
        polynomial: input_polynomial.clone(),
    };

    let verifier = Verifier {
        polynomial: input_polynomial.clone(),
    };

    // Prover computes the sum of the evaluations of the input_polynomial over the boolean hypercube
    let c = hypercube_evaluation(prover.polynomial.clone());

    if verbose {
        println!("Prover sends the constant C = {:?}", c);
        println!("");
    }

    // Steps 1 to N
    let mut provers_messages: Vec<univariate::SparsePolynomial<FF>> = vec![];
    let mut random_challenges: Vec<FF> = vec![];

    for i in 0..prover.polynomial.num_vars {
        if verbose {
            println!("Entering round {} of interaction:", i + 1);
        }
        provers_messages = [provers_messages, vec![prover.message(&random_challenges)]].concat();

        if verbose {
            println!("Prover sends the polynomial {:?}", provers_messages[i]);
            println!("");
        }

        if i == 0 {
            let sum = univariate_hypercube_evaluate(provers_messages[i].clone());

            if sum == c {
                if provers_messages[i].clone().degree()
                    <= degree_in_one_variable(verifier.polynomial.clone(), i)
                {
                    // generating a new random number
                    random_challenges = [random_challenges, vec![verifier.message()]].concat();
                }
            } else {
                println!("Check in round {} failed", i + 1);
                break;
            }
        } else {
            if univariate_hypercube_evaluate(provers_messages[i].clone())
                == provers_messages[i - 1].evaluate(&random_challenges[i - 1])
            {
                if provers_messages[i].clone().degree()
                    <= degree_in_one_variable(verifier.polynomial.clone(), i)
                {
                    // generating a new random number
                    random_challenges = [random_challenges, vec![verifier.message()]].concat();
                }
            } else {
                println!("Check in round {} failed", i + 1);
                break;
            }
        }
    }

    // Final check by the verifier
    if verbose {
        println!("Entering the final round of interaction:");
    }
    if provers_messages[prover.polynomial.num_vars-1]
        .evaluate(&random_challenges[prover.polynomial.num_vars-1])// + FF::from(1)
        == verifier.polynomial.evaluate(&random_challenges)
    {
        println!("Verifyer accepts.");
    } else {
        println!("Final check failed. Verifyer rejects.");
    }
}
