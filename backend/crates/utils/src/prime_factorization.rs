//! 素因数分解ユーティリティ。
//!
//! - `factor(n)` は u64 向けの高速な分解（試し割り + Pollard Rho + Miller-Rabin）
//! - 大きな整数向けの雛形（Quadratic Sieve）は `#[cfg(feature = "quadratic-sieve")]` で提供

use coding::GFp;
use linalg::Matrix;
use num_bigint::{BigInt, ToBigInt};
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use rand::thread_rng;
use rand::Rng;

/// 公開API: u64 の素因数分解（昇順）。
/// 合成数 n を素因数の多重集合に分解する。
pub fn factor(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }
    let mut factors = Vec::new();
    let mut m = n;

    // 小さい素数での試し割り（2,3,5 の 30進ホイール）
    for &p in [2u64, 3, 5].iter() {
        while m % p == 0 {
            factors.push(p);
            m /= p;
        }
    }
    // 30k + r の r in {±1, ±7, ±11, ±13} で試し割り
    let wheel = [1u64, 7, 11, 13, 17, 19, 23, 29];
    let mut k = 0u64;
    let mut d = 1u64;
    while d * d <= m {
        for &r in &wheel {
            d = 30 * k + r;
            if d < 7 {
                continue;
            } // 1は除外（2,3,5は先に処理済み）
            if d * d > m {
                break;
            }
            while m % d == 0 {
                factors.push(d);
                m /= d;
            }
        }
        k += 1;
    }
    if m > 1 {
        // 残りは素数 or 大きい合成数。u64 では Miller-Rabin + Pollard Rho で対応
        if is_probable_prime_u64(m) {
            factors.push(m);
        } else {
            // Pollard Rho で分割
            let (a, b) = pollards_rho_split_u64(m);
            factors.extend(factor(a));
            factors.extend(factor(b));
        }
    }
    factors.sort_unstable();
    factors
}

