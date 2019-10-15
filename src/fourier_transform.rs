use crate::complex::Complex;
use std::f64::consts::PI;

#[allow(non_snake_case)]
pub fn dft(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let N = x.len();
    let mut X = Vec::new();

    for k in 0..N {
        let mut sum = Complex::new(0.0, 0.0);

        for n in 0..N {
            let angle = (2.0 * PI) / (N as f64) * (k as f64) * (n as f64);
            sum += x[n] * Complex::new(angle.cos(), -angle.sin());
        }

        X.push(sum);
    }

    X
}
