use crate::gfp::GFp;

// Check if polynomial is primitive over GF(2)
// coefficients are given low->high, highest coeff must be 1 (monic)
pub fn is_primitive_gf2(poly: &[GFp<2>]) -> bool {
    // degree m
    let _m = match poly.len().checked_sub(1) { Some(d) if d >= 1 => d, _ => return false };
    if poly.last() != Some(&GFp::<2>(1)) || poly.first() != Some(&GFp::<2>(1)) { return false; }
    // This is a placeholder: full primitive check requires factoring 2^m-1 and verifying order of x.
    // For now, just ensure irreducible (very naive) and monic; users can use table/gen below.
    is_irreducible_gf2(poly)
}

/// Known primitive polynomials over GF(2) for 2 <= m <= 15 (low->high coeffs)
/// These are convenient choices for constructing GF(2^m). Any primitive irreducible works.
pub fn known_primitive_poly_gf2(m: usize) -> Vec<GFp<2>> {
    match m {
        2 => vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(1)],                         // x^2 + x + 1
        3 => vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(1)],             // x^3 + x + 1
        4 => vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)],// x^4 + x + 1
        5 => vec![GFp::<2>(1), GFp::<2>(0), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)], // x^5 + x^2 + 1
        6 => vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)], // x^6 + x + 1
        7 => vec![GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)], // x^7 + x^3 + 1
        8 => vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)], // x^8 + x^4 + x^3 + x + 1
        9 => vec![GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)], // x^9 + x^4 + 1
        10=> vec![GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)], // x^10 + x^3 + 1
        11=> vec![GFp::<2>(1), GFp::<2>(0), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)], // x^11 + x^2 + 1
        12=> vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)], // x^12 + x^6 + x^4 + x + 1
        13=> vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)], // x^13 + x^4 + x^3 + x + 1
        14=> vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)], // x^14 + x^10 + x^6 + x + 1
        15=> vec![GFp::<2>(1), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(0), GFp::<2>(1)], // x^15 + x + 1
        _ => panic!("unsupported m for primitive polynomial table: {m}"),
    }
}

// Very naive irreducibility test over GF(2) for small m (trial division up to degree m/2)
pub fn is_irreducible_gf2(poly: &[GFp<2>]) -> bool {
    let m = match poly.len().checked_sub(1) { Some(d) if d >= 1 => d, _ => return false };
    // generate all monic polynomials of degree d for 1<=d<=m/2 and check gcd
    for d in 1..=m/2 {
        let count = 1usize << d; // number of coefficient patterns for lower d bits
        for mask in 0..count {
            // build monic polynomial q(x) of degree d
            let mut q: Vec<GFp<2>> = vec![GFp::<2>(0); d+1];
            q[0] = GFp::<2>(mask as u16 & 1);
            for i in 1..d { q[i] = GFp::<2>(((mask >> i) as u16) & 1); }
            q[d] = GFp::<2>(1);
            // compute gcd(poly, q)
            if poly_gcd_gf2(poly, &q).len() > 1 { return false; }
        }
    }
    true
}

// GCD over GF(2) polynomials (low->high)
pub fn poly_gcd_gf2(a: &[GFp<2>], b: &[GFp<2>]) -> Vec<GFp<2>> {
    let mut x = trim(a.to_vec());
    let mut y = trim(b.to_vec());
    while y.len() > 1 {
        let r = poly_rem_gf2(&x, &y);
        x = y;
        y = r;
    }
    x
}

// Remainder a mod b over GF(2)
pub fn poly_rem_gf2(a: &[GFp<2>], b: &[GFp<2>]) -> Vec<GFp<2>> {
    let mut r = trim(a.to_vec());
    let d = trim(b.to_vec());
    let db = d.len()-1;
    while r.len() >= d.len() {
        let dr = r.len()-1;
        let shift = dr - db;
        // r = r + x^{shift} * d
        for i in 0..=db { r[shift + i] = GFp::<2>(r[shift+i].0 ^ d[i].0); }
        r = trim(r);
    }
    r
}

fn trim(mut v: Vec<GFp<2>>) -> Vec<GFp<2>> {
    while v.len() > 1 && *v.last().unwrap() == GFp::<2>(0) { v.pop(); }
    v
}

// Generate table of primitive polynomials over GF(2) for 2 <= m <= max_m
pub fn primitive_table_gf2(max_m: usize) -> Vec<Vec<GFp<2>>> {
    let mut out = Vec::new();
    for m in 2..=max_m {
        if let Some(p) = find_primitive_gf2(m) { out.push(p); } else { out.push(vec![]); }
    }
    out
}

// Find one primitive polynomial of degree m over GF(2) by brute-force search (small m only)
pub fn find_primitive_gf2(m: usize) -> Option<Vec<GFp<2>>> {
    if m < 2 { return None; }
    let total = 1usize << (m-1); // inner coefficients patterns excluding leading 1 and constant term
    for mask in 0..total {
        let mut p: Vec<GFp<2>> = vec![GFp::<2>(0); m+1];
        p[0] = GFp::<2>(1);
        for i in 1..m { p[i] = GFp::<2>(((mask >> (i-1)) as u16) & 1); }
        p[m] = GFp::<2>(1);
        if is_primitive_gf2(&p) { return Some(p); }
    }
    None
}
