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
            // Værdien af eksponenten
            let angle = (2.0 * PI) / (N as f64) * (k as f64) * (n as f64);
            // Eulers formel benyttes til at danne det komplekse tal
            let complex_exp = Complex::new(angle.cos(), -angle.sin());
            // Læg den nuværende værdi til den foreløbelige sum
            sum += x[n] * complex_exp;
        }

        // DFT'en er beregnet for den nuværende iteration, gem dette `k`.
        X.push(sum);
    }

    X
}

#[allow(non_snake_case)]
pub fn fft(x: &Vec<Complex<f64>>) -> Result<Vec<Complex<f64>>, &str> {
    let N = x.len();
    
    // Denne implementering kræver, at N er en toerpotens. Derfor returneres en
    // fejl, hvis dette ikke er tilfældet - ellers ville programmet crashe ved
    // fejlagtigt input.
    if !utils::is_power_of_two(N) {
        return Err("Input length is not a power of two");
    }

    // Hvis længden er 1, kan vi ikke længere opdele summen.
    if N == 1 {
        return Ok(x.to_vec());
    }

    let N_half = N / 2;

    // Forbered to vektorer, hver med preallokeret plads til N/2 elementer, som
    // skal indeholde samples for de hhv. lige og ulige indekstal.
    let mut x_even = Vec::with_capacity(N_half);
    let mut x_odd = Vec::with_capacity(N_half);

    // 0..X producerer en liste fra 0 til X ekslusiv X, dvs. N-1 bliver opfyldt.
    for i in 0..N_half {
        x_even.push(x[2 * i]); // Lige indekstal
        x_odd.push(x[2 * i + 1]); // Ulige indekstal
    }

    // Foretag denne opdeling af summerne indtil N = 1.
    let X_even = fft(&x_even).unwrap();
    let X_odd = fft(&x_odd).unwrap();

    // Midlertidigt tomt output med allokeret plads til N elementer.
    let mut X = vec![Complex::new(0.0, 0.0); N];

    // Rekombinér nu resultatet igen gradvist jf. sommerfuglediagrammet.
    for k in 0..N_half {
        // Eksponenten til W_N^k = e^(-i * 2pi * k / N) uden -i.
        let wnk_exp = 2.0 * PI / (N as f64) * (k as f64);
        // Benyt Eulers formel til at danne det komplekse tal fra eksponenten.
        let wnk = Complex::new(wnk_exp.cos(), -wnk_exp.sin());
        let wnk_times_X_odd = wnk * X_odd[k];

        // Udnyt den komplekse eksponentialfunktions periodiske natur.
        X[k] = X_even[k] + wnk_times_X_odd;
        X[k + N_half] = X_even[k] - wnk_times_X_odd;
    }
    
    Ok(X)
}
