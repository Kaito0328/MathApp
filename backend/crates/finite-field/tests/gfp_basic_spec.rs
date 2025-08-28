use finite_field::gfp::GFp;

#[test]
fn gfp_add_sub_mul_div_mod5() {
    type F = GFp<5>;
    let a = F::new(7); // 2 mod 5
    let b = F::new(9); // 4 mod 5
    assert_eq!((a + b).value(), 1); // 2+4=6≡1
    assert_eq!((a - b).value(), 3); // 2-4=-2≡3
    assert_eq!((a * b).value(), 3); // 2*4=8≡3
    assert_eq!((b / a).value(), 2); // 4 / 2 = 2
}

#[test]
fn gfp_inverse_property_small_prime() {
    type F = GFp<7>;
    for x in 1..7u16 {
        let a = F::new(x as i64);
        let one = F::new(1);
    assert_eq!((a * a.inv().expect("inv exists")), one);
    }
}
