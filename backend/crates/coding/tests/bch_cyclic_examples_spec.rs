use coding::{CyclicCode, GFp, Poly, GF256, Message};
use finite_field::gf256::gf256_from_u8;

#[test]
fn cyclic_encode_over_gf2_example_matches_example_output() {
    type F = GFp<2>;
    // 例の通り: n=7, g(x)=1+x+x^3, u=[1,0,1,1] -> 全て1
    let code = CyclicCode::<F>::new(7, vec![F::new(1), F::new(1), F::new(0), F::new(1)]);
    let u = vec![F::new(1), F::new(0), F::new(1), F::new(1)];
    let c = code.encode_poly(&u);
    assert_eq!(c, vec![F::new(1); 7]);
}

#[test]
fn bch_encode_minimal_demo_compiles_and_has_length_n() {
    // ダミー最小多項式で g を構築するデモに一致
    let m1 = Poly::<GF256>::new(vec![gf256_from_u8(1), gf256_from_u8(1)]);
    let m2 = Poly::<GF256>::new(vec![gf256_from_u8(1), gf256_from_u8(0), gf256_from_u8(1)]);
    let n = 15usize;
    // g = lcm(m1,m2) を使って CyclicCode で代替
    let mut g = Poly::<GF256>::one();
    g = Poly::lcm(&g, &m1);
    g = Poly::lcm(&g, &m2);
    let bch = CyclicCode::<GF256>::new(n, g.coeffs.clone());
    let k = bch.k();

    let u = Poly::new((0..k).map(|i| gf256_from_u8(i as u8 + 1)).collect());
    let msg = Message::from(linalg::Vector::new(u.coeffs.clone()));
    let c = bch.encode(&msg).expect("encode");
    assert_eq!(c.as_ref().dim(), n);
}
