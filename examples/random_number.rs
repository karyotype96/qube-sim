use qube_sim::qubit::QuantumCircuit;

fn main() {
    let mut random_number: u64 = 0;

    for i in 0..64 {
        let mut qc = QuantumCircuit::new(1);
        
        let _ = qc.H(1);
        let bit = qc.M();

        random_number += (bit.1 as u64) << i;
    }

    println!("Random number generated: {}", random_number);
}