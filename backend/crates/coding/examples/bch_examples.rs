use coding::GFExt;
use coding::GFp;
use coding::{BCHCode, Message, Poly};
use std::sync::Arc;

type GF2 = GFp<2>;
type GF16 = GFExt<GF2>; // GF(2^4) for n=15 BCH

fn main() {
    // 実例: 二元 BCH(15,7, t=2)（狭義）
    // 生成多項式 g(x) = lcm(m1(x), m3(x)) with primitive α in GF(2^4),
    // ここで m1(x) = x^4 + x + 1, m3(x) = x^4 + x^3 + 1（原始多項式 x^4 + x + 1 を使用）
    let n = 15usize;
    // GF(2^4) の構築（原始多項式 x^4 + x + 1）と α
    let px = Arc::new(vec![
        GFp::<2>(1),
        GFp::<2>(1),
        GFp::<2>(0),
        GFp::<2>(0),
        GFp::<2>(1),
    ]);
    let alpha = GF16::new(px.clone(), vec![GFp::<2>(0), GFp::<2>(1)]); // x mod px
                                                                       // 最小多項式（Frobenius 共役で生成）
    let m1 = minimal_polynomial(&alpha, 1, n);
    let m3 = minimal_polynomial(&alpha, 3, n);
    let bch = BCHCode::new_from_minimal_polynomials(n, &[m1.clone(), m3.clone()]);
    let k = bch.k(); // 15 - 8 = 7

    // 情報語 u（長さ k = 7）をエンコードしてコード語 c を得る
    let u_bits = [1u16, 0, 1, 1, 0, 1, 0]; // 末尾0を保持するため直接フィールドに詰める
    let u = Poly {
        coeffs: u_bits.iter().copied().map(GFp::<2>).collect(),
    };
    let msg = Message::from(linalg::Vector::new(u.coeffs.clone()));
    let c = match bch.encode(&msg) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("encode failed: {e}");
            return;
        }
    };

    println!("==== BCH(15,7) encode over GF(2) ====");
    println!("{bch}");
    println!("n={n}, k={k}, g(x) = {}", poly_to_string(bch.g()));
    println!("u: {}", bits_to_string(&u.coeffs));
    let c_vec: Vec<GF2> = c.as_ref().data.clone();
    println!("c: {}", bits_to_string(&c_vec));
    // 検査: c のシンドローム（S1,S3）は 0 になるはず
    let g_at_a1 = eval_poly_gf2_at_gf16(bch.g(), &gf16_pow(&alpha, 1));
    let g_at_a3 = eval_poly_gf2_at_gf16(bch.g(), &gf16_pow(&alpha, 3));
    eprintln!("g(α)={g_at_a1}");
    eprintln!("g(α^3)={g_at_a3}");
    let s1_c = compute_syndrome_at(&c_vec, n, &alpha, 1);
    let s3_c = compute_syndrome_at(&c_vec, n, &alpha, 3);
    eprintln!("check: S1(c)={s1_c}, S3(c)={s3_c}");

    // 最大2ビットまでの誤りを注入して復号（訂正）
    let mut r: Vec<GF2> = c_vec.clone();
    let errs = [2usize, 7usize]; // 2箇所反転
    for &i in &errs {
        r[i] = r[i] + GFp::<2>(1);
    }
    println!(
        "-- inject errors at positions {:?} -> r: {}",
        errs,
        bits_to_string(&r)
    );

    // GF(2^4) の構築（原始多項式 x^4 + x + 1）と α
    let px = Arc::new(vec![
        GFp::<2>(1),
        GFp::<2>(1),
        GFp::<2>(0),
        GFp::<2>(0),
        GFp::<2>(1),
    ]);
    let alpha = GF16::new(px.clone(), vec![GFp::<2>(0), GFp::<2>(1)]); // x mod px

    // 復号（Peterson t<=2 + Chien 検索, 根は α^{-i}）
    let decoded = decode_bch_t2(&r, n, &alpha);
    println!("decoded: {}", bits_to_string(&decoded));
    println!("corrected equals original codeword? {}", decoded == c_vec);
}

// --- helpers ---

fn bits_to_string(v: &[GF2]) -> String {
    let bits: Vec<String> = v
        .iter()
        .map(|x| if x.0 == 0 { "0" } else { "1" }.to_string())
        .collect();
    format!("[{}]", bits.join(" "))
}

fn poly_to_string(p: &Poly<GF2>) -> String {
    // 係数低次→高次を人が読みやすい多項式に
    let mut terms = Vec::new();
    for (i, c) in p.coeffs.iter().enumerate() {
        if c.0 == 0 {
            continue;
        }
        let t = match i {
            0 => "1".to_string(),
            1 => "x".to_string(),
            _ => format!("x^{i}"),
        };
        terms.push(t);
    }
    if terms.is_empty() {
        "0".to_string()
    } else {
        terms.join(" + ")
    }
}

fn eval_poly_gf2_at_gf16(p: &Poly<GF2>, x: &GF16) -> GF16 {
    let mut acc = GF16::from_base(x.px(), GFp::<2>(0));
    for c in p.coeffs.iter().rev() {
        acc = acc * x.clone() + GF16::from_base(x.px(), *c);
    }
    acc
}

