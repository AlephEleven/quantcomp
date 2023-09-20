use quantcomp::{complex::Complex, 
                ket::QuantumMatrix,
                ket::QuantumState};

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

    println!("{}", Complex::parse("2+3i"));

    let qm = QuantumMatrix::new(vec![Complex::new(1.0, 2.0), Complex::new(2.0, 3.0), Complex::new(3.0, 4.0), Complex::new(5.0, 6.0)],vec![2, 2]);
    println!("{:?}", qm);
    println!("{:?}", qm.transpose());
    


}
