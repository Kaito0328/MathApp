use lti_systems::continuous::TransferFunction as CTF;
use lti_systems::Polynomial;

fn main() -> std::io::Result<()> {
    // 例: G(s) = 1 / (s^2 + 0.4 s + 1)
    let g = CTF::new(
        Polynomial::new(vec![1.0]),
        Polynomial::new(vec![1.0, 0.4, 1.0]),
    );
    println!("G(s) = {g}");

    let out = format!("{}/plot/bode_cont.svg", env!("CARGO_MANIFEST_DIR"));
    std::fs::create_dir_all(format!("{}/plot", env!("CARGO_MANIFEST_DIR")))?;

    // 簡易Bode (f_min..f_max 対数サンプリング)
    g.plot_bode_svg_simple(&out, 900, 540, 1e-2, 1e3, 512)?;
    println!("Wrote {out}");
    Ok(())
}
