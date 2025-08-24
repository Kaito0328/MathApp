use lti_systems::{continuous::TransferFunction as CTF, discrete::TransferFunction as DTF};
use poly::polynomial::Polynomial;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Discrete: H(z) = (1 - z^-1)
    let h = DTF::new(Polynomial::new(vec![1.0, -1.0]), Polynomial::new(vec![1.0]));
    let out1 = format!(
        "{}/plot/block_discrete_unity.svg",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::create_dir_all(format!("{}/plot", env!("CARGO_MANIFEST_DIR")))?;
    h.plot_block_feedback_svg(&out1, 800, 320, true, None)?; // 負帰還・ユニティ
    println!("Wrote {out1}");

    let out2 = format!(
        "{}/plot/block_discrete_with_H.svg",
        env!("CARGO_MANIFEST_DIR")
    );
    h.plot_block_feedback_svg(&out2, 800, 360, true, Some("H(z)"))?; // 負帰還・H ブロック
    println!("Wrote {out2}");

    // Continuous: G(s) = 1/(s^2 + 0.4s + 1)
    let g = CTF::new(
        Polynomial::new(vec![1.0]),
        Polynomial::new(vec![1.0, 0.4, 1.0]),
    );
    let out3 = format!(
        "{}/plot/block_continuous_unity.svg",
        env!("CARGO_MANIFEST_DIR")
    );
    g.plot_block_feedback_svg(&out3, 900, 360, true, None)?;
    println!("Wrote {out3}");

    Ok(())
}
