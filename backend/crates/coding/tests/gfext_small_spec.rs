use coding::{GFExt, GFp};
use std::sync::Arc;

// GF(2)[x]/(x^2 + x + 1)
fn px() -> Arc<Vec<GFp<2>>> {
    Arc::new(vec![GFp::<2>::new(1), GFp::<2>::new(1), GFp::<2>::new(1)])
}

#[test]
fn gfext_basic_add_mul_inv_small() {
    // a = 1, b = x, c = 1 + x
    let one = GFExt::from_base(px(), GFp::<2>::new(1));
    let xx = GFExt::new(px(), vec![GFp::<2>::new(0), GFp::<2>::new(1)]);
    let c = GFExt::new(px(), vec![GFp::<2>::new(1), GFp::<2>::new(1)]);

    // 足し算: GF(2) なので 1 + 1 = 0
    assert!((one.clone() + one.clone()).is_zero());
    assert!((xx.clone() + xx.clone()).is_zero());

    // 乗算: (1+x)*(1+x) = 1 + 2x + x^2 = 1 + x^2 ≡ 1 + (x+1) = x
    let prod = c.clone() * c.clone();
    let x_reduced = xx.clone();
    assert_eq!(prod, x_reduced);

    // 逆元: a * a^{-1} = 1
    let inv = c.inv();
    let one2 = c * inv;
    assert!(one2.is_one() || one2 == one);
}
