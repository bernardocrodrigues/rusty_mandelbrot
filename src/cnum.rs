
#[derive(Copy, Clone, Debug)]
pub struct ComplexNumber {
    pub real: f64,
    pub img: f64
}

impl std::ops::Add for ComplexNumber{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            real: self.real + other.real,
            img: self.img + other.img,
        }
    }
}

impl std::ops::Mul for ComplexNumber{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            real: (self.real * other.real) - (self.img * other.img),
            img: (self.real * other.img) + (other.img * self.real),
        }
    }
}

impl ComplexNumber{

    pub fn conjugate(&self) -> ComplexNumber{
        ComplexNumber {real: self.real, img: -self.img}
    }
}
 
impl From<i64> for ComplexNumber {
    fn from(a: i64) -> Self {
        ComplexNumber {real: a as f64, img: 0.0}
    }
}
