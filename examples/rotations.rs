use std::{f64::consts::PI, sync::Arc};

use qube_sim::{qubit::QuantumCircuit, sampler::Sampler};

fn main() {
    println!("Simulating rotation about a Bloch sphere...\n");

    Sampler::print_samples(Arc::new(|| {
        let mut qc = QuantumCircuit::new(3);

        // The operations done on each qubit should all be equivalent
        let _ = qc.Rx(1, PI);

        let _ = qc.X(2);

        let _ = qc.H(3);
        for _ in 0..2 {
            let _ = qc.S(3);
        }
        let _ = qc.H(3);

        qc.M()
    }), 100000, 10);
}