use crate::complex::Complex;



#[derive(Debug, PartialEq)]
pub enum QuantumState {
    Bra(String),
    Ket(String),
    Braket(String, String),
    Ketbra(String, String)
}

#[derive(Debug, PartialEq)]
pub struct QuantumMatrix {
    pub data: Vec<Complex>,
    pub shape: Vec<usize>,
    pub size: usize,
    pub state_id: QuantumState
}


impl QuantumMatrix {
    pub fn new(data: Vec<Complex>, shape: Vec<usize>, state_id: QuantumState) -> QuantumMatrix {
        let n = shape.iter().product();
        if n != data.len() {
            panic!("QuantumMatrix shape does not match data size")
        }

        match state_id {
            QuantumState::Bra(sym) => QuantumMatrix { data: data, shape: vec![1, n], size: n, state_id: QuantumState::Bra(sym)},
            QuantumState::Ket(sym) => QuantumMatrix { data: data, shape: vec![n, 1], size: n, state_id: QuantumState::Ket(sym)},
            QuantumState::Braket(sym1, sym2) => QuantumMatrix { data: data, shape: shape, size: n, state_id: QuantumState::Braket(sym1, sym2)},
            QuantumState::Ketbra(sym1, sym2) if n == 1 => QuantumMatrix { data: data, shape: vec![1, 1], size: n, state_id: QuantumState::Ketbra(sym1, sym2)},
            _ => panic!("QuantumMatrix invalid shape")
        }
    }
}
