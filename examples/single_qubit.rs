use qube_sim::qubit::QuantumCircuit;

fn main() {
    let mut qc = QuantumCircuit::new(2);

    let _ = qc.H(1);
    let _ = qc.CNOT(1, 2);

    println!("qc = {}", qc.observe_debug());
    let a = qc.Msingle(1);
    println!("a = {a}");
    println!("qc = {}", qc.observe_debug());
    let b = qc.Msingle(1);
    println!("b = {b}");
}