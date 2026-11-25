use qube_sim::{qubit::QuantumCircuit};

fn main() {
    println!("A simulation/demonstration of quantum teleportation...\n");

    // In this circuit, the first qubit belongs to Alice, the third qubit
    // belongs to Bob, and the second is shared between them.
    let mut qc = QuantumCircuit::new(3);

    // Alice prepares her qubit into some other state
    let _ = qc.Y(1);

    // prepare the bridge qubit and Bob's qubit into the Bell state
    let _ = qc.H(2);
    let _ = qc.CNOT(2, 3);

    println!("qc state (initial): {}", qc.observe_debug());

    // CNOT Alice's qubit with the bridge qubit
    let _ = qc.CNOT(1, 2);

    // Apply Hadamard gate to Alice's qubit
    let _ = qc.H(1);

    println!("qc state (after CNOT): {}", qc.observe_debug());

    // measure Alice's qubit and the bridge qubit
    let z = qc.Msingle(1);
    println!("qc state (after Z is measured): {}", qc.observe_debug());
    let x = qc.Msingle(1);
    println!("qc state (after X is measured): {}", qc.observe_debug());

    println!("z = {}, x = {}", z, x);

    // Alice transmits the results of both measurements to Bob. If the bridge
    // qubit measures 1, Bob applies the Z gate to his qubit. Then, if Alice's
    // qubit measures 1, Bob applies the X gate to his qubit.
    if z {
        let _ = qc.Z(1);
    }
    if x {
        let _ = qc.X(1);
    }

    println!("qc state (Bob's qubit): {}", qc.observe_debug());
}