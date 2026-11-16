use qube_sim::{qubit::QuantumCircuit, sampler::Sampler};

fn main() {
    Sampler::print_samples(|| {
        let mut qc = QuantumCircuit::new(1);

        let _ = qc.H(1);
        qc.M()
    }, 1000);
}