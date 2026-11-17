use qube_sim::{qubit::QuantumCircuit, sampler::Sampler};

fn main() {
    println!("A (very) simple circuit consisting of one qubit with one Hadamard gate...\n");

    Sampler::print_samples(|| {
        let mut qc = QuantumCircuit::new(1);

        let _ = qc.H(1);
        qc.M()
    }, 10000);
}