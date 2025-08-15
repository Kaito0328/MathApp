use crate::prime::GFp;
use linalg::{Field, Matrix, Scalar, Vector};
use num_traits::{One, Zero};
use std::collections::HashMap;

// ハミング距離
pub fn hamming_distance<F: Scalar + PartialEq + Clone>(a: &Vector<F>, b: &Vector<F>) -> usize {
    assert_eq!(a.dim(), b.dim());
    (0..a.dim()).filter(|&i| a[i] != b[i]).count()
}

// 最小ハミング距離（一般コード）
pub fn hamming_d_min<F: Scalar + PartialEq + Clone>(codebook: &[Vector<F>]) -> usize {
    let m = codebook.len();
    let n = codebook.first().map(|v| v.dim()).unwrap_or(0);
    let mut dmin = n;
    for i in 0..m {
        for j in (i + 1)..m {
            dmin = dmin.min(hamming_distance(&codebook[i], &codebook[j]));
        }
    }
    dmin
}

// 線形符号の最小距離（ゼロ語除外）
pub fn linear_hamming_d_min<F: Field + Clone + PartialEq + Zero>(codebook: &[Vector<F>]) -> usize {
    let n = codebook.first().map(|v| v.dim()).unwrap_or(0);
    let mut dmin = n;
    for cw in codebook.iter() {
        if (0..cw.dim()).all(|i| cw[i].is_zero()) {
            continue;
        }
        let w = (0..cw.dim()).filter(|&i| !cw[i].is_zero()).count();
        dmin = dmin.min(w);
    }
    dmin
}

// 符号化率
pub fn coding_rate_from_generator<F: Field>(g: &Matrix<F>) -> f64 {
    (g.rows as f64) / (g.cols as f64)
}

// 最小距離復号（インデックス返却）
pub fn md_decode<F: Field + Clone + PartialEq>(codebook: &[Vector<F>], r: &Vector<F>) -> usize {
    let mut idx = 0usize;
    let mut best = usize::MAX;
    for (i, c) in codebook.iter().enumerate() {
        let d = hamming_distance(c, r);
        if d < best {
            best = d;
            idx = i;
        }
    }
    idx
}

// GF(p) の全長 k ベクトル列挙（p は型が持つ）
pub fn generate_vectors_gfp<const P: u16>(k: usize) -> Vec<Vec<crate::prime::GFp<P>>> {
    use crate::prime::GFp;
    if k == 0 {
        return vec![vec![]];
    }
    let prev = generate_vectors_gfp::<P>(k - 1);
    let mut out = Vec::new();
    for v in prev.into_iter() {
        for x in 0..(P) {
            let mut nv = v.clone();
            nv.push(GFp::<P>(x));
            out.push(nv);
        }
    }
    out
}

// 生成行列からコードブック（GF(p) 専用、k が小さい用途向け）
pub fn generate_codebook_gfp<const P: u16>(
    g: &Matrix<crate::prime::GFp<P>>,
) -> Vec<Vector<crate::prime::GFp<P>>> {
    let k = g.rows;
    let mut out = Vec::new();
    for u in generate_vectors_gfp::<P>(k).into_iter() {
        let c = (Vector::new(u) * g).row(0).unwrap();
        out.push(c);
    }
    out
}

// 標準形生成行列 [I_k | P] から H = [P^T | I_{n-k}] を生成
pub fn formed_g_to_h<F: Field + Clone + PartialEq + Zero + One>(g: &Matrix<F>) -> Matrix<F> {
    let k = g.rows;
    let n = g.cols;
    let r = n - k;
    // 左側 I_k を確認
    for i in 0..k {
        for j in 0..k {
            let exp = if i == j { F::one() } else { F::zero() };
            assert!(g[(i, j)] == exp, "G must be in standard form [I|P]");
        }
    }
    let mut h = Matrix::new(r, n, vec![F::zero(); r * n]).unwrap();
    // 左ブロックに P^T を配置（G の右ブロックを転置）
    for i in 0..r {
        for j in 0..k {
            h[(i, j)] = g[(j, k + i)].clone();
        }
    }
    // 右ブロックに I_r
    for i in 0..r {
        h[(i, k + i)] = F::one();
    }
    h
}

// 重み分布（0..=n の n+1 ビンを返す）
pub fn weight_distribution<F: Field + Clone + PartialEq + Zero>(
    codebook: &[Vector<F>],
) -> Vec<usize> {
    let n = codebook.first().map(|v| v.dim()).unwrap_or(0);
    let mut bins = vec![0usize; n + 1];
    for c in codebook.iter() {
        let w = (0..n).filter(|&i| !c[i].is_zero()).count();
        bins[w] += 1;
    }
    bins
}

// ベクトルのハミング重み
pub fn hamming_weight<F: Field + Clone + PartialEq + Zero>(v: &Vector<F>) -> usize {
    (0..v.dim()).filter(|&i| !v[i].is_zero()).count()
}

