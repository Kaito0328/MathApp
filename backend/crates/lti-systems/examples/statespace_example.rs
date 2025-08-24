use lti_systems::statespace::{tf_c2d_zoh_siso, ContinuousStateSpace};
use poly::polynomial::Polynomial;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // G(s) = 1 / (s^2 + 0.4 s + 1)
    let num = Polynomial::new(vec![1.0]);
    let den = Polynomial::new(vec![1.0, 0.4, 1.0]);

    // 連続の可制御正準形を生成
    let css = ContinuousStateSpace::from_tf_siso(&num, &den);
    println!("CSS: {css}");

    // 連続SS → TF 表示（簡易逆変換）
    let g = css.to_tf_siso();
    println!("G(s) = {}", g.display_with("s"));

    // ZOH で離散化
    let fs = 20.0; // 20 Hz
    let dss = css.c2d_zoh(fs);
    println!("DSS (ZOH @ {fs} Hz): {dss}");

    // ショートカット: 直接 TF から ZOH 離散化
    let dss2 = tf_c2d_zoh_siso(&num, &den, fs);
    println!("DSS2 (from TF, ZOH): {dss2}");

    Ok(())
}