// minimal polynomial of α^i over GF(2) for GF(2^4), using Frobenius orbit
fn minimal_polynomial(alpha: &GF16, i: usize, n: usize) -> Poly<GF2> {
    use std::collections::BTreeSet;
    let mut exps = BTreeSet::new();
    let mut e = i % n;
    while !exps.contains(&e) {
        exps.insert(e);
        e = (e * 2) % n;
    }
    let mut poly = vec![GF16::from_base(alpha.px(), GFp::<2>(1))]; // 1
    for &e in &exps {
        let root = gf16_pow(alpha, e);
        let term0 = GF16::from_base(alpha.px(), GFp::<2>(0)) - root.clone();
        let term1 = GF16::from_base(alpha.px(), GFp::<2>(1));
        // multiply poly by (term0 + term1*x)
        let mut v = vec![GF16::from_base(alpha.px(), GFp::<2>(0)); poly.len() + 1];
        for a in 0..poly.len() {
            v[a] = v[a].clone() + poly[a].clone() * term0.clone();
            v[a + 1] = v[a + 1].clone() + poly[a].clone() * term1.clone();
        }
        poly = v;
    }
    let coeffs: Vec<GF2> = poly
        .into_iter()
        .map(|c| {
            if c.is_zero() {
                GFp::<2>(0)
            } else {
                assert!(c.is_one());
                GFp::<2>(1)
            }
        })
        .collect();
    Poly::new(coeffs)
}

// α^i の最小多項式（GF(2)係数）を計算
// BCH(15,7), t<=2 向けの簡易復号（Peterson + Chien）
fn decode_bch_t2(r: &[GF2], n: usize, alpha: &GF16) -> Vec<GF2> {
    // S1, S3 を計算（狭義・奇数べき）
    let s1 = compute_syndrome_at(r, n, alpha, 1);
    let s3 = compute_syndrome_at(r, n, alpha, 3);
    eprintln!("S1={s1}, S3={s3}");
    if s1.is_zero() && s3.is_zero() {
        return r.to_vec();
    }
    // 1誤りの場合: S3 == S1^3 かつ S1 != 0
    if !s1.is_zero() && s3 == gf16_pow(&s1, 3) {
        // 位置 i は S1 = α^{i}
        if let Some(i) = find_log(alpha, &s1, n) {
            let mut c = r.to_vec();
            c[i] = c[i] + GFp::<2>(1);
            return c;
        }
    }
    // 2誤り: σ1 = S1, σ2 = (S3 + S1^3)/S1
    let s1_cu = gf16_pow(&s1, 3);
    let sigma1 = s1.clone();
    let sigma2 = (s3.clone() + s1_cu.clone()) / s1.clone();
    eprintln!("sigma1={sigma1}, sigma2={sigma2}");
    let lambda = vec![GF16::from_base(alpha.px(), GFp::<2>(1)), sigma1, sigma2]; // Λ(z) = 1 + σ1 z + σ2 z^2
                                                                                 // Chien 検索（根は α^{-i} = α^{n-i}）
    let mut err_pos = Vec::new();
    for i in 0..n {
        let x = gf16_pow(alpha, (n - i) % n);
        if poly_eval(&lambda, &x).is_zero() {
            err_pos.push(i);
        }
    }
    eprintln!("err_pos={err_pos:?}");
    let mut c = r.to_vec();
    for i in err_pos {
        c[i] = c[i] + GFp::<2>(1);
    }
    c
}

fn compute_syndrome_at(r: &[GF2], n: usize, alpha: &GF16, k: usize) -> GF16 {
    let ak = gf16_pow(alpha, k % n);
    let mut sum = GF16::from_base(alpha.px(), GFp::<2>(0));
    let mut x = GF16::from_base(alpha.px(), GFp::<2>(1));
    for &bit in r.iter().take(n) {
        if bit.0 == 1 {
            sum = sum + x.clone();
        }
        x = x * ak.clone();
    }
    sum
}

fn find_log(alpha: &GF16, x: &GF16, n: usize) -> Option<usize> {
    // find i in [0,n) such that α^i == x
    let mut p = GF16::from_base(alpha.px(), GFp::<2>(1));
    for i in 0..n {
        if &p == x {
            return Some(i);
        }
        p = p * alpha.clone();
    }
    None
}

fn gf16_pow(a: &GF16, e: usize) -> GF16 {
    if e == 0 {
        return GF16::from_base(a.px(), GFp::<2>(1));
    }
    let mut acc = GF16::from_base(a.px(), GFp::<2>(1));
    let mut base = a.clone();
    let mut exp = e;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = acc * base.clone();
        }
        base = base.clone() * base;
        exp >>= 1;
    }
    acc
}

fn poly_eval(c: &[GF16], x: &GF16) -> GF16 {
    let mut acc = GF16::from_base(x.px(), GFp::<2>(0));
    for coeff in c.iter().rev() {
        acc = acc * x.clone() + coeff.clone();
    }
    acc
}

// (unused helpers removed)
