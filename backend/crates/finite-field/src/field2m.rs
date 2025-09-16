use crate::gfp::GFp;
use crate::primitive::known_primitive_poly_gf2;
use crate::gfext::GFExt;
use std::sync::Arc;
use poly::Polynomial as Poly;

/// Finite field manager for GF(2^m) with a given primitive (irreducible) polynomial px(x)
/// Coefficients of px are given as bits over GF(2): px(x) = sum px[i] x^i, px[m]=1, px[0]=1 typically.
/// Elements are represented as u16 bit patterns of polynomial basis (low->high), so m must be <= 15.
/// This is chosen for performance and WASM friendliness; can be generalized to u32 if needed.
///
/// Example (GF(2^4), px = x^4 + x + 1):
/// let px = [1,1,0,0,1].map(GFp::<2>); // low->high
/// let field = FiniteField2m::new(&px);
/// let a = field.alpha_pow(1); // α
/// let b = field.alpha_pow(3); // α^3
/// let c = field.mul(a, b);    // α^4 = α + 1 (mod px)
#[derive(Clone, Debug)]
pub struct FiniteField2m {
    pub m: usize,
    pub n: usize,        // n = 2^m - 1
    pub px_bits: u32,    // primitive polynomial bits (low->high), degree m
    exp: Vec<u16>,       // size: 2^m - 1 (or doubled internally)
    log: Vec<i32>,       // size: 2^m (log[0] = -1)
}

impl FiniteField2m {
    /// Convenience: construct GF(2^m) using a known primitive polynomial table (2 <= m <= 15)
    pub fn new_auto(m: usize) -> Self {
        assert!(m >= 2 && m <= 15, "unsupported m: {m}");
        let px = known_primitive_poly_gf2(m);
        Self::new(&px)
    }

    /// Create GF(2^m) with modulus given as coefficients over GF(2) (low->high), length m+1.
    /// Example: for AES GF(2^8), px = [1,1,0,1,1,0,0,0,1] for x^8 + x^4 + x^3 + x + 1.
    pub fn new(px: &[GFp<2>]) -> Self {
        assert!(!px.is_empty(), "px empty");
        let m = px.len() - 1;
        assert!(m <= 15, "m too large for u16 element: {}", m);
        assert!(px[m].0 == 1, "leading coeff must be 1");
        assert!(px[0].0 == 1, "constant coeff should be 1 for primitive polynomials (usual)");
        // pack bits
        let mut bits: u32 = 0;
        for (i, c) in px.iter().enumerate() { if c.0 != 0 { bits |= 1u32 << i; } }
        let (exp, log) = build_tables(m, bits);
        Self { m, n: (1usize << m) - 1, px_bits: bits, exp, log }
    }

    #[inline]
    pub fn add(&self, a: u16, b: u16) -> u16 { a ^ b }

    #[inline]
    pub fn zero(&self) -> u16 { 0 }

    #[inline]
    pub fn one(&self) -> u16 { 1 }

    #[inline]
    pub fn is_zero(&self, a: u16) -> bool { a == 0 }

    #[inline]
    pub fn mul(&self, a: u16, b: u16) -> u16 {
        if a == 0 || b == 0 { return 0; }
        let la = self.log[a as usize] as i32;
        let lb = self.log[b as usize] as i32;
        let idx = (la + lb) % (self.n as i32);
        self.exp[idx as usize]
    }

    #[inline]
    pub fn inv(&self, a: u16) -> u16 {
        assert!(a != 0, "inv(0)");
        let la = self.log[a as usize] as i32;
        let idx = ((self.n as i32) - la) % (self.n as i32);
        self.exp[idx as usize]
    }

    #[inline]
    pub fn pow(&self, a: u16, e: i64) -> u16 {
        if a == 0 { return 0; }
        let la = self.log[a as usize] as i64;
        let n = self.n as i64;
        let mut idx = (la * (e.rem_euclid(n))) % n;
        if idx < 0 { idx += n; }
        self.exp[idx as usize]
    }

    #[inline]
    pub fn alpha_pow(&self, i: usize) -> u16 { self.exp[i % self.n] }

    /// Return primitive polynomial coefficients as GFp<2> vector (low->high), length m+1
    pub fn px_gf2_coeffs(&self) -> Vec<GFp<2>> {
        let mut v = Vec::with_capacity(self.m + 1);
        for i in 0..=self.m { let bit = ((self.px_bits >> i) & 1) as u16; v.push(GFp::<2>(bit)); }
        v
    }

    /// Get Arc of px coefficients for constructing GFExt elements
    pub fn px_arc(&self) -> Arc<Vec<GFp<2>>> { Arc::new(self.px_gf2_coeffs()) }

    /// Convert a u16 polynomial-basis element to GFExt<GFp<2>> with current px
    pub fn u16_to_gfext(&self, x: u16) -> GFExt<GFp<2>> {
        let mut coeffs: Vec<GFp<2>> = Vec::with_capacity(self.m);
        for i in 0..self.m {
            let bit = ((x >> i) & 1) as u16;
            coeffs.push(GFp::<2>(bit));
        }
        if coeffs.is_empty() { coeffs.push(GFp::<2>(0)); }
        GFExt::new(self.px_arc(), coeffs)
    }

