use coding::{CyclicCode, GFp};

fn main() {
    // GF(2) 上の単純な循環符号例: n=7, g(x)=1 + x + x^3 （例示）
    type F = GFp<2>;
    let n = 7usize;
    let g = vec![F::new(1), F::new(1), F::new(0), F::new(1)]; // 1 + x + x^3
    let code = CyclicCode::<F>::new(n, g);

    // 情報語 u (長さ k = n - (deg(g)))
    let k = code.k();
    let u = vec![F::new(1), F::new(0), F::new(1), F::new(1)];
    assert_eq!(u.len(), k);

    let c = code.encode_poly(&u);
    println!("==== Cyclic encode (GF(2)) ====");
    println!("n={n}, k={k}, g(x)=1+x+x^3");
    println!("u: {u:?}");
    println!("c: {c:?}");
}
