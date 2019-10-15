use std::time::{Duration, Instant};

pub fn sequence(min: f64, max: f64, by: f64) -> Vec<f64> {
    let n = ((max - min) / by).abs().floor() as u32;
    let mut seq = Vec::new();

    for i in 0..n {
        seq.push(min + by * (i as f64));
    }

    seq
}

pub fn benchmark<F: Fn()>(f: F) -> Duration {
    let start = Instant::now();
    f();
    let duration = start.elapsed();

    duration
}

pub fn is_power_of_two(n: usize) -> bool {
    (n != 0) && (n & (n - 1) == 0)
}
