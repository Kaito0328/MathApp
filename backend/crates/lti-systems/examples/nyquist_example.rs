use lti_systems::plot::{ContinuousNyquistOptions, DiscreteNyquistOptions};
use lti_systems::Polynomial;
use lti_systems::{continuous::TransferFunction as CTF, discrete::TransferFunction as DTF};
use std::fs;

fn main() -> std::io::Result<()> {
    fs::create_dir_all(format!("{}/plot", env!("CARGO_MANIFEST_DIR")))?;
    // 出力先
    let discrete_out = format!("{}/plot/nyquist_discrete.svg", env!("CARGO_MANIFEST_DIR"));
    let continuous_out = format!("{}/plot/nyquist_continuous.svg", env!("CARGO_MANIFEST_DIR"));

    // --- 離散系の例: 移動平均と差分器 ---
    let ma = DTF::from_coeffs(vec![0.5, 0.5], vec![1.0]); // H(z) = 0.5(1 + z)
    let diff = DTF::from_coeffs(vec![-1.0, 1.0], vec![1.0]); // H(z) = z - 1

    let opts_z = DiscreteNyquistOptions {
        n_points: 720,
        legend: true,
        ..Default::default()
    };
    DTF::plot_nyquist_svg_multi(
        &[(&ma, "MA"), (&diff, "Diff")],
        &discrete_out,
        800,
        600,
        &opts_z,
    )?;

    // --- 連続系の例: 1次ローパスと2次 ---
    // G1(s) = 1 / (s + 1)
    let g1 = CTF::new(Polynomial::new(vec![1.0]), Polynomial::new(vec![1.0, 1.0]));
    // G2(s) = 1 / (s^2 + 0.4 s + 1)
    let g2 = CTF::new(
        Polynomial::new(vec![1.0]),
        Polynomial::new(vec![1.0, 0.4, 1.0]),
    );

    let opts_s = ContinuousNyquistOptions {
        n_points: 800,
        f_min_hz: 1e-2,
        f_max_hz: 1e3,
        legend: true,
        log_freq: true,
        ..Default::default()
    };
    CTF::plot_nyquist_svg_multi(
        &[(&g1, "1st LP"), (&g2, "2nd")],
        &continuous_out,
        800,
        600,
        &opts_s,
    )?;

    println!("Wrote plot/nyquist_discrete.svg and plot/nyquist_continuous.svg");
    Ok(())
}
