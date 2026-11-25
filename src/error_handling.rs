use std::fmt;
use std::{fmt::{Debug}};

pub type Result<T> = std::result::Result<T, QubeError>;

/// A set of errors that may be returned by qubit operations.
pub enum QubeError {
    /// Returned when an operation attempts to access a qubit
    /// higher than the qubit count of the circuit.
    QubitOutOfRange { index: usize, max: usize },
    /// Returned when a CNOT operation uses the same index for
    /// the control qubit as the target qubit.
    ControlEqualsTarget { index: usize },
    /// Returned when a CCNOT operation uses the same index for
    /// both control qubits.
    Control1EqualsControl2 { index: usize },
    /// Returned when a custom gate entered into the Custom1
    /// method is not a 2x2 matrix.
    OneQubitCustomGateNot2x2,
    /// Returned when a custom gate entered into the Custom1
    /// method is not unitary - that is, the matrix times its
    /// adjoint is not the identity matrix.
    OneQubitCustomGateNotUnitary,
    /// Returned when the two qubits being swapped in the SWAP
    /// methods have the same index
    SwapSameQubit,
    /// Returned when the list of controls in the MCNOT method
    /// contains the index of the target qubit.
    ControlsContainTarget { target: usize },
}

impl fmt::Display for QubeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            QubeError::QubitOutOfRange { index, max } => format!("Attempted to access element {}, above qubit count {}", index, max),
            QubeError::ControlEqualsTarget { index } => format!("Control index equals target index: {}", index),
            QubeError::Control1EqualsControl2 { index } => format!("CCNOT gate control index 1 equal control index 2: {}", index),
            QubeError::OneQubitCustomGateNot2x2 => format!("One-qubit custom matrix gate is not 2x2"),
            QubeError::OneQubitCustomGateNotUnitary => format!("One-qubit custom gate matrix not unitary"),
            QubeError::SwapSameQubit => format!("Attempted to use SWAP on a qubit with itself"),
            QubeError::ControlsContainTarget { target } => format!("Control qubit list on MCNOT contains target qubit index: {}", target),
        };

        write!(f, "{}", s)
    }
}

impl Debug for QubeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            QubeError::QubitOutOfRange { index, max } => format!("Attempted to access element {}, above qubit count {}", index, max),
            QubeError::ControlEqualsTarget { index } => format!("Control index equals target index: {}", index),
            QubeError::Control1EqualsControl2 { index } => format!("CCNOT gate control index 1 equal control index 2: {}", index),
            QubeError::OneQubitCustomGateNot2x2 => format!("One-qubit custom matrix gate is not 2x2"),
            QubeError::OneQubitCustomGateNotUnitary => format!("One-qubit custom gate matrix not unitary"),
            QubeError::SwapSameQubit => format!("Attempted to use SWAP on a qubit with itself"),
            QubeError::ControlsContainTarget { target } => format!("Control qubit list on MCNOT contains target qubit index: {}", target),
        };

        write!(f, "Error: {}", s)
    }
}