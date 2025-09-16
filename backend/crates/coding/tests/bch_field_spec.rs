use coding::{BCHCode, Message};
use finite_field::field2m::FiniteField2m;
use finite_field::gfp::GFp;

#[test]
fn gf2m_minimal_polynomial_root_property() {
    // GF(2^4) with px = x^4 + x + 1
    let px = vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)];
    let field = FiniteField2m::new(&px);
    // minimal polynomials for alpha^1 and alpha^3 should be degree 4 and vanish at those elements
    for &i in &[1usize, 3usize] {
        let p = field.minimal_polynomial_over_gf2(i);
        assert_eq!(p.deg(), 4);
        // evaluate at a = alpha^i
        let a = field.alpha_pow(i % field.n);
        // Horner over GF(2^m) with GF(2) coeffs
        let mut acc: u16 = 0;
        for c in p.coeffs.iter().rev() {
            acc = field.mul(acc, a);
            if c.0 == 1 { acc ^= 1; }
        }
        assert_eq!(acc, 0, "p(alpha^{i}) should be 0");
        // monic and constant term 1 for primitive cosets
        assert_eq!(p.coeffs[0], GFp::<2>(1));
        assert_eq!(*p.coeffs.last().unwrap(), GFp::<2>(1));
    }
}

#[test]
fn bch_15_7_bm_chien_roundtrip_t2() {
    // GF(2^4), narrow-sense BCH(15,7), t=2
    let px = vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)];
    let field = FiniteField2m::new(&px);
    let bch: BCHCode = BCHCode::new_with_field(field, 15, 2);
    assert_eq!(bch.n, 15);
    let k = bch.k();
    // message: simple basis vector
    let mut msg = vec![GFp::<2>(0); k];
    msg[0] = GFp::<2>(1);
    let u = Message::from(linalg::Vector::new(msg.clone()));
    let c = bch.encode(&u).expect("encode");
    // introduce up to t=2 bit errors
    let mut r1 = c.as_ref().clone();
    r1[3] = if r1[3] == GFp::<2>(0) { GFp::<2>(1) } else { GFp::<2>(0) };
    let dec1 = bch.decode_bm(&r1.into(), 1).expect("bm+chien decode 1bit");
    assert_eq!(dec1.as_ref().data, c.as_ref().data);

    let mut r2 = c.as_ref().clone();
    r2[1] = if r2[1] == GFp::<2>(0) { GFp::<2>(1) } else { GFp::<2>(0) };
    r2[5] = if r2[5] == GFp::<2>(0) { GFp::<2>(1) } else { GFp::<2>(0) };
    let dec2 = bch.decode_bm(&r2.into(), 1).expect("bm+chien decode 2bit");
    assert_eq!(dec2.as_ref().data, c.as_ref().data);
}
