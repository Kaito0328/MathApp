use lti_systems::discrete::TransferFunction;
use poly::polynomial::Polynomial;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 例: 離散一時差分器 H(z) = (1 - z^-1)
    let tf = TransferFunction::new(Polynomial::new(vec![1.0, -1.0]), Polynomial::new(vec![1.0]));

    let out = format!("{}/plot/bode_diff.svg", env!("CARGO_MANIFEST_DIR"));
    std::fs::create_dir_all(format!("{}/plot", env!("CARGO_MANIFEST_DIR")))?;
    tf.plot_bode_svg_simple(&out, 900, 540, 256)?;
    println!("Wrote {out}");
    Ok(())
}
