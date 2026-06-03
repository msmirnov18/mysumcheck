# Sumcheck Protocol - Educational Rust Implementation

An educational implementation of the **sumcheck protocol** in Rust, using the [arkworks](https://arkworks.rs/) ecosystem for finite fields and multivariate polynomials.

The sumcheck protocol is a fundamental building block in modern zero-knowledge proof systems such as SNARKs.

A good reference for the theory is Justin Thaler's book [*Proofs, Arguments, and Zero-Knowledge*](https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.html).

## What This Code Does

- Defines `Prover` and `Verifier` structs that exchange messages in `n` rounds
- Implements the boolean hypercube enumeration
- Implements partial evaluation of multivariate polynomials
- Performs all arithmetic in the finite field **F₁₀₁** (integers mod 101), instantiated via `ark-ff`
- Runs a concrete example with a 4-variable polynomial and prints the full protocol transcript

## Project Structure

```
src/
├── main.rs       # Entry point; defines the example polynomial and runs the protocol
├── protocol.rs   # Core logic: Prover, Verifier, hypercube, polynomial evaluation
└── tests.rs      # Unit tests for helper functions
```

## Dependencies

| Crate | Purpose |
|---|---|
| [`ark-ff`](https://docs.rs/ark-ff) | Finite field arithmetic |
| [`ark-poly`](https://docs.rs/ark-poly) | Sparse multivariate and univariate polynomials |
| [`rand`](https://docs.rs/rand) / [`rand_chacha`](https://docs.rs/rand_chacha) | Randomness for verifier challenges |

## Getting Started

**Prerequisites:** [Rust](https://www.rust-lang.org/tools/install) (edition 2021)

```bash
git clone https://github.com/msmirnov18/mysumcheck
cd mysumcheck
cargo run
```

**Run the tests:**

```bash
cargo test
```