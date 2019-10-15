mod complex;
mod fourier_transform;
mod graphing;
mod utils;

use complex::Complex;
use std::f64::consts::PI;

#[allow(non_snake_case)]
fn main() {
    let fs = 1024.0;
    let f_goal = (3.0, 2.5);
    let t_max = 4.0;

    let t = utils::sequence(0.0, t_max, 0.5 / fs);

    let x: Vec<Complex<f64>> = t
        .iter()
        .map(|tn| (f_goal.0 * t_max * PI * tn).cos() + (f_goal.1 * t_max * PI * tn).cos())
        .map(|val| Complex::new(val, 0.0))
        .collect();

    let X = fourier_transform::dft(&x);

    let n = x.len();
    let T = (n as f64) / fs;
    let f: Vec<f64> = (0..n)
        .into_iter()
        .map(|k| (k as f64) / T)
        .collect();

    graphing::plot(&t, &x, &f, &X)
        .expect("Failed to plot");
}
