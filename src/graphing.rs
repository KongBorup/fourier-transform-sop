use std::error::Error;
use csv::Writer;
use std::process::Command;
use crate::complex::Complex;

#[allow(non_snake_case)]
pub fn plot(t: &[f64], x: &[Complex<f64>], f: &[f64], X: &[Complex<f64>]) -> Result<(), Box<dyn Error>> {
    let csv_path = "/home/adrian/coding/projects/fourier-sop/data/plotdata.csv";
    let mut wtr = Writer::from_path(csv_path)?;

    // Normalize X
    let X: Vec<Complex<f64>> = X.iter().map(|Xn| *Xn * (1.0 / (x.len() as f64))).collect();

    wtr.write_record(&["in_x", "in_y", "out_x", "out_y"])?;
    for i in 0..t.len() {
        wtr.write_record(&[
            t[i].to_string(),
            x[i].real().to_string(),
            f[i].to_string(),
            X[i].abs().to_string(),
        ])?;
    }
    
    let py_path = "/home/adrian/coding/projects/fourier-sop/src/python/fourier-plotter.py";

    let output = Command::new("sh")
        .arg("-c")
        .arg(&format!("python3 {}", py_path))
        .output()
        .expect("failed to execute python file");

    println!("PYTHON PLOTTING OUTPUT");
    println!("----------------------");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
