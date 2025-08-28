use num_complex::Complex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Discrete ZPK ---
    let dzpk = lti_systems::zpk::DiscreteZpk::new(
        vec![Complex::new(1.0, 0.0)],                         // zeros
        vec![Complex::new(2.0, 0.0), Complex::new(3.0, 0.0)], // poles
        1.0,                                                  // gain
    );
    // 期待表示: (z-1)/(z-2)(z-3)
    println!("Discrete ZPK: {dzpk}");
    let dtf = dzpk.to_transfer_function(1.0);
    println!("H(z) = {}", dtf.display());
    let dzpk_rt = lti_systems::zpk::DiscreteZpk::from_transfer_function(&dtf);
    println!("Discrete ZPK (from TF): {dzpk_rt}");

    // --- Continuous ZPK ---
    // 低域通過 G(s) = 1/(s+1)
    let czpk = lti_systems::zpk::ContinuousZpk::new(vec![], vec![Complex::new(-1.0, 0.0)], 1.0);
    // 期待表示: 1/(s+1)
    println!("Continuous ZPK: {czpk}");
    let ctf = czpk.to_transfer_function();
    println!("G(s) = {}", ctf.display());
    let czpk_rt = lti_systems::zpk::ContinuousZpk::from_transfer_function(&ctf);
    println!("Continuous ZPK (from TF): {czpk_rt}");

    Ok(())
}
