
#[cfg(test)]
mod ket_test {
    use quantcomp::complex::Complex;
    use quantcomp::ket::{QuantumMatrix};

    #[test]
    fn new() {
        let qm = QuantumMatrix::new(vec![Complex::new(1.0, 2.0), Complex::new(2.0, 3.0)], 
        vec![2, 1]);
        assert_eq!(qm, QuantumMatrix{data: vec![Complex::new(1.0, 2.0), Complex::new(2.0, 3.0)], 
                                     shape: vec![2, 1], 
                                     size: 2});
    }

    #[test]
    fn transpose1d() {
        let qm = QuantumMatrix::new(vec![Complex::new(1.0, 2.0), Complex::new(2.0, 3.0)], 
        vec![2, 1]);
        assert_eq!(qm.transpose(), QuantumMatrix{data: vec![Complex::new(1.0, -2.0), Complex::new(2.0, -3.0)], 
                                     shape: vec![1, 2], 
                                     size: 2});
    }
    #[test]
    fn transpose2d() {
        let qm = QuantumMatrix::new(vec![Complex::new(1.0, 2.0), Complex::new(2.0, 3.0), Complex::new(3.0, 4.0), Complex::new(5.0, 6.0)],vec![2, 2]);
        assert_eq!(qm.transpose(), QuantumMatrix{data: vec![Complex::new(1.0, -2.0), Complex::new(3.0, -4.0), Complex::new(2.0, -3.0), Complex::new(5.0, -6.0)], 
                                     shape: vec![2, 2], 
                                     size: 4});
    }
    #[test]
    fn tranposeidentity() {
        let qm = QuantumMatrix::new(vec![Complex::new(1.0, 2.0), Complex::new(2.0, 3.0)], 
        vec![2, 1]);
        assert_eq!(qm.transpose().transpose(), QuantumMatrix{data: vec![Complex::new(1.0, 2.0), Complex::new(2.0, 3.0)], 
                                     shape: vec![2, 1], 
                                     size: 2});
    }
    #[test]
    fn add() {
        let qm1 = QuantumMatrix::new(vec![Complex::new(1.0, 2.0), Complex::new(2.0, 3.0)],vec![2, 1]);
        let qm2 = QuantumMatrix::new(vec![Complex::new(3.0, 4.0), Complex::new(5.0, 6.0)],vec![2, 1]);
        assert_eq!(qm1+qm2, QuantumMatrix{data: vec![Complex::new(4.0, 6.0), Complex::new(7.0, 9.0)], 
                                     shape: vec![2, 1], 
                                     size: 2});
    }
    #[test]
    fn sub() {
        let qm1 = QuantumMatrix::new(vec![Complex::new(1.0, 2.0), Complex::new(2.0, 3.0)],vec![2, 1]);
        let qm2 = QuantumMatrix::new(vec![Complex::new(3.0, 4.0), Complex::new(5.0, 6.0)],vec![2, 1]);
        assert_eq!(qm1-qm2, QuantumMatrix{data: vec![Complex::new(-2.0, -2.0), Complex::new(-3.0, -3.0)], 
                                     shape: vec![2, 1], 
                                     size: 2});
    }
    #[test]
    fn mul() {
        let qm1 = QuantumMatrix::new(vec![Complex::new(1.0, 2.0), Complex::new(2.0, 3.0)],vec![2, 1]);
        let qm2 = QuantumMatrix::new(vec![Complex::new(3.0, 4.0), Complex::new(5.0, 6.0)],vec![2, 1]);
        assert_eq!(qm1*qm2, QuantumMatrix{data: vec![Complex::new(-5.0, 10.0), Complex::new(-8.0, 27.0)], 
                                     shape: vec![2, 1], 
                                     size: 2});
    }



}