pub fn sequence(min: f64, max: f64, by: f64) -> Vec<f64> {
    let n = ((max - min) / by).abs().floor() as u32;
    let mut seq = Vec::new();

    for i in 0..n {
        seq.push(min + by * (i as f64));
    }

    seq
}