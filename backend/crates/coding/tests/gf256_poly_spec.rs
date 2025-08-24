use finite_field::gf256::gf256_from_u8;
use coding::{PolyGF256, GF256};

fn b(x: u8) -> GF256 {
    gf256_from_u8(x)
}

#[test]
fn gf256_add_mul_examples() {
    // FIPS-197 より既知の例: 0x57 * 0x13 = 0xFE in GF(2^8) with 0x11B modulus
    let a = b(0x57);
    let c = b(0x13);
    println!("start mul...");
    let prod = a * c;
    println!("after mul: {prod:?}");
    assert_eq!(prod.to_u8(), 0xFE);

    // 加算は XOR
    println!("start add...");
    let sum = b(0xA5) + b(0x5A);
    println!("after add: {sum:?}");
    assert_eq!(sum.to_u8(), 0xFF);
}

#[test]
fn gf256_inverse_property() {
    for x in [1u8, 2, 3, 5, 7, 11, 13, 29, 127, 191, 223, 251] {
        let a = b(x);
        let inv = a.inv();
        assert_eq!((a * inv), b(1));
    }
}

#[test]
fn poly_division_over_gf256() {
    // (x^2 + 1) / (x + 1) = x + 1, 余り 0  （GF(2) の既知の恒等式）
    // 係数は低次から: 1 + 0*x + 1*x^2
    let f = PolyGF256::new(vec![b(1), b(0), b(1)]);
    let g = PolyGF256::new(vec![b(1), b(1)]); // 1 + x
    let (q, r) = f.div_rem(&g);
    assert_eq!(q.coeffs, vec![b(1), b(1)]); // x + 1
    assert_eq!(r.coeffs, vec![b(0)]); // 0
}
