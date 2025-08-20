use std::cmp::Ordering;

use crate::polynomial::Polynomial;
use linalg::Field;
use num_complex::Complex;
use num_traits::Zero;

#[derive(Clone, Copy, Debug)]
pub enum RootMethod {
    // 後方互換: 既定ではハイブリッドを指す
    JenkinsTraub,
    // 純粋なJenkins–Traub
    JenkinsTraubPure,
    // Durand–Kernerで初期化するハイブリッドJT
    JenkinsTraubHybrid,
    DurandKerner,
}

impl<F: Field> Polynomial<F> {
    pub fn from_roots(roots: Vec<F>) -> Self {
        let deg = roots.len();
        if deg == 0 {
            return Polynomial::one();
        }

        if deg == 1 {
            return Polynomial::new(vec![-roots[0].clone(), F::one()]);
        }

        let mid = deg / 2;
        let left = Polynomial::from_roots(roots[..mid].to_vec());
        let right = Polynomial::from_roots(roots[mid..].to_vec());
        &left * &right
    }

    pub fn gcd(a: &Self, b: &Self) -> Self {
        let mut r0 = a.clone();
        let mut r1 = b.clone();
        while r1.deg() >= 0 && !(r1.coeffs.len() == 1 && r1.coeffs[0].is_zero()) {
            let (_q, r) = r0.div_rem(&r1);
            r0 = r1;
            r1 = r;
        }
        r0.monic()
    }

