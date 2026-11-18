use std::sync::Arc;

use qube_sim::{qubit::QuantumCircuit, sampler::Sampler};

fn main() {
    println!("Simulating quantum entanglement...\n");

    Sampler::print_samples(Arc::new(|| {
        let mut qc = QuantumCircuit::new(2);

        let _ = qc.H(1);
        let _ = qc.CNOT(1, 2);

        qc.M()
    }), 100000, 10);
}