use coding::types::{Codeword, Message};
use coding::{ReedSolomon, GF256};
use finite_field::gf256::gf256_from_u8;
use linalg::Vector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GF(256) の元 α^i を単純化のために 1..=n の u8 から生成
    let n = 8; // 小さめの n でデモ
    let k = 4; // 2t = n-k -> t = 2
    let alphas: Vec<GF256> = (1u8..=n as u8).map(gf256_from_u8).collect();

    let rs = ReedSolomon::<GF256>::new(k, alphas)?;

    // メッセージ f (長さ k)
    let f = Vector::new(vec![
        gf256_from_u8(1),
        gf256_from_u8(2),
        gf256_from_u8(3),
        gf256_from_u8(4),
    ]);
    // Message ラッパーで encode する
    let c = rs.encode(&Message::from(f.clone()))?;
    println!("==== RS encode (GF(256)) ====");
    println!("{rs}");
    println!(
        "n={}, k={}, t={} (<= t errors correctable)",
        rs.n, rs.k, rs.t
    );
    println!("f (msg): {}", fmt_vec_gf256_hex(&f));
    println!("c (code): {}", fmt_vec_gf256_hex(&c));

    // 1) 成功する復号（誤り t=2 以内）
    // 誤り注入のため一旦 Vector を取り出して編集し、Codeword に戻す
    let mut r_ok_vec = c.as_ref().clone();
    // 2 箇所までの誤りを注入
    r_ok_vec.data[1] = r_ok_vec.data[1].clone() + gf256_from_u8(0x55);
    r_ok_vec.data[6] = r_ok_vec.data[6].clone() + gf256_from_u8(0xAA);
    let r_ok: Codeword<GF256> = Codeword::from(r_ok_vec);
    let dec_ok = rs.decode(&r_ok)?;
    println!("\n---- decode (<= t errors) ----");
    println!("r: {}", fmt_vec_gf256_hex(&r_ok));
    println!("f': {}", fmt_vec_gf256_hex(&dec_ok.decoded));
    println!(
        "ok? {}",
        if dec_ok.decoded.as_ref() == &f {
            "true"
        } else {
            "false"
        }
    );

    // 2) 失敗する復号（誤り t を超える）
    let mut r_ng_vec = c.as_ref().clone();
    // 3 箇所の誤りを注入（t=2 を超える）
    r_ng_vec.data[0] = r_ng_vec.data[0].clone() + gf256_from_u8(0x11);
    r_ng_vec.data[2] = r_ng_vec.data[2].clone() + gf256_from_u8(0x22);
    r_ng_vec.data[5] = r_ng_vec.data[5].clone() + gf256_from_u8(0x33);
    let r_ng: Codeword<GF256> = Codeword::from(r_ng_vec);
    let dec_ng = match rs.decode(&r_ng) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("decode failed as expected when errors > t: {e}");
            return Ok(());
        }
    };
    println!("\n---- decode (> t errors) ----");
    println!("r: {}", fmt_vec_gf256_hex(&r_ng));
    println!("f': {}", fmt_vec_gf256_hex(&dec_ng.decoded));
    println!(
        "ok? {} (expected false)",
        if dec_ng.decoded.as_ref() == &f {
            "true"
        } else {
            "false"
        }
    );
    Ok(())
}

fn fmt_vec_gf256_hex(v: &Vector<GF256>) -> String {
    let parts: Vec<String> = v.iter().map(|x| format!("{:02X}", x.to_u8())).collect();
    format!("[{}]", parts.join(" "))
}
