mod complex;
mod fourier_transform;
mod graphing;
mod utils;

use complex::Complex;
use std::f64::consts::PI;
use fourier_transform::{dft, fft};

#[allow(non_snake_case)]
fn main() {
    let fs = 4096.0;
    let f_goal = (3.0, 2.5);
    let t_max = 4.0;

    let t = utils::sequence(0.0, t_max, 0.5 / fs);

    let x: Vec<Complex<f64>> = t
        .iter()
        .map(|tn| (f_goal.0 * t_max * PI * tn).cos() + (f_goal.1 * t_max * PI * tn).cos())
        .map(|val| Complex::new(val, 0.0))
        .collect();

    let dur = utils::benchmark(|| { dft(&x); });
    let nans = dur.as_nanos();
    let secs = nans as f64 / 1_000_000_000.0;
    println!("DFT: {:.6} seconds \t{:6} ns per sample",
        secs, nans / x.len() as u128);

    let dur = utils::benchmark(|| { fft(&x).unwrap(); });
    let nans = dur.as_nanos();
    let secs = nans as f64 / 1_000_000_000.0;
    println!("FFT: {:.6} seconds \t{:6} ns per sample",
        secs, nans / x.len() as u128);

    let X = fft(&x).unwrap();

    let n = X.len();
    let T = (n as f64) / fs;
    let f: Vec<f64> = (0..n)
        .into_iter()
        .map(|k| (k as f64) / T)
        .collect();

    graphing::plot(&t, &x, &f, &X)
        .expect("Failed to plot");
}
