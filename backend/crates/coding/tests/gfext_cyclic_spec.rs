use coding::{CyclicCode, GFExt, GFp};
use std::sync::Arc;

#[test]
fn gfext_basic_inv() {
    type F = GFp<2>;
    // px = x^8 + x^4 + x^3 + x + 1 （AES多項式）
    let px: Arc<Vec<F>> = Arc::new(vec![
        F::new(1),
        F::new(1),
        F::new(0),
        F::new(1),
        F::new(1),
        F::new(0),
        F::new(0),
        F::new(0),
        F::new(1),
    ]);
    let a = GFExt::from_u8(px.clone(), 0x57);
    let b = GFExt::from_u8(px.clone(), 0x13);
    let c = (a.clone() * b.clone()).to_u8();
    assert_eq!(c, 0xFE);
    // 逆元
    let inv = a.clone().inv().expect("inv exists");
    let one = (a * inv).to_u8();
    assert_ne!(one, 0x00);
}

#[test]
fn cyclic_encode_gf2() {
    type F = GFp<2>;
    // n=7, g(x) = 1 + x + x^3 (例)
    let code = CyclicCode::<F>::new(7, vec![F::new(1), F::new(1), F::new(0), F::new(1)]);
    let u = vec![F::new(1), F::new(0), F::new(1), F::new(1)];
    let c = code.encode_poly(&u);
    assert_eq!(c.len(), 7);
}

//符号のテスト
#[test]
fn cyclic_encode_gf2_symbols() {
    type F = GFp<2>;
    // n=7, g(x) = 1 + x + x^3 (例)
    let code = CyclicCode::<F>::new(7, vec![F::new(1), F::new(1), F::new(0), F::new(1)]);
    let u = vec![F::new(1), F::new(0), F::new(1), F::new(1)];
    let c = code.encode_poly(&u);
    assert_eq!(c.len(), 7);
}

#[test]
fn cyclic_encode_gf2_expected_all_ones() {
    type F = GFp<2>;
    // n=7, g(x) = 1 + x + x^3, u = [1,0,1,1] のとき、例の実装では c は全て 1
    let code = CyclicCode::<F>::new(7, vec![F::new(1), F::new(1), F::new(0), F::new(1)]);
    let u = vec![F::new(1), F::new(0), F::new(1), F::new(1)];
    let c = code.encode_poly(&u);
    assert_eq!(c, vec![F::new(1); 7]);
}
