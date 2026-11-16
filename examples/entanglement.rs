use qube_sim::{qubit::QuantumCircuit, sampler::Sampler};

fn main() {
    Sampler::print_samples(|| {
        let mut qc = QuantumCircuit::new(2);

        let _ = qc.H(1);
        let _ = qc.CNOT(1, 2);

        qc.M()
    }, 10000);
}