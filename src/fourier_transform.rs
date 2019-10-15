use crate::complex::Complex;
use std::f64::consts::PI;
use crate::utils;

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

#[allow(non_snake_case)]
pub fn fft(x: &Vec<Complex<f64>>) -> Result<Vec<Complex<f64>>, &str> {
    let N = x.len();
    
    if !utils::is_power_of_two(N) {
        return Err("Input length is not a power of two");
    }

    if N == 1 {
        return Ok(x.to_vec());
    }

    let N_half = N / 2;

    let mut x_even = Vec::with_capacity(N_half);
    let mut x_odd = Vec::with_capacity(N_half);

    for i in 0..N_half {
        x_even.push(x[2 * i]);
        x_odd.push(x[2 * i + 1]);
    }

    let X_even = fft(&x_even).unwrap();
    let X_odd = fft(&x_odd).unwrap();

    // FIXME: Create vector where i can index into the "future"
    // let mut X1 = Vec::with_capacity(N_half);
    // let mut X2 = Vec::with_capacity(N_half);

    // let mut X = Vec::with_capacity(N);

    let mut X = vec![Complex::new(0.0, 0.0); N];

    for k in 0..N_half {
        let angle = 2.0 * PI / (N as f64) * (k as f64);
        let doodledidoo = X_odd[k] * Complex::new(angle.cos(), -angle.sin());

        X[k] = X_even[k] + doodledidoo;
        X[k + N_half] = X_even[k] - doodledidoo;
        // X1.push(X_even[k] + doodledidoo);
        // X2.push(X_even[k] - doodledidoo);
    }
    
    // X.append(&mut X1);
    // X.append(&mut X2);

    Ok(X)
}
