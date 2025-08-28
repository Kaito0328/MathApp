use coding::types::{Codeword, Message};
use coding::{CyclicCode, GFp};
use linalg::Vector;

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

    let msg: Message<F> = Message::from(Vector::new(u.clone()));
    let c: Codeword<F> = code.encode(&msg).expect("encode");

    println!("==== Cyclic encode (GF(2)) ====");
    println!("{code}"); // uses Display for CyclicCode
    println!("u: {msg}");
    println!("c: {c}");
}