pub fn factor_for_big(n: &BigInt) -> Vec<BigInt> {
    // 0) u64に収まるなら高速版に委譲
    if let Some(u) = n.to_u64() {
        if u < 2 {
            return vec![];
        }
        return factor(u).into_iter().map(BigInt::from).collect();
    }
    // 1) n が素数なら即返す
    if is_probable_prime_big(n) {
        return vec![n.clone()];
    }
    // 2) 小さな素数で試し割り（最大 ~5万）
    let (mut small_factors, rem_after_small) = trial_divide_collect(n, 50_000);
    if rem_after_small == BigInt::one() {
        return small_factors;
    }
    if rem_after_small != *n {
        // 何か見つかったので残りを再帰分解
        let mut rest = factor_for_big(&rem_after_small);
        small_factors.append(&mut rest);
        return small_factors;
    }
    // 3) BigInt Pollard's Rho を複数回試す（小〜中規模への近道）
    for _ in 0..5 {
        if let Some(d) = pollards_rho_big(n, 20_000) {
            if d > BigInt::one() && &d < n {
                let mut left = factor_for_big(&d);
                let mut right = factor_for_big(&(n / &d));
                left.append(&mut right);
                return left;
            }
        }
    }
    // 4) ここまでで割れなければ QS にフォールバック
    let bits = n.bits();
    // Heuristic: increase B for 30-32bit inputs and widen M for more relations
    let b_bound: u64 = if bits <= 28 {
        200
    } else if bits <= 32 {
        1200
    } else if bits <= 40 {
        1500
    } else if bits <= 50 {
        2500
    } else if bits <= 60 {
        4000
    // ===== ここから変更 =====
    } else if bits <= 85 {
        // 約83ビットの入力に対応する分岐を追加
        20000 // Bを大幅に増やす
    } else {
        // それより大きい数向けの計算式
        ((bits as f64).powf(1.8)).min(100_000.0) as u64
    };

    // MもBに合わせてスケールさせる
    let m_width: i64 = (b_bound as i64 * 30).clamp(5000, 1_000_000);
    let factor_base = build_factor_base(n, b_bound);
    let sqrt_n = isqrt_big(n);
    let mut sieve_array: Vec<f64> = ((-m_width)..=m_width)
        .map(|z| {
            let x = &sqrt_n + BigInt::from(z);
            let val = &x * &x - n;
            // 初期値: log2(|x^2 - n|) の近似として bit 長を使う
            (val.bits() as f64).max(1.0)
        })
        .collect();
    perform_sieving(&mut sieve_array, &factor_base, n, &sqrt_n, m_width);
    let target_rel = factor_base.len() + 20;
    let mut relations =
        collect_smooth_relations(&sieve_array, &factor_base, n, &sqrt_n, m_width, target_rel);
    if relations.len() <= factor_base.len() {
        for scale in [2, 4, 8] {
            let m2 = (m_width * scale).min(300_000);
            let mut sieve_array2: Vec<f64> = ((-m2)..=m2)
                .map(|z| {
                    let x = &sqrt_n + BigInt::from(z);
                    let val = &x * &x - n;
                    (val.bits() as f64).max(1.0)
                })
                .collect();
            perform_sieving(&mut sieve_array2, &factor_base, n, &sqrt_n, m2);
            let mut rel2 =
                collect_smooth_relations(&sieve_array2, &factor_base, n, &sqrt_n, m2, target_rel);
            relations.append(&mut rel2);
            if relations.len() > factor_base.len() {
                break;
            }
        }
    }
    if relations.len() <= factor_base.len() {
        if let Some(u) = n.to_u64() {
            return factor(u).into_iter().map(BigInt::from).collect();
        }
        return vec![];
    }
    let mut combos = solve_linear_algebra(&relations, factor_base.len());
    if combos.is_empty() {
        // Fallback: enlarge factor base and retry once
        let b2 = (b_bound * 2).min(20_000);
        let factor_base2 = build_factor_base(n, b2);
        let m2 = (m_width * 2).min(300_000);
        let mut sieve_array2: Vec<f64> = ((-m2)..=m2)
            .map(|z| {
                let x = &sqrt_n + BigInt::from(z);
                let val = &x * &x - n;
                (val.bits() as f64).max(1.0)
            })
            .collect();
        perform_sieving(&mut sieve_array2, &factor_base2, n, &sqrt_n, m2);
        let relations2 = collect_smooth_relations(
            &sieve_array2,
            &factor_base2,
            n,
            &sqrt_n,
            m2,
            factor_base2.len() + 20,
        );
        if relations2.len() > factor_base2.len() {
            combos = solve_linear_algebra(&relations2, factor_base2.len());
            relations = relations2;
            // also replace factor_base for subsequent find
            // shadowing is fine in this scope
        } else if let Some(u) = n.to_u64() {
            return factor(u).into_iter().map(BigInt::from).collect();
        } else {
            return vec![];
        }
    }
    let mut factor_opt = None;
    for combo in combos.iter() {
        if let Some(f) = find_factor(n, combo, &relations, &factor_base) {
            factor_opt = Some(f);
            break;
        }
    }
    if let Some(f) = factor_opt {
        let mut out = vec![];
        let other = n / &f;
        for t in [f, other] {
            if is_probable_prime_big(&t) {
                out.push(t);
            } else {
                out.extend(factor_for_big(&t));
            }
        }
        out
    } else if let Some(u) = n.to_u64() {
        factor(u).into_iter().map(BigInt::from).collect()
    } else {
        vec![]
    }
}

// ファクターベースを構築するメインの処理
fn build_factor_base(n: &BigInt, bound: u64) -> Vec<u64> {
    let prime_candidates = sieve_of_eratosthenes(bound);
    let mut factor_base = Vec::new();

    for p in prime_candidates {
        if is_quadratic_residue(n, p) {
            factor_base.push(p);
        }
    }
    factor_base
}

fn is_quadratic_residue(n: &BigInt, p: u64) -> bool {
    if p == 2 {
        return (n & BigInt::one()) == BigInt::one();
    }
    let exponent = (p - 1) / 2;
    let result = mod_pow_big_u64(n, exponent, p);
    result == 1
}

