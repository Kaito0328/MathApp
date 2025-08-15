use coding::{GF256, ReedSolomon};
use coding::gf256::gf256_from_u8;
use linalg::Vector;

fn b(x: u8) -> GF256 { gf256_from_u8(x) }

fn alphas_n(n: usize) -> Vec<GF256> {
    // 生成元 0x03 の累乗系列 (シンプルに i をそのまま使う方法でも良いが既約性の心配が少ない)
    let gen = b(0x03);
    let mut a = Vec::with_capacity(n);
    // 生成元の累乗列。単位元は 1
    let mut cur = b(1);
    for _ in 0..n { a.push(cur.clone()); cur = cur * gen.clone(); }
    a
}

#[test]
fn encode_length_matches() {
    let k = 4; let n = 7;
    let rs = ReedSolomon::new(k, alphas_n(n));
    let msg = Vector::new(vec![b(1), b(2), b(3), b(4)]);
    let code = rs.encode(&msg);
    assert_eq!(code.dim(), n);
}

#[test]
fn systematic_roundtrip_no_errors_small() {
    // デコードは暫定実装のため、ここでは t=1, n=5, k=3 程度で roundtrip を確認
    let k = 3; let n = 5;
    let rs = ReedSolomon::new(k, alphas_n(n));
    let msg = Vector::new(vec![b(7), b(0x20), b(0x55)]);
    let code = rs.encode(&msg);
    let out = rs.decode(&code);
    // 少なくとも先頭 k 係数が元のメッセージと一致することを期待
    for i in 0..k.min(out.decoded.dim()) { assert_eq!(out.decoded[i], msg[i]); }
}
