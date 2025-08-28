use signal_processing::signal::Signal;
use signal_processing::window::WindowType;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 元信号：サンプル数256、10Hz + 40Hz/2 の合成正弦波
    let n = 256usize;
    let fs = 256.0f64;
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

    let taps = 41usize;
    let win = WindowType::Hamming;

    let l = 3usize; // upsample factor
    let m = 2usize; // downsample factor

    let x_up = x.upsample(l, taps, win);
    let _x_down = x.downsample(m, taps, win);
    let x_resamp = x.resample(l, m, taps, win);

    let out_dir = format!("{}/plot", env!("CARGO_MANIFEST_DIR"));
    let _ = fs::create_dir_all(&out_dir);
    let svg_path = format!("{out_dir}/sampling_demo.svg");

    // 3段に分けず、複数系列を同一図に凡例付きで表示
    let _ = x.save_svg_multi(
        &svg_path,
        1000,
        400,
        &[
            (&x, "Original"),
            (&x_up, &format!("Upsampled x{l}")),
            (&x_resamp, &format!("Resampled x{l}/{m}")),
        ],
    );
    // Terminal summaries
    println!("original: {x}");
    println!("upsampled: {x_up}");
    println!("resampled: {x_resamp}");
    println!("Wrote {svg_path}");
    Ok(())
}
