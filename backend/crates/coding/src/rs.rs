use crate::error::{CodingError, Result as CodingResult};
use crate::types::{Codeword, Message};
use crate::Poly;
use crate::codec_common::{
    systematic_cyclic_encode_shortened_over_full,
    rs_compute_syndromes_over_indices,
    rs_chien_positions_over_indices,
    rs_forney_values,
};
use linalg::{Field, Vector};
use std::{fmt, vec};
use num_traits::{One, Zero};
use crate::lfsr::berlekamp_massey;
use finite_field::field2m::FiniteField2m;
use finite_field::gfext::GFExt;
use finite_field::gfp::GFp;

#[derive(Debug, Clone)]
pub struct RSDecodeResult<F: Field + Clone> { pub decoded: Message<F> }

// RS over GF(2^m): field-owning, cyclic form with generator polynomial g(x) = ∏_{i=1..n-k} (x - α^i)
#[derive(Debug, Clone)]
pub struct ReedSolomon {
    pub field: FiniteField2m,
    pub n: usize,
    pub k: usize,
    pub t: usize,
    pub g: Poly<GFExt<GFp<2>>>,
}

impl ReedSolomon {
    pub fn encode(&self, f: &Message<GFExt<GFp<2>>>) -> CodingResult<Codeword<GFExt<GFp<2>>>> {
        if f.dim() != self.k {
            return Err(CodingError::InvalidParameters { text: format!("message length {} must equal k {}", f.dim(), self.k) });
        }
        let n_full = self.field.n;
        systematic_cyclic_encode_shortened_over_full(n_full, self.n, self.k, &self.g, f)
    }

    // 系統符号化に合わせ、誤りなしのときは末尾 k シンボルをそのまま返す
    // 誤りありの場合は、標準的な BM + Chien + Forney で訂正し、末尾 k を返す
    pub fn decode(&self, r: &Codeword<GFExt<GFp<2>>>) -> CodingResult<RSDecodeResult<GFExt<GFp<2>>>> {
        if r.dim() != self.n {
            return Err(CodingError::InvalidParameters { text: format!("received length {} must equal n {}", r.dim(), self.n) });
        }
        let rvec = r.as_ref();
        let two_t = 2 * self.t;
        let n_full = self.field.n; // N
        let r_full = self.n - self.k; // parity length in short code (equals full parity)
        // Build mapped full indices for shortened RS: parity [0..r_full) and message tail [N-k..N)
        let mut indices_full: Vec<usize> = Vec::with_capacity(self.n);
        for i_local in 0..r_full { indices_full.push(i_local); }
        for i_local in r_full..self.n { indices_full.push(n_full - self.k + (i_local - r_full)); }
        // Prepare slice of received symbols
        let r_slice: Vec<GFExt<GFp<2>>> = (0..self.n).map(|i| rvec[i].clone()).collect();
        let synd = rs_compute_syndromes_over_indices(&self.field, &r_slice, &indices_full, two_t);
        if synd.iter().all(|x| *x == GFExt::<GFp<2>>::zero()) {
            // 誤りなし: 系統化により、末尾 k がメッセージ
            let parity = self.n - self.k;
            let msg = Vector::new((0..self.k).map(|i| rvec[parity + i].clone()).collect());
            return Ok(RSDecodeResult { decoded: Message::from(msg) });
        }
        // BM to get locator Σ(x)
        let sigma_coeffs = berlekamp_massey(&synd);
        let sigma: Poly<GFExt<GFp<2>>> = Poly::new(sigma_coeffs.clone());
        // error evaluator Ω(x) = [S(x) Σ(x)] mod x^{2t}
        let omega: Poly<GFExt<GFp<2>>> = crate::lfsr::error_evaluator_from_syndromes(&synd, &sigma, two_t);
        let dsigma = crate::lfsr::formal_derivative(&sigma);
        // Chien over mapped indices
        let error_pos: Vec<usize> = rs_chien_positions_over_indices(&self.field, &sigma, &indices_full);
        let l = error_pos.len();
        if l == 0 { // unable to find errors though synd!=0 -> treat as failure
            return Err(CodingError::DecodeFailure { text: "no error locations found (nonzero syndromes)".into() });
        }
        if l > self.t { return Err(CodingError::DecodeFailure { text: "too many errors".into() }); }
        // Forney: e_i = -Ω(X_i^{-1}) / Σ'(X_i^{-1}); char-2 なので '-' は不要
        let mut corrected = rvec.clone();
        let positions_full: Vec<usize> = error_pos.iter().map(|&pos| if pos < r_full { pos } else { n_full - self.k + (pos - r_full) }).collect();
        let values = rs_forney_values(&self.field, &omega, &dsigma, &positions_full)?;
        for (idx, &pos) in error_pos.iter().enumerate() { corrected[pos] = corrected[pos].clone() + values[idx].clone(); }
        // extract systematic message
        let parity = self.n - self.k;
        let msg = Vector::new((0..self.k).map(|i| corrected[parity + i].clone()).collect());
        Ok(RSDecodeResult { decoded: Message::from(msg) })
    }
}

impl fmt::Display for ReedSolomon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RS(n={}, k={}, t={}, deg(g)={})", self.n, self.k, self.t, self.g.deg())
    }
}

impl ReedSolomon {
    // BM 版も decode に委譲（内部はBM+Chien+Forneyで訂正し系統メッセージを返す）
    pub fn decode_bm(&self, r: &Codeword<GFExt<GFp<2>>>) -> CodingResult<RSDecodeResult<GFExt<GFp<2>>>> {
        self.decode(r)
    }
}
impl ReedSolomon {
    /// Construct RS over GF(2^m) with generator polynomial g(x)=∏_{i=1..n-k}(x-α^i)
    /// Constraints: n <= 2^m - 1
    pub fn new_with_field(k: usize, n: usize, field: &FiniteField2m) -> CodingResult<Self> {
        if n == 0 || k == 0 || k > n { return Err(CodingError::InvalidParameters { text: format!("invalid (n,k)=({n},{k})") }); }
        if n > field.n { return Err(CodingError::InvalidParameters { text: format!("n must be <= 2^m-1 = {}", field.n) }); }
        let deg = n - k; // designed distance d = n-k+1
        let mut g: Poly<GFExt<GFp<2>>> = Poly::one();
        for i in 1..=deg {
            let ai = field.u16_to_gfext(field.alpha_pow(i));
            let lin = Poly::new(vec![GFExt::<GFp<2>>::zero() - ai, GFExt::<GFp<2>>::one()]); // (x - α^i)
            g = &g * &lin;
        }
        let t = (n - k + 1 - 1) / 2; // floor((d-1)/2) = floor((n-k)/2)
        Ok(Self { field: field.clone(), n, k, t, g })
    }

    /// Construct RS by auto-selecting primitive polynomial for smallest m with 2^m-1 >= n (2<=m<=15)
    pub fn new_auto(k: usize, n: usize) -> CodingResult<Self> {
        if n == 0 || k == 0 || k > n { return Err(CodingError::InvalidParameters { text: format!("invalid (n,k)=({n},{k})") }); }
        let mut m: usize = 1;
        loop {
            let q_minus_1 = (1usize << m) - 1;
            if q_minus_1 >= n { break; }
            m += 1;
            if m > 15 { return Err(CodingError::InvalidParameters { text: format!("unsupported n={}, requires m>15 (table limited)", n) }); }
        }
        if m < 2 { return Err(CodingError::InvalidParameters { text: "m must be >= 2".into() }); }
        let field = FiniteField2m::new_auto(m);
        Self::new_with_field(k, n, &field)
    }
}

