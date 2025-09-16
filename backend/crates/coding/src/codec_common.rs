use crate::error::{CodingError, Result as CodingResult};
use crate::types::{Codeword, Message};
use crate::Poly;
use finite_field::field2m::FiniteField2m;
use finite_field::gfext::GFExt;
use finite_field::gfp::GFp;
use linalg::{Field, Vector};
use num_traits::{One, Zero};

// Systematic cyclic encode for full-length n
pub fn systematic_cyclic_encode_full<F>(n: usize, k: usize, g: &Poly<F>, u: &Message<F>) -> CodingResult<Codeword<F>>
where
    F: Field + Clone + PartialEq + Zero + One,
{
    if u.dim() != k {
        return Err(CodingError::InvalidParameters { text: format!("message length {} must equal k {}", u.dim(), k) });
    }
    let r = n - k;
    let mut tmp: Vec<F> = vec![F::zero(); n];
    for i in 0..k { tmp[i + r] = u[i].clone(); }
    let tmp_poly: Poly<F> = Poly::new(tmp.clone());
    let (_q, mut rem) = tmp_poly.div_rem(g);
    if rem.coeffs.len() < r { rem.coeffs.resize(r, F::zero()); }
    let mut c = tmp;
    for i in 0..r { c[i] = c[i].clone() + rem.coeffs[i].clone(); }
    Ok(Codeword::from(Vector::new(c)))
}

// Systematic encode via full-length N then shorten to n_short = r_full + k_short
pub fn systematic_cyclic_encode_shortened_over_full<F>(n_full: usize, n_short: usize, k_short: usize, g: &Poly<F>, u: &Message<F>) -> CodingResult<Codeword<F>>
where
    F: Field + Clone + PartialEq + Zero + One,
{
    if u.dim() != k_short {
        return Err(CodingError::InvalidParameters { text: format!("message length {} must equal k {}", u.dim(), k_short) });
    }
    let s = n_full - n_short; // shortening amount
    let k_full = k_short + s;
    let r_full = n_full - k_full; // equals n_short - k_short

    // full-length message: [0;s) zeros + u
    let mut m_full: Vec<F> = vec![F::zero(); k_full];
    for i in 0..k_short { m_full[s + i] = u[i].clone(); }

    // tmp_full = x^{r_full} * m_full
    let mut tmp_full: Vec<F> = vec![F::zero(); n_full];
    for i in 0..k_full { tmp_full[i + r_full] = m_full[i].clone(); }
    let tmp_poly_full: Poly<F> = Poly::new(tmp_full.clone());
    let (_q, mut rem_full) = tmp_poly_full.div_rem(g);
    if rem_full.coeffs.len() < r_full { rem_full.coeffs.resize(r_full, F::zero()); }
    let mut c_full = tmp_full;
    for i in 0..r_full { c_full[i] = c_full[i].clone() + rem_full.coeffs[i].clone(); }

    // shorten: parity [0..r_full) + message tail [N-k_short..N)
    let mut c_short: Vec<F> = Vec::with_capacity(n_short);
    c_short.extend_from_slice(&c_full[0..r_full]);
    c_short.extend_from_slice(&c_full[n_full - k_short..n_full]);
    Ok(Codeword::from(Vector::new(c_short)))
}

// Compute RS syndromes S_j over selected full-length indices
pub fn rs_compute_syndromes_over_indices(
    field: &FiniteField2m,
    r: &[GFExt<GFp<2>>],
    indices_full: &[usize],
    two_t: usize,
) -> Vec<GFExt<GFp<2>>> {
    let mut synd: Vec<GFExt<GFp<2>>> = vec![GFExt::<GFp<2>>::zero(); two_t];
    for j in 0..two_t {
        let aj = field.u16_to_gfext(field.alpha_pow(j + 1)); // α^{j+1}
        let mut acc = GFExt::<GFp<2>>::zero();
        for (i_local, &i_full) in indices_full.iter().enumerate() {
            let mut pow = GFExt::<GFp<2>>::one();
            for _ in 0..i_full { pow = pow * aj.clone(); }
            acc = acc + r[i_local].clone() * pow;
        }
        synd[j] = acc;
    }
    synd
}