    /// Compute cyclotomic coset modulo n under multiplication by 2 (Frobenius for GF(2^m)).
    pub fn cyclotomic_coset(&self, start: usize) -> Vec<usize> {
        let mut v = Vec::new();
        let mut seen = vec![false; self.n];
        let mut x = start % self.n;
        while !seen[x] {
            seen[x] = true;
            v.push(x);
            x = (2 * x) % self.n;
        }
        v
    }

    /// Minimal polynomial of alpha^i over GF(2) via Frobenius orbit product ∏ (X - α^{i·2^k}).
    pub fn minimal_polynomial_over_gf2(&self, i: usize) -> Poly<GFp<2>> {
        let coset = self.cyclotomic_coset(i % self.n);
        // polynomial over GF(2^m) with u16 coefficients (low->high)
        let mut coeffs: Vec<u16> = vec![1];
        for &e in &coset {
            let r = self.alpha_pow(e);
            // multiply current by (X + r)
            let d = coeffs.len() - 1;
            let mut next = vec![0u16; coeffs.len() + 1];
            // next[0] = r * c0
            next[0] = self.mul(coeffs[0], r);
            // inner terms
            for k in 1..=d {
                next[k] = self.add(coeffs[k - 1], self.mul(coeffs[k], r));
            }
            // highest term
            next[d + 1] = coeffs[d];
            coeffs = next;
        }
        // Map to GF(2) coeffs (must be 0 or 1 in GF(2^m) representation)
        let mut out: Vec<GFp<2>> = Vec::with_capacity(coeffs.len());
        for c in coeffs {
            match c {
                0 => out.push(GFp::<2>(0)),
                1 => out.push(GFp::<2>(1)),
                _ => panic!("minimal polynomial has non-binary coefficient: {c}"),
            }
        }
        Poly::new(out)
    }


}

fn build_tables(m: usize, px_bits: u32) -> (Vec<u16>, Vec<i32>) {
    let n = (1usize << m) - 1;
    let mut exp = vec![0u16; n];
    let mut log = vec![-1i32; 1 << m];
    let prim_low: u16 = (px_bits & !(1u32 << m)) as u16; // px without the x^m term
    let mask_m: u16 = ((1u32 << m) as u16) - 1; // keep m bits
    let mut x: u16 = 1; // alpha^0 = 1
    for i in 0..n {
        exp[i] = x;
        log[x as usize] = i as i32;
        // multiply by alpha (x) modulo px
        let carry = (x & (1 << (m - 1))) != 0; // highest bit before shift
        x = (x << 1) & mask_m;
        if carry { x ^= prim_low; }
    }
    (exp, log)
}

// Solve linear system over GF(2): rows are (mask, rhs) with mask width = cols <= 31
// Returns Some(solution_mask) for variables u_0..u_{cols-1}, or None if inconsistent
fn solve_linear_gf2(cols: u8, rows: &mut Vec<(u32, u8)>) -> Option<u32> {
    let cols = cols as usize;
    // Forward elimination
    let mut row_i = 0usize;
    for col in 0..cols {
        // find pivot row with bit col
        let mut pivot = None;
        for r in row_i..rows.len() {
            if ((rows[r].0 >> col) & 1) == 1 { pivot = Some(r); break; }
        }
        if let Some(piv) = pivot {
            rows.swap(row_i, piv);
            // eliminate other rows' bit col
            let (pmask, prhs) = rows[row_i];
            for r in 0..rows.len() {
                if r == row_i { continue; }
                if ((rows[r].0 >> col) & 1) == 1 {
                    rows[r].0 ^= pmask;
                    rows[r].1 ^= prhs;
                }
            }
            row_i += 1;
            if row_i == rows.len() { break; }
        }
    }
    // Check consistency
    for r in 0..rows.len() { if rows[r].0 == 0 && rows[r].1 == 1 { return None; } }
    // Back substitution: set free vars = 0, solve pivots from top to bottom
    let mut sol: u32 = 0;
    // Determine pivot position for each row
    let mut pivot_cols: Vec<Option<usize>> = Vec::with_capacity(rows.len());
    for r in 0..rows.len() {
        let mut pc: Option<usize> = None;
        for c in 0..cols { if ((rows[r].0 >> c) & 1) == 1 { pc = Some(c); break; } }
        pivot_cols.push(pc);
    }
    for r in 0..rows.len() {
        if let Some(c) = pivot_cols[r] {
            // u_c = rhs XOR sum_{j>c, bit=1} u_j (but we set free vars=0, and pivot below c only)
            // Since we eliminated column-wise across all rows, other pivots don't interfere here.
            if rows[r].1 == 1 { sol |= 1u32 << c; }
        }
    }
    Some(sol)
}
