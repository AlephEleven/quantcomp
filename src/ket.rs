use crate::complex::Complex;
use std::ops::{
    Add,
    Sub,
    Mul,
    Div
};
use std::iter::zip;




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
        assert_eq!(n, data.len());

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

impl Add<QuantumMatrix> for QuantumMatrix{
    type Output = QuantumMatrix;
    fn add(self, rhs: QuantumMatrix) -> QuantumMatrix {
        assert_eq!(self.shape, rhs.shape);
        QuantumMatrix {data: zip(self.data, rhs.data).into_iter().map(|(x, y)| x+y).collect(),
                       shape: self.shape,
                       size: self.size}
    }
}

impl Sub<QuantumMatrix> for QuantumMatrix{
    type Output = QuantumMatrix;
    fn sub(self, rhs: QuantumMatrix) -> QuantumMatrix {
        assert_eq!(self.shape, rhs.shape);
        QuantumMatrix {data: zip(self.data, rhs.data).into_iter().map(|(x, y)| x-y).collect(),
                       shape: self.shape,
                       size: self.size}
    }
}

impl Mul<QuantumMatrix> for QuantumMatrix{
    type Output = QuantumMatrix;
    fn mul(self, rhs: QuantumMatrix) -> QuantumMatrix {
        assert_eq!(self.shape, rhs.shape);
        QuantumMatrix {data: zip(self.data, rhs.data).into_iter().map(|(x, y)| x*y).collect(),
                       shape: self.shape,
                       size: self.size}
    }
}

impl Div<QuantumMatrix> for QuantumMatrix{
    type Output = QuantumMatrix;
    fn div(self, rhs: QuantumMatrix) -> QuantumMatrix {
        assert_eq!(self.shape, rhs.shape);
        QuantumMatrix {data: zip(self.data, rhs.data).into_iter().map(|(x, y)| x/y).collect(),
                       shape: self.shape,
                       size: self.size}
    }
}