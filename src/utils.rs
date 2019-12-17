use std::time::{Duration, Instant};

// Genererer en jævnt fordelt talrække fra `min` til `max` med `by` mellemrum
pub fn sequence(min: f64, max: f64, by: f64) -> Vec<f64> {
    let n = ((max - min) / by).abs().floor() as u32;
    let mut seq = Vec::new();

    for i in 0..n {
        seq.push(min + by * (i as f64));
    }

    seq
}

// Benchmarker en funktion ved at eksekvere den 100 gange, hvorefter den gennem-
// snitlige eksekveringstid returneres i enheden ns/iter.
pub fn benchmark<F: Fn()>(f: F) -> u128 {
    let mut sum = Duration::new(0, 0);

    for _ in 0..100 {
        let start = Instant::now();
        f();
        sum += start.elapsed();
    }

    sum.as_nanos() / 100
}

// Tjekker om et tal er en toerpotens ved at sammenligne tallets bits.
pub fn is_power_of_two(n: usize) -> bool {
    (n != 0) && (n & (n - 1) == 0)
}
