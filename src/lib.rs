//! [qube_sim] is a simple simulator for quantum circuits.
//! 
//! Provides tools for simulating quantum computation, primarily through
//! matrix manipulation, though some gates are simulated through simple
//! bit manipulation instead of using matrices.
//! 
//! A simulation can either be done once or multiple times. This crate
//! offers a sampler tool to run the circuit in a loop (or multiple loops
//! in parallel) and print out the probabilities of each outcome.
//! 
//! # Examples:
//! 
//! ```
//! use std::sync::Arc;
//! 
//! use qube_sim::{qubit::QuantumCircuit, sampler::Sampler};
//! 
//! fn main() {
//!     Sampler::print_samples(Arc::new(|| {
//!         let mut qc = QuantumCircuit::new(1);
//! 
//!         let _ = qc.H(1);
//!         qc.M()
//!     }), 10000, 10);
//! }
//! ```
pub mod complex;

pub mod qubit;
mod qubit_vector;
pub mod qubit_matrix;

pub mod error_handling;
pub mod sampler;