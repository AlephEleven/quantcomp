use crate::complex::Complex;

pub struct QuantumState {
    pub data: Vec<Complex>,
    pub shape: Vec<usize>,
    pub size: usize
}