// 篩を実行し、sieve_arrayを更新する（修正版）
fn perform_sieving(
    sieve_array: &mut [f64],
    factor_base: &[u64],
    n: &BigInt,
    sqrt_n: &BigInt,
    m: i64,
) {
    let m_bigint = m.to_bigint().unwrap();

    for &p in factor_base {
        let p_bigint = p.to_bigint().unwrap();
        let log_p = (p as f64).log2(); // 対数の底は何でも良いが、一貫させることが重要

        // 1. s^2 ≡ n (mod p) を解く
        if let Some((s1, s2)) = solve_square_roots(n, p) {
            // --- s1に対する処理 ---
            // 2. 最初のインデックスを見つける
            // sqrt_n + z ≡ s1 (mod p)  =>  z ≡ s1 - sqrt_n (mod p)
            let rem = (&s1 - (sqrt_n % &p_bigint) + &p_bigint) % &p_bigint;
            let mut current_z = -m_bigint.clone();

            // z = -m から始めて、remと合同になる最初のzを探す
            let offset = (rem - (&current_z % &p_bigint) + &p_bigint) % &p_bigint;
            current_z += offset;

            // 3. ステップpでジャンプしながらlog(p)を引く
            while current_z <= m_bigint {
                let index = (current_z.clone() + &m_bigint).to_usize().unwrap();
                if index < sieve_array.len() {
                    sieve_array[index] -= log_p;
                }
                current_z += &p_bigint;
            }

            // --- s2に対する処理（s1 != s2 の場合のみ）---
            if s1 != s2 {
                let rem = (&s2 - (sqrt_n % &p_bigint) + &p_bigint) % &p_bigint;
                let mut current_z = -m_bigint.clone();
                let offset = (rem - (&current_z % &p_bigint) + &p_bigint) % &p_bigint;
                current_z += offset;

                while current_z <= m_bigint {
                    let index = (current_z.clone() + &m_bigint).to_usize().unwrap();
                    if index < sieve_array.len() {
                        sieve_array[index] -= log_p;
                    }
                    current_z += &p_bigint;
                }
            }
        }
    }
}

// 関係式（xと、x^2-nの素因数分解の結果）を収集する
fn collect_smooth_relations(
    sieve_array: &[f64],
    factor_base: &[u64],
    n: &BigInt,
    sqrt_n: &BigInt,
    m: i64,
    desired: usize,
) -> Vec<Relation> {
    let mut relations = Vec::new();
    // indices sorted by sieve score ascending (more negative/smaller first)
    let mut idxs: Vec<usize> = (0..sieve_array.len()).collect();
    idxs.sort_unstable_by(|&a, &b| sieve_array[a].partial_cmp(&sieve_array[b]).unwrap());
    for i in idxs.into_iter() {
        let z = i as i64 - m;
        let x = sqrt_n + BigInt::from(z);
        let mut y = (&x * &x - n).abs();
        if y.is_zero() {
            continue;
        }
        let mut exps: Vec<u32> = vec![0; factor_base.len()];
        for (j, &p) in factor_base.iter().enumerate() {
            let p_bi = BigInt::from(p);
            while (&y % &p_bi).is_zero() {
                y /= &p_bi;
                exps[j] += 1;
            }
        }
        if y.is_one() {
            relations.push(Relation { x, exponents: exps });
            if relations.len() >= desired {
                break;
            }
        }
    }
    relations
}

// 関係式を保持するための構造体
struct Relation {
    x: BigInt,
    exponents: Vec<u32>,
}

// 行列の線形従属な関係を見つける
fn solve_linear_algebra(relations: &[Relation], num_primes: usize) -> Vec<Vec<usize>> {
    // 行: num_primes, 列: relations.len()
    type F = GFp<2>;
    let rows = num_primes;
    let cols = relations.len();
    if cols == 0 {
        return vec![];
    }
    let mut data: Vec<F> = Vec::with_capacity(rows * cols);
    for i in 0..rows {
        for rel in relations {
            let bit = rel.exponents[i] & 1;
            data.push(F::new(bit as i64));
        }
    }
    let a: Matrix<F> = Matrix::new(rows, cols, data).unwrap();
    let rref = a.rref().unwrap();
    // ピボット列検出
    let mut pivot_col = vec![false; cols];
    let mut r = 0usize;
    for c in 0..cols {
        if r >= rows {
            break;
        }
        if !rref[(r, c)].is_zero() {
            // この列はピボット
            pivot_col[c] = true;
            // 次の行に進む
            r += 1;
        }
    }
    // 最初の自由列を1にして依存を構築
    let mut combos: Vec<Vec<usize>> = Vec::new();
    for j in 0..cols {
        if pivot_col[j] {
            continue;
        }
        let mut select = vec![GFp::<2>::new(0); cols];
        select[j] = F::new(1);
        let mut row = 0usize;
        for c in 0..cols {
            if row >= rows {
                break;
            }
            if !rref[(row, c)].is_zero() {
                let coeff = rref[(row, j)];
                select[c] = coeff;
                row += 1;
            }
        }
        let chosen: Vec<usize> = select
            .into_iter()
            .enumerate()
            .filter_map(|(idx, v)| if !v.is_zero() { Some(idx) } else { None })
            .collect();
        if !chosen.is_empty() {
            combos.push(chosen);
        }
    }
    combos
}

