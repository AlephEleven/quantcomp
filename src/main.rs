use quantcomp::complex::Complex;

fn main() {

    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(2.0, 3.0);

    println!("{}", c1);
    println!("{}", c1.conj());

    println!("{}", c1+c2);
    println!("{}", c1*c2);
    println!("{}", c1/c2);
    println!("{}", c1.abs());
    println!("{}", c1.prob());
    println!("{}", c2.conj().exp());
}
