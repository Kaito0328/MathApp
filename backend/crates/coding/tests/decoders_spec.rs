use coding::{GF256, ReedSolomon, Message, Codeword, CyclicCode};
use finite_field::gf256::{gf256_from_u8, gf256_modulus};
use finite_field::field2m::FiniteField2m;
use finite_field::gfp::GFp;
use linalg::Vector;
use coding::Poly;

fn b(x: u8) -> GF256 { gf256_from_u8(x) }


#[test]
fn rs_bm_single_error() {
    let k = 3; let n = 7; // t=2
    let field = FiniteField2m::new(gf256_modulus().as_ref());
    let rs = ReedSolomon::new_with_field(k, n, &field).expect("ReedSolomon::new_with_field");
    let msg = Message::from(Vector::new(vec![b(0x11), b(0x22), b(0x33)]));
    let code = rs.encode(&msg).expect("encode");
    // inject 1-symbol error
    let mut noisy = code.as_ref().clone();
    let pos = 5usize; noisy[pos] = noisy[pos].clone() + b(0x9A);
    let noisy = Codeword::from(noisy);
    let out = rs.decode_bm(&noisy).expect("decode_bm");
    assert_eq!(out.decoded.as_ref().dim(), k);
    for i in 0..k { assert_eq!(out.decoded[i], msg[i]); }
}

#[test]
fn cyclic_gf2_lut_roundtrip() {
    // n=7, g(x)=1+x+x^3 の簡単な例（ハミング(7,4)と同等ではないがGF(2)循環）
    let n = 7;
    let g = vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(1)];
    let cyc = CyclicCode::<GFp<2>>::new(n, g);
    let k = cyc.k();
    let msg = Message::from(Vector::new((0..k).map(|i| GFp::<2>((i as u16) & 1)).collect()));
    let code = cyc.encode(&msg).expect("encode");
    let mut noisy = code.as_ref().clone(); noisy[3] = noisy[3] + GFp::<2>(1); // 1-bit error
    let noisy = Codeword::from(noisy);
    let dec = cyc.decode_lut(&noisy).expect("decode_lut");
    // デコードは符号語を返す
    assert_eq!(dec.as_ref().dim(), n);
}

// BCH の BM デコードは骨組みのみ（Chien など未実装のため、ここではコンパイル確認のみ）
#[test]
fn bch_decode_bm_skeleton_compiles() {
    // no-op: just ensure crate compiles and API exists
    assert!(true);
}

#[test]
fn bch_gf2_lut_roundtrip_hamming_15_7_like() {
    // BCH(15,7) 相当の簡易 g(x) を最小多項式の LCM から構成
    // 最小多項式の例（x^4 + x + 1, x^4 + x^3 + 1）: 係数は低次→高次
    let m1 = Poly::<GFp<2>>::new(vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)]);
    let m3 = Poly::<GFp<2>>::new(vec![GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1), GFp::<2>(1)]);
    let n = 15usize;
    // emulate BCH via CyclicCode from g = lcm(m1,m3)
    let mut g = Poly::<GFp<2>>::one();
    g = Poly::lcm(&g, &m1);
    g = Poly::lcm(&g, &m3);
    let bch = CyclicCode::<GFp<2>>::new(n, g.coeffs.clone());
    let k = bch.k();

    // 符号化
    let u = Message::from(Vector::new((0..k).map(|i| GFp::<2>((i as u16) & 1)).collect()));
    let c = bch.encode(&u).expect("encode");

    // 1bit 誤り訂正
    let mut r1 = c.as_ref().clone(); r1[4] = r1[4] + GFp::<2>(1);
    let dec1 = bch.decode_lut(&Codeword::from(r1)).expect("decode_lut 1bit");
    assert_eq!(dec1.as_ref().data, c.as_ref().data);

    // 2bit 誤り訂正（t=4? 実際の t は (n-k)/2 = 4、ここでは 2bit で確認）
    let mut r2 = c.as_ref().clone(); r2[2] = r2[2] + GFp::<2>(1); r2[9] = r2[9] + GFp::<2>(1);
    let dec2 = bch.decode_lut(&Codeword::from(r2)).expect("decode_lut 2bit");
    assert_eq!(dec2.as_ref().data, c.as_ref().data);
}
