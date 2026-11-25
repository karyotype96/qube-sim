use std::{f64::consts::PI, sync::Arc};

use qube_sim::{qubit::{QuantumCircuit}, sampler::Sampler};

fn diffuse(qc: &mut QuantumCircuit, qubit_count: usize, key: usize) {
    for i in 0..qubit_count {
        if key & (1 << i) > 0 {
            let _ = qc.X(i+1);
        }
    }

    let controls: Vec<usize> = (1..qubit_count+1).collect();
    let _ = qc.MCNOT(&controls, qubit_count + 1);
}

fn main() {
    println!("Simulating Grover's algorithm using a random number generator as a \"black box\"...");

    let qubit_count: usize = 4;
    let key: usize = rand::random_range(0..(1<<4));

    Sampler::print_samples(Arc::new(move || {
        let mut qc = QuantumCircuit::new(qubit_count + 1);

        // Put all qubits besides the ancillary qubit into Hadamard states
        for i in 1..qubit_count+1 {
            let _ = qc.H(i);
        }
        // Set ancillary qubit to |-> state
        let _ = qc.X(5);
        let _ = qc.H(5);

        let iter_count = (PI * (qubit_count as f64).sqrt() / 4.0) as usize;

        for _ in 0..iter_count {
            diffuse(&mut qc, qubit_count, key);
        }

        qc.M()
    }), 100, 4);
}