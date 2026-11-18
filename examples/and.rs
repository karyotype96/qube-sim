use qube_sim::{qubit::QuantumCircuit, sampler::Sampler};

fn main() {
    println!("Simulating AND using a Toffoli (CCNOT) gate...\n");

    Sampler::print_samples(|| {
        let mut qc = QuantumCircuit::new(3);

        let _ = qc.H(1);
        let _ = qc.H(2);
        let _ = qc.CCNOT(1, 2, 3);

        qc.M()
    }, 100000);
}