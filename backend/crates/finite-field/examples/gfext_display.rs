use finite_field::gfext::GFExt;
use finite_field::gfp::GFp;
use std::sync::Arc;

fn main() {
    // GF(p)[x]/(px) の簡単な例: p=5, px = x^2 + 1（実際に既約かは文脈依存の単純例）
    type F = GFp<5>;
    let px = Arc::new(vec![F::new(1), F::new(0), F::new(1)]); // x^2+1
    let a = GFExt::new(px.clone(), vec![F::new(2), F::new(1)]); // 2 + 1*x
    let b = GFExt::new(px.clone(), vec![F::new(3), F::new(4)]); // 3 + 4*x

    // 既定表示（生成元は x）
    println!("a = {}", a.display());
    println!("b = {}", b.display());

    // 生成元記号の変更と上付き指数
    println!(
        "a (alpha) = {}",
        a.display_with("α").unicode_superscript(true)
    );
    println!(
        "b (alpha) = {}",
        b.display_with("α").unicode_superscript(true)
    );

    // 演算と表示
    let s = a.clone() + b.clone();
    let p = a.clone() * b.clone();
    println!("a+b = {}", s.display_with("α").unicode_superscript(true));
    println!("a*b = {}", p.display_with("α").unicode_superscript(true));
}
