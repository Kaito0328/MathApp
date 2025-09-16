use crate::error::{CodingError, Result as CodingResult};
use crate::types::{Codeword, Message};
use crate::Poly;
use crate::codec_common::{systematic_cyclic_encode_full, bch_generator_poly};
// no serde for BCHCode since FiniteField2m isn't serializable
use std::fmt;
use crate::types::{GeneratorMatrix, ParityCheckMatrix};
use crate::code_utils::parity_check_from_generator as g2h;
use crate::code_utils::syndrome_decode_gf2;
use finite_field::gfp::GFp;
use finite_field::field2m::FiniteField2m;
use finite_field::primitive::known_primitive_poly_gf2;
use crate::codec_common::{bch_compute_syndromes_binary, rs_chien_positions_over_indices};

// 最小限の BCH( n, k ) 構築: 基底体 F 上の拡大体 GF(q^m) の原始 n 次元の α を用い
// t 個の連続べき最小多項式の least common multiple を g(x) とし、CyclicCode 相当で符号化
#[derive(Debug, Clone)]
pub struct BCHCode {
    pub field: FiniteField2m,
    pub n: usize,
    pub k: usize,
    pub t: usize,
    pub g: Poly<GFp<2>>,
}

impl BCHCode {
    /// Construct binary BCH from field and parameters (n, t), narrow-sense (b=1).
    pub fn new_with_field(field: FiniteField2m, n: usize, t: usize) -> Self {
        assert_eq!(n, field.n, "n must equal 2^m-1 of the field");
        let g = bch_generator_poly(&field, n, t, 1);
        let k = n - (g.coeffs.len() - 1);
        Self { field, n, k, t, g }
    }

    /// Backward-compat: construct with explicit b (root start). Prefer `new_with_field` if b=1.
    pub fn new_with_field_b(field: FiniteField2m, n: usize, t: usize, b: usize) -> Self {
        assert_eq!(n, field.n, "n must equal 2^m-1 of the field");
        let g = bch_generator_poly(&field, n, t, b);
        let k = n - (g.coeffs.len() - 1);
        Self { field, n, k, t, g }
    }

    /// Construct BCH by degree m of GF(2^m) (n = 2^m - 1), choosing a known primitive polynomial automatically.
    pub fn new_auto(m: usize, t: usize) -> Self {
        assert!(m >= 2 && m <= 15, "unsupported m: {m}");
        let n = (1usize << m) - 1;
        let px = known_primitive_poly_gf2(m);
        let field = FiniteField2m::new(&px);
        Self::new_with_field(field, n, t)
    }

    pub fn g(&self) -> &Poly<GFp<2>> { &self.g }

    pub fn k(&self) -> usize { self.k }

    // Message -> Codeword（x^n≡1で縮約）
    pub fn encode(&self, u: &Message<GFp<2>>) -> CodingResult<Codeword<GFp<2>>> {
        if u.dim() != self.k {
            return Err(CodingError::InvalidParameters {
                text: format!("message length {} must be k {}", u.dim(), self.k),
            });
        }
        systematic_cyclic_encode_full(self.n, self.k, &self.g, u)
    }
}


impl fmt::Display for BCHCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 試しに f64 専用の Display は polynomial 側にあるが、ここは一般 F を前提に簡易表記
        // g(x) の表示は係数から直接組み立て
        let mut terms: Vec<String> = Vec::new();
        for (i, c) in self.g.coeffs.iter().enumerate() {
            if *c == GFp::<2>(0) {
                continue;
            }
            let term = if i == 0 {
                format!("{c}")
            } else if *c == GFp::<2>(1) {
                if i == 1 {
                    "x".to_string()
                } else {
                    format!("x^{i}")
                }
            } else if i == 1 {
                format!("{c}x")
            } else {
                format!("{c}x^{i}")
            };
            terms.push(term);
        }
        let gstr = if terms.is_empty() {
            "0".to_string()
        } else {
            terms.join(" + ")
        };
        write!(
            f,
            "BCH(n={}, k={}, t={}, g(x)={})",
            self.n,
            self.k,
            self.t,
            gstr
        )
    }
}

// GF(2) 向け: 生成多項式 g(x) を用いた循環符号としてのシンドロームLUT復号
impl BCHCode {
    /// GF(2) 専用の有界距離（重み <= floor((n-k)/2)）シンドロームLUT復号
    /// 入力は符号長 n の受信語 r（GF(2) ベクトル）。
    pub fn decode_lut(&self, r: &crate::types::Codeword<GFp<2>>) -> crate::error::Result<crate::types::Codeword<GFp<2>>> {
        let k = self.k;
        let n = self.n;
        // 巡回符号の標準的な生成行列: 各行は x^i * g(x) を x^n-1 で縮約した長さ n のベクトル
        let mut gmat = linalg::Matrix::new(k, n, vec![GFp::<2>(0); k * n])
            .map_err(|e| crate::error::CodingError::InvalidParameters { text: format!("matrix new failed: {e}") })?;
        let gcoefs = &self.g.coeffs; // 低次→高次
        for i in 0..k {
            for j in 0..gcoefs.len() {
                if gcoefs[j] != GFp::<2>(0) {
                    let col = (i + j) % n; // x^n ≡ 1 の巡回縮約
                    gmat[(i, col)] = gmat[(i, col)] + GFp::<2>(1);
                }
            }
        }
        let g = GeneratorMatrix(gmat);
        let h: ParityCheckMatrix<GFp<2>> = g2h(&g)?;
        // t は一般に (n-k)/2 が上限。ここでは保守的に rpar/2 を用いる。
        let rpar = n - k;
        syndrome_decode_gf2(&h, r, rpar / 2)
    }

    /// BM + Chien（狭義 BCH 前提: 連続根 α^b..α^{b+2t-1}）。GF(2) なので誤り値は 1。
    pub fn decode_bm(&self, r: &crate::types::Codeword<GFp<2>>, b: usize) -> crate::error::Result<crate::types::Codeword<GFp<2>>> {
        let n = self.n;
        if r.dim() != n { return Err(crate::error::CodingError::InvalidParameters { text: format!("received length {} must equal n {}", r.dim(), n) }); }
        // 2t syndromes over GF(2^m) as GFExt elements
        let synd = bch_compute_syndromes_binary(&self.field, &r.as_ref().data, n, self.t, b);
        if synd.iter().all(|x| x.is_zero()) { return Ok(r.clone()); }
        // BM over GF(2^m) in generic form (GFExt)
        let sigma_coeffs = crate::lfsr::berlekamp_massey(&synd);
        let sigma: Poly<finite_field::gfext::GFExt<GFp<2>>> = Poly::new(sigma_coeffs);
        // Chien search over full indices [0..n)
        let indices_full: Vec<usize> = (0..n).collect();
        let positions = rs_chien_positions_over_indices(&self.field, &sigma, &indices_full);
        if positions.len() > self.t { return Err(crate::error::CodingError::DecodeFailure { text: "too many errors".into() }); }
        // Flip bits
        let mut out = r.as_ref().clone();
        for i in positions { out[i] = if out[i] == GFp::<2>(0) { GFp::<2>(1) } else { GFp::<2>(0) }; }
        Ok(crate::types::Codeword::from(out))
    }
}
