use num_traits::Num;
use std::ops::{Mul, Add, AddAssign, Sub};
use std::convert::From;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Complex<T: Num> {
    re: T,
    im: T,
}

impl<T> Complex<T>
    where T: Num + Copy + Into<f64>,
{
    pub fn new(re: T, im: T) -> Self {
        Self {
            re,
            im,
        }
    }

    pub fn abs(&self) -> f64 {
        let re: f64 = self.re.into();
        let im: f64 = self.im.into();
        
        (re * re + im * im).sqrt()
    }

    pub fn real(&self) -> T {
        self.re
    }
}

impl<T> fmt::Display for Complex<T>
    where T: Num + fmt::Display + From<i32> + PartialOrd + Copy
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = if self.im < 0.into() { '-' } else { '+' };
        let im = if op == '-' { self.im * (-1).into() } else { self.im };

        write!(f, "{} {} {}i", self.re, op, im)
    }
}

impl<T: Num> Add for Complex<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl<T> Mul for Complex<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl<T> Mul<T> for Complex<T>
    where T: Num + Copy
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            re: self.re * scalar - self.im * scalar,
            im: self.re * scalar + self.im * scalar,
        }
    }
}

impl<T> AddAssign for Complex<T>
    where T: Num + Copy
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            re: self.re + other.re,
            im: self.im + other.im,
        };
    }
}

impl<T: Num> Sub for Complex<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}