// 一般の生成行列 G を標準形 [I_k | P] に変換（列の並べ替えも許可）
// 返り値: (G_sys, perm) where perm[j] は G_sys の列 j が元の G のどの列だったか。
pub fn to_systematic_g<F: Field + Clone + PartialEq + Zero + One>(
    g: &Matrix<F>,
) -> (Matrix<F>, Vec<usize>) {
    let k = g.rows;
    let n = g.cols;
    let mut a = g.clone();
    let mut perm: Vec<usize> = (0..n).collect();
    let mut row = 0usize;
    for col in 0..n {
        if row >= k {
            break;
        }
        // ピボット探索（row..k-1, col..n-1 の中で非零）
        let mut pivot_col = None;
        let mut pivot_row = None;
        for c in col..n {
            for r in row..k {
                if !a[(r, c)].is_zero() {
                    pivot_col = Some(c);
                    pivot_row = Some(r);
                    break;
                }
            }
            if pivot_col.is_some() {
                break;
            }
        }
        if pivot_col.is_none() {
            continue;
        }
        let pc = pivot_col.unwrap();
        let pr = pivot_row.unwrap();
        // 列入替え pc <-> col
        if pc != col {
            for r in 0..k {
                let tmp = a[(r, col)].clone();
                a[(r, col)] = a[(r, pc)].clone();
                a[(r, pc)] = tmp;
            }
            perm.swap(col, pc);
        }
        // 行入替え pr <-> row
        if pr != row {
            for c in 0..n {
                let tmp = a[(row, c)].clone();
                a[(row, c)] = a[(pr, c)].clone();
                a[(pr, c)] = tmp;
            }
        }
        // ピボットを 1 にスケール
        let inv = F::one() / a[(row, col)].clone();
        for c in 0..n {
            a[(row, c)] = a[(row, c)].clone() * inv.clone();
        }
        // 他行の該当列を 0 に
        for r in 0..k {
            if r == row {
                continue;
            }
            let factor = a[(r, col)].clone();
            if factor.is_zero() {
                continue;
            }
            for c in 0..n {
                a[(r, c)] = a[(r, c)].clone() - factor.clone() * a[(row, c)].clone();
            }
        }
        row += 1;
        if row == k {
            break;
        }
    }
    (a, perm)
}

// 一般の生成行列 G からパリティ検査行列 H を構成（列順は元の G に合わせて返す）
pub fn parity_check_from_generator<F: Field + Clone + PartialEq + Zero + One>(
    g: &Matrix<F>,
) -> Matrix<F> {
    let (gs, perm) = to_systematic_g(g);
    let hs_sys = formed_g_to_h(&gs);
    // perm は G_sys の列 -> 元の列の対応。H_sys の列も同じ対応で元の順に戻す。
    let r = hs_sys.rows;
    let n = hs_sys.cols;
    let mut h = Matrix::new(r, n, vec![F::zero(); r * n]).unwrap();
    for j in 0..n {
        let orig = perm[j];
        for i in 0..r {
            h[(i, orig)] = hs_sys[(i, j)].clone();
        }
    }
    h
}

// GF(2) 向けシンドローム復号（有界距離復号: 重み <= t の誤りを補正）。見つからなければ None。
pub fn syndrome_decode_gf2(
    h: &Matrix<GFp<2>>,
    r: &Vector<GFp<2>>,
    t: usize,
) -> Option<Vector<GFp<2>>> {
    let n = h.cols;
    let m = h.rows;
    assert_eq!(r.dim(), n);
    // s = H * r^T
    let s = |v: &Vector<GFp<2>>| -> Vec<GFp<2>> {
        let mut out = vec![GFp::<2>(0); m];
        for i in 0..m {
            let mut acc = GFp::<2>(0);
            for j in 0..n {
                // acc += h[i,j] * v[j] over GF(2)
                let prod = h[(i, j)] * v[j];
                acc = acc + prod;
            }
            out[i] = acc;
        }
        out
    };
    let syn_r = s(r);
    let zero_syn = syn_r.iter().all(|x| x.0 == 0);
    if zero_syn {
        return Some(r.clone());
    }

    // 症候表: syndrome -> エラーベクトル（最小重み）
    let mut table: HashMap<Vec<u16>, Vector<GFp<2>>> = HashMap::new();
    // 重み 1..=t を列挙
    fn enumerate_errors(
        n: usize,
        start: usize,
        left: usize,
        cur: &mut Vec<usize>,
        out: &mut Vec<Vec<usize>>,
    ) {
        if left == 0 {
            out.push(cur.clone());
            return;
        }
        for pos in start..=n - left {
            cur.push(pos);
            enumerate_errors(n, pos + 1, left - 1, cur, out);
            cur.pop();
        }
    }
    for w in 1..=t {
        let mut combs = Vec::new();
        enumerate_errors(n, 0, w, &mut Vec::new(), &mut combs);
        for idxs in combs {
            let mut e = vec![GFp::<2>(0); n];
            for &j in &idxs {
                e[j] = GFp::<2>(1);
            }
            let ev = Vector::new(e);
            let syn = s(&ev);
            let key: Vec<u16> = syn.iter().map(|x| x.0).collect();
            table.entry(key).or_insert(ev);
        }
    }
    let key: Vec<u16> = syn_r.iter().map(|x| x.0).collect();
    if let Some(e) = table.get(&key) {
        let corrected = r.clone() + e.clone();
        return Some(corrected);
    }
    None
}

pub fn bounded_distance_decode_gf2(
    h: &Matrix<GFp<2>>,
    r: &Vector<GFp<2>>,
    t: usize,
) -> Option<Vector<GFp<2>>> {
    syndrome_decode_gf2(h, r, t)
}
