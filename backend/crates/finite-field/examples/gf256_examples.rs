use finite_field::gf256::{gf256_from_u8, gf256_modulus, GF256};

fn main() {
    let a: GF256 = gf256_from_u8(0x57);
    let b: GF256 = gf256_from_u8(0x13);
    let c = a.clone() * b.clone();
    println!("({a})*({b}) = {c}");

    // 逆元の確認
    let inv = match a.clone().inv() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("inverse does not exist: {e}");
            return;
        }
    };
    println!("a*inv(a) = {}", (a * inv));

    // 基本データ
    let _px = gf256_modulus();
}
