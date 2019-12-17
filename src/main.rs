mod complex;
mod fourier_transform;
mod utils;

use complex::Complex;
use std::f64::consts::PI;
use fourier_transform::{dft, fft};
use csv;

#[allow(non_snake_case)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Definér csv-filen, hvori data skal gemmes
    let csv_path = "/home/adrian/coding/projects/fourier-sop/data/benchmark.csv";
    let mut wtr = csv::Writer::from_path(csv_path)?;

    // Skriv først kolonnernes navne
    wtr.write_record(&["N", "DFT", "FFT"])?;

    // Test med 12 toerpotenser. (Fra N=8 til N=16384)
    for i in 0..12 {
        let fs = 2.0_f64.powf(i as f64);
        // Generér inputsignalets samples som komplekse tal
        let x = generate_signal(fs);

        // Benchmark DFT-algoritmen
        let avg_dur = utils::benchmark(|| { dft(&x); });
        let ms_dft = avg_dur as f64 / 1_000_000.0;
        print!("DFT: N = {}, {:.6} ms/iter", x.len(), ms_dft);

        // Benchmark FFT-algoritmen
        let avg_dur = utils::benchmark(|| { fft(&x).unwrap(); });
        let ms_fft = avg_dur as f64 / 1_000_000_000.0;
        println!("\t FFT: N = {}, {:.6} ms/iter", x.len(), ms_fft);

        // Gem benchmark for denne N i datafilen
        wtr.write_record(&[x.len().to_string(), ms_dft.to_string(), ms_fft.to_string()])?;
    }

    Ok(())
}

// Genererer et 4-sekunders signal af en sum af 2.5Hz- og 3Hz-bølger med `fs`
// samples per sekund.
pub fn generate_signal(fs: f64) -> Vec<Complex<f64>> {
    let f_goal = (3.0, 2.5);
    let t_max = 4.0;

    // Generér inputsignalets sample-tidspunkter
    let t = utils::sequence(0.0, t_max, 0.5 / fs);

    // Generér inputsignalets samples som komplekse tal
    let x: Vec<Complex<f64>> = t
        .iter()
        .map(|tn| (f_goal.0 * t_max * PI * tn).cos() + (f_goal.1 * t_max * PI * tn).cos())
        // Alle reelle tal er komplekse tal, hvis imaginære komponent er nul.
        .map(|val| Complex::new(val, 0.0))
        .collect();

    x
}