// Robust Chien search over selected full-length indices; returns local positions where sigma root matches
pub fn rs_chien_positions_over_indices(
    field: &FiniteField2m,
    sigma: &Poly<GFExt<GFp<2>>>,
    indices_full: &[usize],
) -> Vec<usize> {
    let n_full = field.n;
    let deg_sigma = sigma.deg() as usize;
    let mut error_pos: Vec<usize> = Vec::new();
    for (i_local, &i_full) in indices_full.iter().enumerate() {
        let a = field.u16_to_gfext(field.alpha_pow(i_full % n_full));
        let a_inv = field.u16_to_gfext(field.alpha_pow((n_full - (i_full % n_full)) % n_full));
        let cond_inv = sigma.eval(a_inv.clone()) == GFExt::<GFp<2>>::zero();
        let cond_rev = {
            let mut acc = GFExt::<GFp<2>>::zero();
            for (i, coef) in sigma.coeffs.iter().enumerate() {
                let pwr = deg_sigma.saturating_sub(i);
                let mut pow = GFExt::<GFp<2>>::one();
                for _ in 0..pwr { pow = pow * a.clone(); }
                acc = acc + coef.clone() * pow;
            }
            acc == GFExt::<GFp<2>>::zero()
        };
        if cond_inv || cond_rev { error_pos.push(i_local); }
    }
    error_pos
}

// Forney error values for RS at given full-length positions mapped to xinv = α^{-i_full}
pub fn rs_forney_values(
    field: &FiniteField2m,
    omega: &Poly<GFExt<GFp<2>>>,
    dsigma: &Poly<GFExt<GFp<2>>>,
    positions_full: &[usize],
) -> CodingResult<Vec<GFExt<GFp<2>>>> {
    let n_full = field.n;
    let mut out = Vec::with_capacity(positions_full.len());
    for &i_full in positions_full {
        let xinv = field.u16_to_gfext(field.alpha_pow((n_full - (i_full % n_full)) % n_full));
        let num = omega.eval(xinv.clone());
        let den = dsigma.eval(xinv.clone());
        if den == GFExt::<GFp<2>>::zero() {
            return Err(CodingError::DecodeFailure { text: "singular derivative in Forney".into() });
        }
        out.push(num / den);
    }
    Ok(out)
}

// BCH（GF(2)）のシンドローム（GF(2^m)上）: S_j = sum r_i * (α^{b+j})^i
pub fn bch_compute_syndromes_binary(
    field: &FiniteField2m,
    r: &[GFp<2>],
    n: usize,
    t: usize,
    b: usize,
) -> Vec<GFExt<GFp<2>>> {
    let two_t = 2 * t;
    let mut synd: Vec<GFExt<GFp<2>>> = vec![GFExt::<GFp<2>>::zero(); two_t];
    for j in 0..two_t {
        let aj = field.u16_to_gfext(field.alpha_pow(b + j));
        let mut acc = GFExt::<GFp<2>>::zero();
        for i in 0..n {
            if r[i].0 == 0 { continue; }
            let mut pow = GFExt::<GFp<2>>::one();
            for _ in 0..i { pow = pow * aj.clone(); }
            acc = acc + pow;
        }
        synd[j] = acc;
    }
    synd
}

// Build BCH generator polynomial g(x) from consecutive roots α^b..α^{b+2t-1} over GF(2)
pub fn bch_generator_poly(field: &FiniteField2m, n: usize, t: usize, b: usize) -> Poly<GFp<2>> {
    assert_eq!(n, field.n, "For narrow-sense primitive BCH, n should be 2^m-1");
    let mut seen = vec![false; field.n];
    let mut polys: Vec<Poly<GFp<2>>> = Vec::new();
    for j in 0..(2 * t) {
        let e = (b + j) % field.n;
        if seen[e] { continue; }
        let coset = field.cyclotomic_coset(e);
        for &u in &coset { seen[u] = true; }
        polys.push(field.minimal_polynomial_over_gf2(e));
    }
    // g = lcm of polys; over GF(2) and disjoint cosets => pairwise coprime, multiply
    let mut g: Poly<GFp<2>> = Poly::one();
    for p in polys { g = &g * &p; }
    g
}