    pub fn lcm(a: &Self, b: &Self) -> Self {
        if a.deg() < 0 {
            return b.clone();
        }
        if b.deg() < 0 {
            return a.clone();
        }
        let g = Polynomial::gcd(a, b);
        let ab = a * b; // Mul演算子
        let (q, _r) = ab.div_rem(&g);
        q.monic()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Root {
    pub value: Complex<f64>,
    pub multiplicity: usize,
}

impl Polynomial<f64> {
    /// 実数係数多項式を、複素数係数多項式に変換する。
    pub fn to_complex(&self) -> Polynomial<Complex<f64>> {
        let complex_coeffs = self
            .coeffs
            .iter()
            .map(|&c| Complex::new(c, 0.0)) // 各f64係数をComplex<f64>に変換
            .collect();

        Polynomial::new(complex_coeffs)
    }
    pub fn find_roots(&self) -> Vec<Complex<f64>> {
        // 既定は安定なハイブリッドJT
        self.find_roots_with(RootMethod::JenkinsTraubHybrid)
    }

    pub fn find_roots_with(&self, method: RootMethod) -> Vec<Complex<f64>> {
        match method {
            RootMethod::JenkinsTraub => Self::find_roots_jenkins_traub_hybrid(self),
            RootMethod::JenkinsTraubHybrid => Self::find_roots_jenkins_traub_hybrid(self),
            RootMethod::JenkinsTraubPure => Self::find_roots_jenkins_traub_pure(self),
            RootMethod::DurandKerner => Self::find_roots_durand_kerner(self),
        }
    }

    pub fn group_roots(roots: &[Complex<f64>], tolerance: f64) -> Vec<Root> {
        if roots.is_empty() {
            return vec![];
        }

        // ★修正点1: 最初に可変なVecのコピーを作成する
        let mut sorted_roots = roots.to_vec();

        // ★修正点2: コピーしたVecをソートする
        sorted_roots.sort_by(|a, b| {
            // f64のソートはpartial_cmpを使うのが丁寧
            match a.re.partial_cmp(&b.re) {
                Some(Ordering::Equal) => a.im.partial_cmp(&b.im).unwrap_or(Ordering::Equal),
                Some(ord) => ord,
                None => Ordering::Equal, // NaNなどの場合の処理
            }
        });

        let mut grouped: Vec<Root> = Vec::new();

        // ★修正点3: ソート済みのVecに対してループを回す
        for root in sorted_roots {
            // 最後のグループと比較し、近ければマージする
            if let Some(last) = grouped.last_mut() {
                if (last.value - root).norm() < tolerance {
                    // 既存のグループにマージ（平均値を更新）
                    let total_weight = last.multiplicity as f64 + 1.0;
                    last.value = (last.value * (last.multiplicity as f64) + root) / total_weight;
                    last.multiplicity += 1;
                    continue; // 次のrootへ
                }
            }

            // 新しいグループを作成
            grouped.push(Root {
                value: root,
                multiplicity: 1,
            });
        }

        grouped
    }

    fn find_roots_durand_kerner(p: &Self) -> Vec<Complex<f64>> {
        let n = p.deg();
        if n <= 0 {
            return Vec::new();
        }
        // 複素係数にして正規化（モニック化）
        let mut pc = p.to_complex();
        let lc = pc.coeffs.last().cloned().unwrap_or_else(Complex::zero);
        if lc != Complex::new(1.0, 0.0) && lc != Complex::new(0.0, 0.0) {
            pc = &pc / &Polynomial::new(vec![lc]);
        }

        let n_usize = n as usize;
        // 初期値: 単位円上に均等配置（わずかに半径 < 1）
        let radius = 0.8f64;
        let mut roots: Vec<Complex<f64>> = (0..n_usize)
            .map(|k| {
                let theta = 2.0 * std::f64::consts::PI * (k as f64) / (n as f64);
                Complex::new(radius * theta.cos(), radius * theta.sin())
            })
            .collect();

        let max_iter = 256;
        let tol = 1e-12;
        for _ in 0..max_iter {
            let mut max_delta = 0.0;
            for i in 0..n_usize {
                let xi = roots[i];
                let fx = pc.eval(xi);
                // 既存の他根との差の積
                let mut denom = Complex::new(1.0, 0.0);
                for (j, &xj) in roots.iter().enumerate() {
                    if i != j {
                        denom *= xi - xj;
                    }
                }
                if denom == Complex::new(0.0, 0.0) {
                    // まれに重複してしまう場合は微小摂動
                    denom = Complex::new(1e-12, 0.0);
                }
                let delta = fx / denom;
                roots[i] -= delta;
                let d = delta.norm();
                if d > max_delta {
                    max_delta = d;
                }
            }
            if max_delta < tol {
                break;
            }
        }
        roots
    }

    fn find_roots_deg_1(p: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        let mut roots = Vec::new();
        if p[1].is_zero() {
            return roots; // 係数が0の場合は根なし
        }
        roots.push(-p[0] / p[1]);
        roots
    }

    fn find_roots_deg_2(p: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        let mut roots = Vec::new();
        let c2 = p[2];
        let c1 = p[1];
        let c0 = p[0];
        let delta = (c1 * c1 - 4.0 * c2 * c0).sqrt();
        roots.push((-c1 + delta) / (2.0 * c2));
        roots.push((-c1 - delta) / (2.0 * c2));

        roots
    }

    // 共通: 実多項式 -> 複素モニック化
    fn to_monic_complex(p: &Self) -> Polynomial<Complex<f64>> {
        let mut pc = p.to_complex();
        let lc = pc.coeffs.last().cloned().unwrap_or_else(Complex::zero);
        if lc != Complex::new(0.0, 0.0) && lc != Complex::new(1.0, 0.0) {
            pc = &pc / &Polynomial::new(vec![lc]);
        }
        pc
    }

    // 共通: Newton研磨
    fn newton_polish(
        p: &Polynomial<Complex<f64>>,
        mut z: Complex<f64>,
        max_iter: usize,
        tol: f64,
    ) -> Complex<f64> {
        let d = p.differentiate();
        for _ in 0..max_iter {
            let f = p.eval(z);
            if f.norm() < tol {
                break;
            }
            let fp = d.eval(z);
            if fp == Complex::new(0.0, 0.0) {
                break;
            }
            z -= f / fp;
        }
        z
    }

    // 共通: 推定根に基づく一次/二次デフレーション
    fn deflate_with_estimate(
        mut p: Polynomial<Complex<f64>>,
        s: Complex<f64>,
        real_thr: f64,
        roots: &mut Vec<Complex<f64>>,
    ) -> Polynomial<Complex<f64>> {
        if s.im.abs() < real_thr {
            let s_real = Complex::new(s.re, 0.0);
            roots.push(s_real);
            let (np, _r) = p.div_rem(&Polynomial::from_roots(vec![s_real]));
            p = np.monic();
        } else {
            let sc = s.conj();
            roots.push(s);
            roots.push(sc);
            let (np, _r) = p.div_rem(&Polynomial::from_roots(vec![s, sc]));
            p = np.monic();
        }
        p
    }

    // 共通: 低次数の仕上げ
    fn finish_low_degree(p: Polynomial<Complex<f64>>, roots: &mut Vec<Complex<f64>>) {
        if p.deg() == 2 {
            roots.extend(Self::find_roots_deg_2(p.coeffs));
        } else if p.deg() == 1 {
            roots.extend(Self::find_roots_deg_1(p.coeffs));
        }
    }

    // ハイブリッドJT: DKで初期化→Newton→デフレーション
    fn find_roots_jenkins_traub_hybrid(p: &Self) -> Vec<Complex<f64>> {
        let deg = p.deg();
        if deg <= 0 {
            return Vec::new();
        }
        let mut roots = Vec::with_capacity(deg as usize);
        let mut pc = Self::to_monic_complex(p);

        // DKで初期根集合を取得
        let real_poly = Polynomial::new(pc.coeffs.iter().map(|c| c.re).collect::<Vec<f64>>());
        let mut seeds = Self::find_roots_durand_kerner(&real_poly);
        if seeds.is_empty() {
            // 稀な失敗時は純JTへフォールバック
            return Self::find_roots_jenkins_traub_pure(p);
        }

        let tol = 1e-12;
        let real_thr = 1e-10;
        while pc.deg() > 2 {
            // 残差が小さいシードを優先
            seeds.sort_by(|a, b| pc.eval(*a).norm().partial_cmp(&pc.eval(*b).norm()).unwrap());
            let mut s = seeds.remove(0);
            s = Self::newton_polish(&pc, s, 50, tol);
            pc = Self::deflate_with_estimate(pc, s, real_thr, &mut roots);
            // 近いシードを間引き
            seeds.retain(|z| (*z - s).norm() > 1e-6 && (*z - s.conj()).norm() > 1e-6);
        }
        Self::finish_low_degree(pc, &mut roots);
        roots
    }

    // 純JT: DK不使用
    fn find_roots_jenkins_traub_pure(p: &Self) -> Vec<Complex<f64>> {
        let deg = p.deg();
        if deg <= 0 {
            return Vec::new(); // 0次または1次の多項式は根なし
        }
        let mut roots = Vec::with_capacity(deg as usize);

        // 複素モニック化
        let mut p = Self::to_monic_complex(p);

        // Cauchy の上界: 全根は |z| <= 1 + max |a_i/a_n|
        let max_ratio = p
            .coeffs
            .iter()
            .take(p.coeffs.len().saturating_sub(1))
            .map(|c| c.norm())
            .fold(0.0f64, f64::max);
        let root_bound = 1.0 + max_ratio; // a_n は 1（モニック）

        const NO_SHIFT_ITER: usize = 5;
        const REAL_THRESHOLD: f64 = 1e-10;

        let mut guard = 0usize;
        let mut stagnation = 0usize;
        while p.deg() > 2 {
            let deg_before = p.deg();
            let mut h: Polynomial<Complex<f64>> = p.differentiate();
            for _ in 0..NO_SHIFT_ITER {
                h = Self::calc_h_lambda(&p, &h, &Complex::zero());
            }

            // 受容ベースの反復を導入
            const MAX_TRIES: usize = 5;
            const EPS_ACCEPT: f64 = 1e-8;
            let mut best_s = Complex::new(0.0, 0.0);
            let mut best_res = f64::INFINITY;

            for tri in 0..MAX_TRIES {
                // 候補選択
                let candidates = 12usize;
                let radius = root_bound.clamp(0.5, 10.0) * (1.0 + 0.05 * (tri as f64));
                let mut s = Complex::new(1.0, 0.0);
                let mut best_metric = f64::INFINITY;
                for k in 0..candidates {
                    let theta = 2.0 * std::f64::consts::PI * (k as f64) / (candidates as f64);
                    let s0 = Complex::new(radius * theta.cos(), radius * theta.sin());
                    let pe = p.eval(s0);
                    let he = h.eval(s0);
                    let metric = if he == Complex::new(0.0, 0.0) {
                        f64::INFINITY
                    } else {
                        (pe / he).norm()
                    };
                    if metric < best_metric && metric.is_finite() {
                        best_metric = metric;
                        s = s0;
                    }
                }

                // Step2（控えめ）
                for _ in 0..20 {
                    h = Self::calc_h_lambda(&p, &h, &s);
                    let he = h.eval(s);
                    if he == Complex::new(0.0, 0.0) {
                        break;
                    }
                    let t = s - p.eval(s) / he;
                    if (t - s).norm() < 1e-8 {
                        s = t;
                        break;
                    }
                    s = t;
                }

                // Step3（控えめ）
                for _ in 0..10 {
                    let p_s = p.eval(s);
                    if p_s.norm() < EPS_ACCEPT {
                        break;
                    }
                    h = Self::calc_h_lambda(&p, &h, &s);
                    let he = h.eval(s);
                    if he == Complex::new(0.0, 0.0) {
                        break;
                    }
                    s -= p_s / he;
                }

                // 仕上げのNewton（固定h）
                for _ in 0..3 {
                    let p_s = p.eval(s);
                    if p_s.norm() < EPS_ACCEPT {
                        break;
                    }
                    let he = h.eval(s);
                    if he == Complex::new(0.0, 0.0) {
                        break;
                    }
                    s -= p_s / he;
                }

                let res = p.eval(s).norm();
                if res < best_res {
                    best_res = res;
                    best_s = s;
                }
                if res < EPS_ACCEPT {
                    break;
                }

                // 微小ジッターで再試行
                let jitter = Complex::new(1e-3 * (tri as f64 + 1.0), -1e-3 * (tri as f64 + 1.0));
                s += jitter;
            }

            // 最良推定を受容してデフレーション
            p = Self::deflate_with_estimate(p, best_s, REAL_THRESHOLD, &mut roots);

            guard += 1;
            if guard > 50 {
                break;
            }
            if p.deg() >= deg_before {
                stagnation += 1;
            } else {
                stagnation = 0;
            }
            if stagnation >= 3 {
                break;
            }
        }

        Self::finish_low_degree(p, &mut roots);
        roots
    }

    fn calc_h_lambda(
        p_complex: &Polynomial<Complex<f64>>,
        h_before: &Polynomial<Complex<f64>>,
        shift: &Complex<f64>,
    ) -> Polynomial<Complex<f64>> {
        // 数値安定化: 分母が小さい場合は微小に摂動
        let mut denom = p_complex.eval(*shift);
        if denom == Complex::new(0.0, 0.0) {
            let eps = Complex::new(1e-12, 1e-12);
            denom = p_complex.eval(*shift + eps);
        }
        let ratio = h_before.eval(*shift) / denom;
        let poly_dividend = h_before - &(p_complex * ratio);
        let poly_divisor = Polynomial::from_roots(vec![*shift]);
        &poly_dividend / &poly_divisor
    }
}