// 最終的な因数を見つける
fn find_factor(
    n: &BigInt,
    combination: &[usize],
    relations: &[Relation],
    factor_base: &[u64],
) -> Option<BigInt> {
    if combination.is_empty() {
        return None;
    }
    // X = Π x_i (mod n)
    let mut x_prod = BigInt::one();
    for &idx in combination {
        x_prod = (x_prod * &relations[idx].x) % n;
    }
    // E = sum of exponents
    let num_p = relations[0].exponents.len();
    let mut e_sum = vec![0u32; num_p];
    for &idx in combination {
        for (j, exp) in e_sum.iter_mut().enumerate() {
            *exp += relations[idx].exponents[j];
        }
    }
    // Y = Π p_j^(e_sum[j]/2) (mod n)
    let mut y_prod = BigInt::one();
    for j in 0..num_p {
        let half = e_sum[j] / 2;
        if half == 0 {
            continue;
        }
        let p = BigInt::from(factor_base[j]);
        y_prod = (y_prod * mod_pow_bigint(&p, &BigInt::from(half), n)) % n;
    }
    let diff = (&x_prod - &y_prod).abs();
    let g = diff.gcd(n);
    if g > BigInt::one() && &g < n {
        Some(g)
    } else {
        None
    }
}

fn sieve_of_eratosthenes(max: u64) -> Vec<u64> {
    let mut is_prime = vec![true; (max + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=((max as f64).sqrt() as u64) {
        if is_prime[i as usize] {
            for j in (i * i..=max).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i as u64) } else { None })
        .collect()
}

// ===== Pre-processing for BigInt factorization =====
fn trial_divide_collect(n: &BigInt, max_prime: u64) -> (Vec<BigInt>, BigInt) {
    let mut rem = n.clone();
    let mut out: Vec<BigInt> = Vec::new();
    if rem.is_zero() || rem.is_one() {
        return (out, rem);
    }
    let primes = sieve_of_eratosthenes(max_prime);
    for p in primes {
        let pb = BigInt::from(p);
        while (&rem % &pb).is_zero() {
            rem /= &pb;
            out.push(pb.clone());
            if rem.is_one() {
                break;
            }
        }
        if pb.clone() * pb.clone() > rem {
            break;
        }
    }
    (out, rem)
}

fn pollards_rho_big(n: &BigInt, max_iters: usize) -> Option<BigInt> {
    use rand::Rng;
    if n.is_even() {
        return Some(BigInt::from(2u32));
    }
    let mut rng = thread_rng();
    let one = BigInt::one();
    let f = |x: &BigInt, c: &BigInt, m: &BigInt| -> BigInt { (x * x + c) % m };
    for _ in 0..5 {
        // different random constants
        let c = BigInt::from(rng.gen_range(1u64..=1_000_000u64));
        let mut x = BigInt::from(rng.gen_range(2u64..=1_000_000u64)) % n;
        let mut y = x.clone();
        let mut d = one.clone();
        let mut it = 0usize;
        while d == one && it < max_iters {
            x = f(&x, &c, n);
            y = f(&f(&y, &c, n), &c, n);
            let mut diff = &x - &y;
            if diff.is_negative() {
                diff = -diff;
            }
            d = diff.gcd(n);
            it += 1;
        }
        if d > one && &d < n {
            return Some(d);
        }
    }
    None
}

// ===== Miller-Rabin (u64) =====
fn modmul_u128(a: u128, b: u128, m: u128) -> u128 {
    ((a % m) * (b % m)) % m
}
fn modpow_u128(mut a: u128, mut e: u128, m: u128) -> u128 {
    let mut r = 1u128;
    while e > 0 {
        if e & 1 == 1 {
            r = modmul_u128(r, a, m);
        }
        a = modmul_u128(a, a, m);
        e >>= 1;
    }
    r
}

fn is_probable_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37].iter() {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return n == p;
        }
    }
    // n-1 = d * 2^s (d odd)
    let mut d = n - 1;
    let mut s = 0u32;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    let bases: [u64; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    'next_base: for &a in bases.iter() {
        let mut x = modpow_u128((a as u128) % (n as u128), d as u128, n as u128);
        if x == 1 || x == (n as u128 - 1) {
            continue;
        }
        for _ in 1..s {
            x = modmul_u128(x, x, n as u128);
            if x == n as u128 - 1 {
                continue 'next_base;
            }
        }
        return false;
    }
    true
}

