use poly::polynomial::special::{
    chebyshev_first_kind, chebyshev_second_kind, hermite_physicists, laguerre, legendre,
};

#[test]
fn chebyshev_t_low_degree() {
    // T0 = 1
    assert_eq!(chebyshev_first_kind::<f64>(0).coeffs, vec![1.0]);
    // T1 = x
    assert_eq!(chebyshev_first_kind::<f64>(1).coeffs, vec![0.0, 1.0]);
    // T2 = 2x^2 -1
    assert_eq!(chebyshev_first_kind::<f64>(2).coeffs, vec![-1.0, 0.0, 2.0]);
}

#[test]
fn chebyshev_u_low_degree() {
    // U0 = 1
    assert_eq!(chebyshev_second_kind::<f64>(0).coeffs, vec![1.0]);
    // U1 = 2x
    assert_eq!(chebyshev_second_kind::<f64>(1).coeffs, vec![0.0, 2.0]);
    // U2 = 4x^2 - 1
    assert_eq!(chebyshev_second_kind::<f64>(2).coeffs, vec![-1.0, 0.0, 4.0]);
}

#[test]
fn legendre_low_degree() {
    // P0 = 1, P1 = x, P2 = (3x^2-1)/2
    assert_eq!(legendre::<f64>(0).coeffs, vec![1.0]);
    assert_eq!(legendre::<f64>(1).coeffs, vec![0.0, 1.0]);
    let p2 = legendre::<f64>(2);
    // (3x^2-1)/2 -> coeffs: [-0.5, 0, 1.5]
    assert_eq!(p2.coeffs, vec![-0.5, 0.0, 1.5]);
}

#[test]
fn hermite_physicists_low_degree() {
    // H0=1, H1=2x, H2=4x^2-2
    assert_eq!(hermite_physicists::<f64>(0).coeffs, vec![1.0]);
    assert_eq!(hermite_physicists::<f64>(1).coeffs, vec![0.0, 2.0]);
    assert_eq!(hermite_physicists::<f64>(2).coeffs, vec![-2.0, 0.0, 4.0]);
}

#[test]
fn laguerre_low_degree() {
    // L0=1, L1=1-x, L2=1/2(x^2 -4x +2) => coeffs: [1.0, -1.0] and [1.0, -4.0, 1.0]/2
    assert_eq!(laguerre::<f64>(0).coeffs, vec![1.0]);
    assert_eq!(laguerre::<f64>(1).coeffs, vec![1.0, -1.0]);
    let l2 = laguerre::<f64>(2);
    // 実装の漸化式から得られる形を直接比較（[1, -2, 0.5] は 1 - 2x + 0.5x^2 = (x^2 - 4x + 2)/2）
    assert_eq!(l2.coeffs, vec![1.0, -2.0, 0.5]);
}
