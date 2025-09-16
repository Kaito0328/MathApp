use coding::{GF256, ReedSolomon, Message, Codeword};
use finite_field::gf256::{gf256_from_u8, gf256_modulus};
use finite_field::field2m::FiniteField2m;
use linalg::Vector;

fn b(x: u8) -> GF256 { gf256_from_u8(x) }


#[test]
fn rs_roundtrip_no_error_matches_message() {
    let k = 4; let n = 7;
    let field = FiniteField2m::new(gf256_modulus().as_ref());
    let rs = ReedSolomon::new_with_field(k, n, &field).expect("ReedSolomon::new_with_field");
    let msg = Message::from(Vector::new(vec![b(1), b(2), b(3), b(4)]));
    let code = rs.encode(&msg).expect("encode");
    let out = rs.decode(&code).expect("decode");
    assert_eq!(out.decoded.as_ref().dim(), k);
    for i in 0..k { assert_eq!(out.decoded[i], msg[i]); }
}

#[test]
fn rs_single_error_within_t_is_corrected() {
    // n=7, k=3 として t = ceil((n-k)/2) = 2 誤りまで。単一誤りを補正できるかを確認。
    let k = 3; let n = 7;
    let field = FiniteField2m::new(gf256_modulus().as_ref());
    let rs = ReedSolomon::new_with_field(k, n, &field).expect("ReedSolomon::new_with_field");
    let msg = Message::from(Vector::new(vec![b(7), b(0x20), b(0x55)]));
    let code = rs.encode(&msg).expect("encode");
    // 1シンボル誤りを注入
    let pos = 2usize;
    let err = b(0xAB);
    let mut noisy = code.as_ref().clone();
    noisy[pos] = noisy[pos].clone() + err;
    let noisy_code = Codeword::from(noisy);
    let out = rs.decode(&noisy_code).expect("decode");
    // 復号が成功し、元メッセージと一致
    assert_eq!(out.decoded.as_ref().dim(), k);
    for i in 0..k { assert_eq!(out.decoded[i], msg[i]); }
}
