use snafu::Snafu;

pub type Result<T> = std::result::Result<T, QubeError>;

#[derive(Debug, Snafu)]
pub enum QubeError {
    #[snafu(display("Attempted to access element {}, above qubit count {}", index, max))]
    QubitOutOfRange { index: usize, max: usize },

    #[snafu(display("Control index equals target index: {}", index))]
    ControlEqualsTarget { index: usize },

    #[snafu(display("CCNOT gate control index 1 equals control index 2: {}", index))]
    Control1EqualsControl2 { index: usize },

    #[snafu(display("One-qubit custom gate matrix is not 2x2"))]
    OneQubitCustomGateNot2x2,

    #[snafu(display("One-qubit custom gate matrix not unitary"))]
    OneQubitCustomGateNotUnitary,
}