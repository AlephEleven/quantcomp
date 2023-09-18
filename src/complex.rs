use std::fmt;
use std::ops::{
    Add,
    Sub,
    Mul,
    Div
};
use regex::Regex;



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f32,
    pub im: f32
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Complex{
        Complex { re: re, im: im }
    }

    pub fn parse(str_complex: &str) -> Complex {
        let re = Regex::new(r"\s*(?<re>\-?\d*\.?\d*)\s*(?<im>[\+|\-]\d*\.?\d*)\s*i\s*").unwrap();
        let Some(caps) = re.captures(str_complex) else {
            panic!("Could not parse string!");
        };
        Complex { re: caps["re"].parse::<f32>().unwrap(), im: caps["im"].parse::<f32>().unwrap() }
    }

    pub fn real(&self) -> f32 {
        self.re
    }

    pub fn imag(&self) -> f32 {
        self.im
    }

    pub fn conj(self) -> Complex {
        Complex {re: self.re,
                 im: -self.im}
    }

    pub fn abs(self) -> Complex {
        Complex {re: f32::sqrt(self.re*self.re + self.im*self.im),
                 im: 0.0}
    }

    pub fn prob(self) -> Complex {
        Complex { re: self.re*self.re + self.im*self.im,
                  im: 0.0 }
    }

    pub fn exp(self) -> Complex {
        match self.im >= 0.0 {
            true =>
                Complex { re: f32::exp(self.re)*f32::cos(self.im),
                          im: f32::exp(self.re)*f32::sin(self.im)},
            false =>
                Complex { re: f32::exp(self.re)*f32::cos(-self.im),
                          im: -f32::exp(self.re)*f32::sin(-self.im)}
        }
    }

    pub fn sin(self) -> Complex {
        todo!()
    }

    pub fn cos(self) -> Complex {
        todo!()
    }


}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match self.im < 0.0 {
            true => "",
            false => "+"
        };
        write!(f, "{:?}{}{:?}i", self.re, op, self.im)
    }
}

impl Add<Complex> for Complex{
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl Sub<Complex> for Complex{
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Complex {
        Complex { re: self.re - rhs.re, im: self.im - rhs.im }
    }
}

impl Mul<Complex> for Complex{
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex { re: self.re*rhs.re - self.im*rhs.im, 
                  im: self.re*rhs.im + self.im*rhs.re}
    }
}

impl Div<Complex> for Complex{
    type Output = Complex;
    fn div(self, rhs: Complex) -> Complex {
        let denom = rhs.re*rhs.re + rhs.im*rhs.im;
        Complex { re: (self.re*rhs.re + self.im*rhs.im)/denom, 
                  im: (self.im*rhs.re - self.re*rhs.im)/denom}
    }
}