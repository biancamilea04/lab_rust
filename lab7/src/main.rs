#[derive(PartialEq, Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl From<i32> for Complex {
    fn from(var: i32) -> Self {
        Complex {
            real: var as f64,
            imag: 0.0,
        }
    }
}

impl From<f64> for Complex {
    fn from(var: f64) -> Self {
        Complex {
            real: var,
            imag: 0.0,
        }
    }
}

impl Complex {
    fn new<T1, T2>(real1: T1, imag1: T2) -> Self
    where
        f64: From<T1>,
        f64: From<T2>,
    {
        Complex {
            real: f64::from(real1),
            imag: f64::from(imag1),
        }
    }
    fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imag: self.imag * (-1.0),
        }
    }
}

use std::ops::Add;
impl<T> Add<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Complex;
    fn add(self, other: T) -> Self::Output {
        let other = other.into();
        Complex {
            imag: self.imag + other.imag,
            real: self.real + other.real,
        }
    }
}

use std::ops::Sub;
impl<T> Sub<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Complex;
    fn sub(self, other: T) -> Self::Output {
        let other = other.into();
        Complex {
            imag: self.imag - other.imag,
            real: self.real - other.real,
        }
    }
}

use std::ops::Mul;
impl<T> Mul<T> for Complex
where
    T: Into<Complex>,
{
    type Output = Complex;
    fn mul(self, other: T) -> Self::Output {
        let other = other.into();
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

use std::ops::Neg;
impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Self::Output {
        Complex {
            imag: self.imag * (-1.0),
            real: self.real * (-1.0),
        }
    }
}

use std::ops::AddAssign;
impl AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

use std::ops::SubAssign;
impl SubAssign for Complex {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

use std::ops::MulAssign;
impl MulAssign for Complex {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

use std::fmt;
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.real == 0.0 && self.imag == 0.0 {
            write!(f, "0")
        } else if self.real == 0.0 {
            write!(f, "{}i", self.imag)
        } else if self.imag == 0.0 {
            write!(f, "{}", self.real)
        } else if self.imag < 0.0 {
            write!(f, "{}{}i", self.real, self.imag)
        } else {
            write!(f, "{}+{}i", self.real, self.imag)
        }
    }
}

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}
