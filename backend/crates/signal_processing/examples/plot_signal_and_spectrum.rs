use signal_processing::signal::Signal;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a sample time signal: sum of two sinusoids
    let n = 256usize;
    let fs = 256.0f64; // sample rate
    let f1 = 10.0f64;
    let f2 = 40.0f64;
    let x = Signal::new(
        (0..n)
            .map(|i| {
                let t = i as f64 / fs;
                (2.0 * std::f64::consts::PI * f1 * t).sin()
                    + 0.5 * (2.0 * std::f64::consts::PI * f2 * t).sin()
            })
            .collect(),
        fs,
    );

    let x_freq = x.dft();

    // Resolve output directory inside this crate
    let out_dir = format!("{}/plot", env!("CARGO_MANIFEST_DIR"));
    let _ = fs::create_dir_all(&out_dir);
    // Plot time-domain signal to SVG (axes + legend)
    let time_svg = format!("{out_dir}/time_signal.svg");
    x.save_svg_with_axes(&time_svg, 800, 400, "signal")?;

    // Plot magnitude spectrum (only first half due to symmetry for real signals)
    // Plot magnitude spectrum (dB) to SVG (axes + legend)
    let spec_svg = format!("{out_dir}/magnitude_spectrum.svg");
    x_freq.save_svg_magnitude_db_with_axes(&spec_svg, 800, 400, "|X| (dB)")?;

    println!("Wrote {out_dir}/time_signal.svg and {out_dir}/magnitude_spectrum.svg");
    Ok(())
}
