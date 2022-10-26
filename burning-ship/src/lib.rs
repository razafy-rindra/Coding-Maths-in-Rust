use std::ops::{Add, Mul};


/// The set of complex numbers extend the real numbers by adding in an element i such that i^2 = -1.
///
/// A complex number can be expressed in the form a+bi, where a,b are real numbers. a is called the
/// "real" part and b is called the "imaginary" part.
#[derive(Copy, Clone)]
pub struct Complex {
    real: f32,
    imaginary: f32,
}

impl Complex {
    pub fn new(real: f32, imaginary: f32) -> Self {
        Self { real, imaginary }
    }

    pub fn re(&self) -> f32 {
        self.real
    }

    pub fn im(&self) -> f32 {
        self.imaginary
    }
    /// The norm of a complex number tells you the distance between the number seen as a point in
    /// R^2 and the orgin. 
    pub fn norm(&self) -> f32 {
        let n = self.real * self.real + self.imaginary * self.imaginary;
        n.sqrt()
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let zr = self.real * rhs.real - self.imaginary * rhs.imaginary;
        let zi = self.imaginary * rhs.real + self.real * rhs.imaginary;
        Complex::new(zr, zi)
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Complex::new(self.real + other.real, self.imaginary + other.imaginary)
    }
}

