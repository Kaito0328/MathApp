use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn nt_factor_u64(n: u64) -> Vec<u64> {
    number_theory::prime_factorization::factor(n)
}

// BigInt系は stringで受け取り、string配列で返す（d.ts安定）
#[wasm_bindgen]
pub fn nt_factor_bigint_str(n_str: String) -> Vec<String> {
    use num_bigint::BigInt;
    match n_str.parse::<BigInt>() {
        Ok(n) => {
            let v = number_theory::prime_factorization::factor_for_big(&n);
            v.into_iter().map(|x| x.to_string()).collect()
        }
        Err(_) => vec![],
    }
}
