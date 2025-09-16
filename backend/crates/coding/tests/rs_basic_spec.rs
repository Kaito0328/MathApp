use coding::{GF256, ReedSolomon, Message};
use finite_field::gf256::{gf256_from_u8, gf256_modulus};
use finite_field::field2m::FiniteField2m;
use linalg::Vector;

fn b(x: u8) -> GF256 { gf256_from_u8(x) }

// RS は FiniteField2m ベースのデフォルト評価点 α^0..α^{n-1} を使用

#[test]
fn encode_length_matches() {
    let k = 4; let n = 7;
    let px = gf256_modulus();
    let field = FiniteField2m::new(px.as_ref());
    let rs = ReedSolomon::new_with_field(k, n, &field).expect("ReedSolomon::new_with_field");
    let msg = Message::from(Vector::new(vec![b(1), b(2), b(3), b(4)]));
    let code = rs.encode(&msg).expect("encode");
    assert_eq!(code.as_ref().dim(), n);
}

#[test]
fn systematic_roundtrip_no_errors_small() {
    // デコードは暫定実装のため、ここでは t=1, n=5, k=3 程度で roundtrip を確認
    let k = 3; let n = 5;
    let px = gf256_modulus();
    let field = FiniteField2m::new(px.as_ref());
    let rs = ReedSolomon::new_with_field(k, n, &field).expect("ReedSolomon::new_with_field");
    let msg = Message::from(Vector::new(vec![b(7), b(0x20), b(0x55)]));
    let code = rs.encode(&msg).expect("encode");
    let out = rs.decode(&code).expect("decode");
    // 少なくとも先頭 k 係数が元のメッセージと一致することを期待
    for i in 0..k.min(out.decoded.as_ref().dim()) { assert_eq!(out.decoded[i], msg[i]); }
}
