use num_complex::Complex;
use signal_processing::dft::dft;
use signal_processing::fir::{
    design_fir_bandpass, design_fir_bandstop, design_fir_highpass, design_fir_lowpass,
};
use signal_processing::window::WindowType;
use textplots::{Chart, Plot, Shape};

fn to_complex(v: &[f64]) -> Vec<Complex<f64>> {
    v.iter().map(|&r| Complex::new(r, 0.0)).collect()
}

fn main() {
    let n_fft = 512usize;

    let filters = vec![
        (
            "lowpass",
            design_fir_lowpass(101, 0.15, WindowType::Hamming),
        ),
        (
            "highpass",
            design_fir_highpass(101, 0.2, WindowType::Hamming),
        ),
        (
            "bandpass",
            design_fir_bandpass(101, 0.18, 0.3, WindowType::Hamming),
        ),
        (
            "bandstop",
            design_fir_bandstop(101, 0.18, 0.3, WindowType::Hamming),
        ),
    ];

    for (name, h) in filters {
        let h_freq = dft(&to_complex(&h));
        let mags: Vec<f32> = (0..(n_fft.min(h_freq.len())))
            .map(|k| h_freq[k].norm() as f32)
            .collect();
        println!("FIR {name} magnitude (first half)");
        let half = mags.len() / 2;
        let points: Vec<(f32, f32)> = (0..half)
            .map(|k| (k as f32 / half as f32, mags[k]))
            .collect();
        Chart::new(120, 20, 0.0, 1.0)
            .lineplot(&Shape::Lines(&points[..]))
            .display();
    }
}