// ===== Pollard's Rho (u64) =====
fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn pollards_rho_split_u64(n: u64) -> (u64, u64) {
    if n % 2 == 0 {
        return (2, n / 2);
    }
    let mut rng = thread_rng();
    let f = |x: u128, c: u128, m: u128| -> u128 { (modmul_u128(x, x, m) + c) % m };
    loop {
        let c = (rng.gen_range(1u64..n - 1)) as u128;
        let mut x = (rng.gen_range(0u64..n)) as u128;
        let mut y = x;
        let m = n as u128;
        let mut d = 1u64;
        while d == 1 {
            x = f(x, c, m);
            y = f(f(y, c, m), c, m);
            let diff = x.abs_diff(y);
            d = gcd_u64(diff as u64, n);
            if d == n {
                break;
            }
        }
        if d > 1 && d < n {
            return (d, n / d);
        }
    }
}

// ===== BigInt helpers for QS stubs =====
fn isqrt_big(n: &BigInt) -> BigInt {
    // Newton iteration for integer sqrt
    if n.is_zero() {
        return BigInt::zero();
    }
    let mut x = BigInt::from(1u64) << (n.bits() / 2);
    loop {
        let y = (&x + n / &x) >> 1;
        if y >= x {
            return x;
        }
        x = y;
    }
}

fn mod_pow_big_u64(a: &BigInt, e: u64, m: u64) -> u64 {
    let mut base = (a % m).to_u64().unwrap_or(0);
    let mut exp = e;
    let mut res = 1u64;
    while exp > 0 {
        if exp & 1 == 1 {
            res = ((res as u128 * base as u128) % m as u128) as u64;
        }
        base = ((base as u128 * base as u128) % m as u128) as u64;
        exp >>= 1;
    }
    res
}

fn solve_square_roots(_n: &BigInt, _p: u64) -> Option<(BigInt, BigInt)> {
    // Tonelli–Shanks for odd prime p
    let p = _p;
    if p == 2 {
        return Some((BigInt::from(1u64), BigInt::from(1u64)));
    }
    let n_mod = (_n % p).to_u64().unwrap() % p;
    if n_mod == 0 {
        return Some((BigInt::from(0u64), BigInt::from(0u64)));
    }
    // Check Legendre symbol n^((p-1)/2) ≡ 1
    if mod_pow_big_u64(_n, (p - 1) / 2, p) != 1 {
        return None;
    }
    // write p-1 = q * 2^s with q odd
    let mut q = p - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    // find z non-residue
    let mut z = 2u64;
    while mod_pow_big_u64(&BigInt::from(z), (p - 1) / 2, p) == 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = modpow_u128(z as u128, q as u128, p as u128) as u64;
    let mut t = modpow_u128(n_mod as u128, q as u128, p as u128) as u64;
    let mut r = modpow_u128(n_mod as u128, q.div_ceil(2) as u128, p as u128) as u64;
    while t != 1 {
        let mut i = 1u32;
        let mut t2i = (t * t) % p;
        while t2i != 1 {
            t2i = (t2i * t2i) % p;
            i += 1;
            if i == m {
                return None;
            }
        }
        let b = modpow_u128(c as u128, 1u128 << (m - i - 1), p as u128) as u64;
        r = (r * b) % p;
        c = (b * b) % p;
        t = (t * c) % p;
        m = i;
    }
    let s1 = BigInt::from(r);
    let s2 = BigInt::from((p - r) % p);
    Some((s1, s2))
}

fn mod_pow_bigint(a: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
    let mut base = a % m;
    let mut exp = e.clone(); // ループ内で変更するためクローン
    let mut res = BigInt::one();

    while exp > BigInt::zero() {
        if &exp & BigInt::one() == BigInt::one() {
            res = (&res * &base) % m;
        }
        base = (&base * &base) % m;
        exp >>= 1; // 右に1ビットシフト
    }
    res
}

fn is_probable_prime_big(n: &BigInt) -> bool {
    if let Some(u) = n.to_u64() {
        return is_probable_prime_u64(u);
    }
    if n < &BigInt::from(2u64) {
        return false;
    }
    // simple bases for Miller–Rabin on BigInt
    let bases: [u64; 5] = [2, 3, 5, 7, 11];
    // n-1 = d * 2^s
    let mut d = n - 1u32;
    let mut s = 0u32;
    while (&d & BigInt::from(1u32)).is_zero() {
        d >>= 1;
        s += 1;
    }
    'outer: for &a in bases.iter() {
        let a_b = BigInt::from(a) % n;
        let mut x = mod_pow_bigint(&a_b, &d, n);
        if x.is_one() || x == n - 1u32 {
            continue;
        }
        for _ in 1..s {
            x = (&x * &x) % n;
            if x == n - 1u32 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

// removed fallback helper: we now pass factor_base into find_factor
