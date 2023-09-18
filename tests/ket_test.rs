
#[cfg(test)]
mod ket_test {
    use quantcomp::complex::Complex;
    use quantcomp::ket::{QuantumMatrix,
                         QuantumState};

    #[test]
    fn new() {
        let qm = QuantumMatrix::new(vec![Complex::new(1.0, 2.0), Complex::new(2.0, 3.0)], 
        vec![2, 1], 
        QuantumState::Ket(r"\Psi".to_string()));
        assert_eq!(qm, QuantumMatrix{data: vec![Complex::new(1.0, 2.0), Complex::new(2.0, 3.0)], 
                                     shape: vec![2, 1], 
                                     size: 2, 
                                     state_id: QuantumState::Ket(r"\Psi".to_string())});

    }

}