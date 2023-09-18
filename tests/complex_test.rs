
#[cfg(test)]
mod complex_test {
    use quantcomp::complex::Complex;

    #[test]
    fn new() {
        let c = Complex::new(1.0, 2.0);
        assert_eq!(c, Complex{re: 1.0, im: 2.0});
    }

    #[test]
    fn parse() {
        assert_eq!(Complex{re: 2.0, im: 3.0}, Complex::parse("2+3i"));
        assert_eq!(Complex{re: 2.23, im: 3.51312}, Complex::parse("2.23+3.51312i"));
        assert_eq!(Complex{re: 0.1, im: 0.0}, Complex::parse(".1+0.i"));
        assert_eq!(Complex{re: -2.0, im: 3.0}, Complex::parse("-2+3i"));
        assert_eq!(Complex{re: -2.232, im: -0.532}, Complex::parse("-2.232-.532i"));
    }

    #[test]
    fn display() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(1.0, -2.0);
        assert_eq!("1.0+2.0i", c1.to_string());
        assert_eq!("1.0-2.0i", c2.to_string());

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
        assert_eq!(c1+c2, Complex{re: 3.0, im: 5.0});
    }

    #[test]
    fn sub() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(2.0, 3.0);
        assert_eq!(c1-c2, Complex{re: -1.0, im: -1.0});
    }

    #[test]
    fn mul() {
        let c1 = Complex::new(1.0, 2.0);
        let c2 = Complex::new(2.0, 3.0);
        assert_eq!(c1*c2, Complex{re: -4.0, im: 7.0});
    }

}