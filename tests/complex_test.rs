
#[cfg(test)]
mod complex_test {
    use quantcomp::complex::Complex;

    #[test]
    fn new() {
        let c = Complex::new(1.0, 2.0);
        assert_eq!(c, Complex{re: 1.0, im: 2.0});
    }

    #[test]
    fn conj() {
        let c = Complex::new(1.0, 2.0);
        assert_eq!(c.conj(), Complex{re: 1.0, im: -2.0});
    }

    #[test]
    fn add() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(2.0, 3.0);
        assert_eq!(c1+c2, Complex{re: 2.0, im: 5.0});
    }
}