use lti_systems::diagram::DiagramTf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build: ((G1 series G2) parallel G3) feedback with H (negative)
    let g1 = DiagramTf::atom_label("G1(z)");
    let g2 = DiagramTf::atom_label("G2(z)");
    let g3 = DiagramTf::atom_label("G3(z)");
    let h  = DiagramTf::atom_label("H(z)");

    let diag = g1.series(g2).parallel(g3).feedback(Some(h), true);
    let out = format!("{}/plot/diagram_ir.svg", env!("CARGO_MANIFEST_DIR"));
    std::fs::create_dir_all(format!("{}/plot", env!("CARGO_MANIFEST_DIR")))?;
    diag.to_svg(&out, 960, 480)?;
    println!("Wrote {out}");
    Ok(())
}
