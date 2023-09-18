use crate::complex::Complex;



#[derive(Debug, PartialEq)]
pub struct QuantumMatrix {
    pub data: Vec<Complex>,
    pub shape: Vec<usize>,
    pub size: usize
}

#[derive(Debug, PartialEq)]
pub enum QuantumState {
    Bra(QuantumMatrix, String),
    Ket(QuantumMatrix, String),
    Braket(QuantumMatrix, String, String),
    Ketbra(QuantumMatrix, String, String)
}


impl QuantumMatrix {
    pub fn new(data: Vec<Complex>, shape: Vec<usize>) -> QuantumMatrix {
        let n = shape.iter().product();
        if n != data.len() {
            panic!("QuantumMatrix shape does not match data size")
        }

        QuantumMatrix { data: data, shape: shape, size: n}
    }

    pub fn transpose(self) -> QuantumMatrix {
        match self.shape.as_slice() {
            [1, _] | [_, 1] => QuantumMatrix { data: self.data.iter().map(|&x| Complex::conj(x)).collect(), 
                                                       shape: self.shape.into_iter().rev().collect(), 
                                                        size: self.size },
            _ => todo!()
        }
    }
}
