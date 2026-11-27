use std::{f64::consts::PI, sync::Arc};

use qube_sim::{qubit::{QuantumCircuit}, sampler::Sampler};

fn initialize(qc: &mut QuantumCircuit) {
    let qubit_count = qc.qubit_count();
    for i in 0..qubit_count-1 {
        let _ = qc.H(i+1);
    }
    
    let _ = qc.X(qubit_count);
    let _ = qc.H(qubit_count);
}

fn oracle(qc: &mut QuantumCircuit, key: usize) {
    let qubit_count = qc.qubit_count();
    for i in 0..qubit_count-1 {
        if key & (1 << i) == 0 {
            let _ = qc.X(i+1);
        }
    }

    let controls: Vec<usize> = (1..qubit_count).collect();
    let _ = qc.MCNOT(&controls, qubit_count);

    for i in 0..qubit_count-1 {
        if key & (1 << i) == 0 {
            let _ = qc.X(i+1);
        }
    }
}

fn diffuse(qc: &mut QuantumCircuit) {
    let qubit_count = qc.qubit_count();
    for i in 0..qubit_count-1 {
        let _ = qc.H(i+1);
        let _ = qc.X(i+1);
    }
    let _ = qc.H(qubit_count-1);

    let controls: Vec<usize> = (1..qubit_count-1).collect();
    let _ = qc.MCNOT(&controls, qubit_count-1);

    let _ = qc.H(qubit_count-1);

    for i in 0..qubit_count-1 {
        let _ = qc.X(i+1);
        let _ = qc.H(i+1);
    }
}

// Note: running this example can take a while!
fn main() {
    let qubit_count: usize = 6;
    let key: usize = rand::random_range(0..(1<<qubit_count));

    println!("Simulating Grover's algorithm using a random number generator as a \"black box\" with key {}...", key);

    // calculate the number of times to loop the algorithm
    let n = 2_usize.pow(qubit_count as u32);
    let iter_count = (PI * (n as f64).sqrt() / 4.0).ceil() as usize;
    
    Sampler::print_samples_raw_numbers(Arc::new(move || {
        let mut qc = QuantumCircuit::new(qubit_count + 1);

        initialize(&mut qc);
        for _ in 0..iter_count {
            oracle(&mut qc, key);
            diffuse(&mut qc);
        }

        let _ = qc.H(qubit_count+1);
        let _ = qc.X(qubit_count+1);

        qc.Msingle(qubit_count + 1);

        qc.Mrev()
    }), 1000, 100);
}