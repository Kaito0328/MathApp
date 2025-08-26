use crate::common::{clean_finite, welford_sample_variance};
use crate::distribution::continuous::{
    chi_square::ChiSquare, core::Distribution, f::F, normal::Normal, t::T,
};
use crate::error::{Result, StatisticsError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tail {
    TwoSided,
    Less,
    Greater,
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub stat: f64,
    pub p_value: f64,
    pub df1: Option<f64>,
    pub df2: Option<f64>,
    pub ci: Option<(f64, f64)>,
    pub effect: Option<f64>,
    pub n1: usize,
    pub n2: Option<usize>,
    pub method: &'static str,
    pub tail: Tail,
}

fn tail_p(p_left: f64, p_right: f64, tail: Tail) -> f64 {
    match tail {
        Tail::Less => p_left,
        Tail::Greater => p_right,
        Tail::TwoSided => (2.0 * p_left.min(p_right)).min(1.0),
    }
}

fn mean_var(xs: &[f64]) -> Result<(f64, f64)> {
    let clean = clean_finite(xs);
    if clean.len() < 2 {
        return Err(StatisticsError::EmptyInput);
    }
    let n = clean.len() as f64;
    let mean = clean.iter().sum::<f64>() / n;
    let (_, s2) = welford_sample_variance(&clean).ok_or(StatisticsError::DomainError {
        what: "variance",
        details: "failed",
    })?;
    if !s2.is_finite() {
        return Err(StatisticsError::DomainError {
            what: "variance",
            details: "non-finite",
        });
    }
    Ok((mean, s2))
}

pub fn one_sample_t(xs: &[f64], mu0: f64, tail: Tail, alpha: Option<f64>) -> Result<TestResult> {
    let (mean, s2) = mean_var(xs)?;
    let n = clean_finite(xs).len();
    let se = (s2 / n as f64).sqrt();
    if se == 0.0 {
        return Err(StatisticsError::DomainError {
            what: "one_sample_t",
            details: "zero standard error",
        });
    }
    let t = (mean - mu0) / se;
    let df = (n - 1) as f64;
    let tdist = T::new(n).unwrap();
    let p_left = tdist.cdf(t);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);

    let ci = if let Some(a) = alpha {
        if a > 0.0 && a < 1.0 {
            let q = 1.0 - 0.5 * a;
            let crit = tdist.quantile(q).abs();
            let half = crit * se;
            Some((mean - half, mean + half))
        } else {
            None
        }
    } else {
        None
    };

    let effect = Some((mean - mu0) / s2.sqrt());
    Ok(TestResult {
        stat: t,
        p_value: p,
        df1: Some(df),
        df2: None,
        ci,
        effect,
        n1: n,
        n2: None,
        method: "one-sample t-test",
        tail,
    })
}

pub fn two_sample_t_welch(
    x: &[f64],
    y: &[f64],
    tail: Tail,
    alpha: Option<f64>,
) -> Result<TestResult> {
    let (mx, sx2) = mean_var(x)?;
    let (my, sy2) = mean_var(y)?;
    let nx = clean_finite(x).len();
    let ny = clean_finite(y).len();
    let se2 = sx2 / nx as f64 + sy2 / ny as f64;
    let se = se2.sqrt();
    if se == 0.0 {
        return Err(StatisticsError::DomainError {
            what: "two_sample_t_welch",
            details: "zero standard error",
        });
    }
    let t = (mx - my) / se;
    let df = se2 * se2
        / ((sx2 / nx as f64).powi(2) / (nx as f64 - 1.0)
            + (sy2 / ny as f64).powi(2) / (ny as f64 - 1.0));
    let tdist = T::new(df.round() as usize).unwrap();
    let p_left = tdist.cdf(t);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);
    let ci = if let Some(a) = alpha {
        if a > 0.0 && a < 1.0 {
            let q = 1.0 - 0.5 * a;
            let crit = tdist.quantile(q).abs();
            let half = crit * se;
            Some(((mx - my) - half, (mx - my) + half))
        } else {
            None
        }
    } else {
        None
    };
    let effect = Some((mx - my) / ((sx2 + sy2) / 2.0).sqrt());
    Ok(TestResult {
        stat: t,
        p_value: p,
        df1: Some(df),
        df2: None,
        ci,
        effect,
        n1: nx,
        n2: Some(ny),
        method: "Welch two-sample t-test",
        tail,
    })
}

pub fn two_sample_t_pooled(
    x: &[f64],
    y: &[f64],
    tail: Tail,
    alpha: Option<f64>,
) -> Result<TestResult> {
    let (mx, sx2) = mean_var(x)?;
    let (my, sy2) = mean_var(y)?;
    let nx = clean_finite(x).len();
    let ny = clean_finite(y).len();
    let df = (nx + ny - 2) as f64;
    let sp2 = (((nx - 1) as f64) * sx2 + ((ny - 1) as f64) * sy2) / df;
    let se = (sp2 * (1.0 / nx as f64 + 1.0 / ny as f64)).sqrt();
    if se == 0.0 {
        return Err(StatisticsError::DomainError {
            what: "two_sample_t_pooled",
            details: "zero standard error",
        });
    }
    let t = (mx - my) / se;
    let tdist = T::new(nx + ny - 2).unwrap();
    let p_left = tdist.cdf(t);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);
    let ci = if let Some(a) = alpha {
        if a > 0.0 && a < 1.0 {
            let q = 1.0 - 0.5 * a;
            let crit = tdist.quantile(q).abs();
            let half = crit * se;
            Some(((mx - my) - half, (mx - my) + half))
        } else {
            None
        }
    } else {
        None
    };
    let effect = Some((mx - my) / sp2.sqrt());
    Ok(TestResult {
        stat: t,
        p_value: p,
        df1: Some(df),
        df2: None,
        ci,
        effect,
        n1: nx,
        n2: Some(ny),
        method: "Pooled two-sample t-test",
        tail,
    })
}

pub fn f_test_variance_ratio(
    x: &[f64],
    y: &[f64],
    tail: Tail,
    alpha: Option<f64>,
) -> Result<TestResult> {
    let (_, sx2) = mean_var(x)?;
    let (_, sy2) = mean_var(y)?;
    let nx = clean_finite(x).len();
    let ny = clean_finite(y).len();
    let f_stat = sx2 / sy2;
    let d1 = (nx - 1) as usize;
    let d2 = (ny - 1) as usize;
    let fdist = F::new(d1, d2).unwrap();
    let p_left = fdist.cdf(f_stat);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);
    let ci = if let Some(a) = alpha {
        if a > 0.0 && a < 1.0 {
            let lower_q = fdist.quantile(a / 2.0);
            let upper_q = fdist.quantile(1.0 - a / 2.0);
            Some((sx2 / upper_q, sx2 / lower_q))
        } else {
            None
        }
    } else {
        None
    };
    Ok(TestResult {
        stat: f_stat,
        p_value: p,
        df1: Some(d1 as f64),
        df2: Some(d2 as f64),
        ci,
        effect: None,
        n1: nx,
        n2: Some(ny),
        method: "F test variance ratio",
        tail,
    })
}

pub fn chisq_gof(obs: &[f64], exp: &[f64], tail: Tail) -> Result<TestResult> {
    if obs.len() != exp.len() || obs.is_empty() {
        return Err(StatisticsError::EmptyInput);
    }
    if !exp.iter().all(|&e| e.is_finite() && e > 0.0) {
        return Err(StatisticsError::InvalidParameter {
            what: "chisq_gof::exp",
            value: format!("{exp:?}"),
        });
    }
    if !obs.iter().all(|&o| o.is_finite() && o >= 0.0) {
        return Err(StatisticsError::InvalidParameter {
            what: "chisq_gof::obs",
            value: format!("{obs:?}"),
        });
    }
    let stat: f64 = obs
        .iter()
        .zip(exp.iter())
        .map(|(o, e)| {
            let d = o - e;
            d * d / e
        })
        .sum();
    let df = (obs.len() - 1) as f64; // 注: 推定パラメータは考慮していない
    let chisq = ChiSquare::new(df as usize).unwrap();
    let p_left = chisq.cdf(stat);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);
    Ok(TestResult {
        stat,
        p_value: p,
        df1: Some(df),
        df2: None,
        ci: None,
        effect: None,
        n1: obs.len(),
        n2: None,
        method: "Chi-square goodness-of-fit",
        tail,
    })
}

/// カイ二乗独立性検定（R×C 交差表）。
pub fn chisq_independence(table: &[&[u64]], tail: Tail) -> Result<TestResult> {
    let r = table.len();
    if r < 2 {
        return Err(StatisticsError::InvalidParameter {
            what: "chisq_independence::rows",
            value: r.to_string(),
        });
    }
    let c = table[0].len();
    if c < 2 || !table.iter().all(|row| row.len() == c) {
        return Err(StatisticsError::InvalidParameter {
            what: "chisq_independence::cols",
            value: c.to_string(),
        });
    }
    let mut row_sum = vec![0f64; r];
    let mut col_sum = vec![0f64; c];
    let mut grand: f64 = 0.0;
    for (i, row) in table.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            let vf = v as f64;
            row_sum[i] += vf;
            col_sum[j] += vf;
            grand += vf;
        }
    }
    if grand == 0.0 {
        return Err(StatisticsError::EmptyInput);
    }
    // 統計量
    let mut stat = 0.0;
    for i in 0..r {
        for j in 0..c {
            let e = row_sum[i] * col_sum[j] / grand;
            if e <= 0.0 {
                return Err(StatisticsError::DomainError {
                    what: "chisq_independence",
                    details: "expected cell <= 0",
                });
            }
            let o = table[i][j] as f64;
            let d = o - e;
            stat += d * d / e;
        }
    }
    let df1 = ((r - 1) * (c - 1)) as f64;
    let chisq = ChiSquare::new(df1 as usize).unwrap();
    let p_left = chisq.cdf(stat);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);
    Ok(TestResult {
        stat,
        p_value: p,
        df1: Some(df1),
        df2: None,
        ci: None,
        effect: None,
        n1: r,
        n2: Some(c),
        method: "Chi-square independence test",
        tail,
    })
}

pub fn z_test_proportion(
    successes: u64,
    n: u64,
    p0: f64,
    tail: Tail,
    alpha: Option<f64>,
) -> Result<TestResult> {
    if n == 0 {
        return Err(StatisticsError::EmptyInput);
    }
    if !(0.0..=1.0).contains(&p0) || p0.is_nan() {
        return Err(StatisticsError::InvalidParameter {
            what: "z_test_proportion::p0",
            value: p0.to_string(),
        });
    }
    if successes > n {
        return Err(StatisticsError::InvalidParameter {
            what: "z_test_proportion::successes",
            value: successes.to_string(),
        });
    }
    let phat = successes as f64 / n as f64;
    let se = (p0 * (1.0 - p0) / n as f64).sqrt();
    if se == 0.0 {
        return Err(StatisticsError::DomainError {
            what: "z_test_proportion",
            details: "zero standard error",
        });
    }
    let z = (phat - p0) / se;
    let std_norm = Normal::new(0.0, 1.0).unwrap();
    let p_left = std_norm.cdf(z);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);
    let ci = if let Some(a) = alpha {
        if a > 0.0 && a < 1.0 {
            let crit = Normal::new(0.0, 1.0).unwrap().quantile(1.0 - 0.5 * a).abs();
            let half = crit * (phat * (1.0 - phat) / n as f64).sqrt();
            Some(((phat - half).max(0.0), (phat + half).min(1.0)))
        } else {
            None
        }
    } else {
        None
    };
    Ok(TestResult {
        stat: z,
        p_value: p,
        df1: None,
        df2: None,
        ci,
        effect: None,
        n1: n as usize,
        n2: None,
        method: "One-sample proportion z-test",
        tail,
    })
}

/// 二標本の比率Z検定（差 p1-p2 = 0 の検定）
pub fn z_test_two_proportions(
    x1: u64,
    n1: u64,
    x2: u64,
    n2: u64,
    tail: Tail,
    alpha: Option<f64>,
) -> Result<TestResult> {
    if n1 == 0 || n2 == 0 {
        return Err(StatisticsError::EmptyInput);
    }
    if x1 > n1 || x2 > n2 {
        return Err(StatisticsError::InvalidParameter {
            what: "z_test_two_proportions::counts",
            value: format!("({x1}/{n1},{x2}/{n2})"),
        });
    }
    let p1 = x1 as f64 / n1 as f64;
    let p2 = x2 as f64 / n2 as f64;
    let p_pool = (x1 + x2) as f64 / (n1 + n2) as f64;
    let se0 = (p_pool * (1.0 - p_pool) * (1.0 / n1 as f64 + 1.0 / n2 as f64)).sqrt();
    if se0 == 0.0 {
        return Err(StatisticsError::DomainError {
            what: "z_test_two_proportions",
            details: "zero standard error",
        });
    }
    let z = (p1 - p2) / se0;
    let std_norm = Normal::new(0.0, 1.0).unwrap();
    let p_left = std_norm.cdf(z);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);
    // Wald CI for difference
    let ci = if let Some(a) = alpha {
        if a > 0.0 && a < 1.0 {
            let crit = std_norm.quantile(1.0 - 0.5 * a).abs();
            let se = (p1 * (1.0 - p1) / n1 as f64 + p2 * (1.0 - p2) / n2 as f64).sqrt();
            let half = crit * se;
            Some(((p1 - p2 - half).max(-1.0), (p1 - p2 + half).min(1.0)))
        } else {
            None
        }
    } else {
        None
    };
    Ok(TestResult {
        stat: z,
        p_value: p,
        df1: None,
        df2: None,
        ci,
        effect: Some(p1 - p2),
        n1: n1 as usize,
        n2: Some(n2 as usize),
        method: "Two-sample proportion z-test",
        tail,
    })
}

// === Helpers for ranking (average ranks with ties) ===
fn average_ranks(values: &[(f64, usize)]) -> Vec<f64> {
    // values: (value, original_index), returns ranks aligned to original order
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let n = v.len();
    let mut ranks = vec![0.0; n];
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && v[j].0 == v[i].0 {
            j += 1;
        }
        let start = i as f64 + 1.0;
        let end = j as f64;
        let avg = (start + end) / 2.0;
        for k in i..j {
            ranks[v[k].1] = avg;
        }
        i = j;
    }
    ranks
}

// Tie correction factor for ranks: 1 - sum(t^3 - t) / (N^3 - N)
fn tie_correction(counts: &[usize], n: usize) -> f64 {
    if n < 2 {
        return 1.0;
    }
    let denom = (n as f64).powi(3) - n as f64;
    if denom <= 0.0 {
        return 1.0;
    }
    let sum = counts
        .iter()
        .filter(|&&t| t > 1)
        .map(|&t| {
            let tf = t as f64;
            tf * tf * tf - tf
        })
        .sum::<f64>();
    1.0 - sum / denom
}

fn tie_group_counts(sorted_values: &[f64]) -> Vec<usize> {
    let mut counts = Vec::new();
    let mut i = 0;
    while i < sorted_values.len() {
        let mut j = i + 1;
        while j < sorted_values.len() && sorted_values[j] == sorted_values[i] {
            j += 1;
        }
        counts.push(j - i);
        i = j;
    }
    counts
}

/// 一元配置分散分析（between-groups ANOVA）
pub fn one_way_anova(groups: &[&[f64]], tail: Tail) -> Result<TestResult> {
    let k = groups.len();
    if k < 2 {
        return Err(StatisticsError::InvalidParameter {
            what: "one_way_anova::k",
            value: k.to_string(),
        });
    }
    let mut ns = Vec::with_capacity(k);
    let mut means = Vec::with_capacity(k);
    let mut ss_within = 0.0f64;
    let mut n_total = 0usize;
    for g in groups {
        let clean = clean_finite(g);
        if clean.len() < 2 {
            return Err(StatisticsError::InvalidParameter {
                what: "one_way_anova::group_size",
                value: clean.len().to_string(),
            });
        }
        let n = clean.len();
        let mean = clean.iter().sum::<f64>() / n as f64;
        let (_, s2) = welford_sample_variance(&clean).ok_or(StatisticsError::DomainError {
            what: "variance",
            details: "failed",
        })?;
        ns.push(n);
        means.push(mean);
        ss_within += s2 * (n as f64 - 1.0);
        n_total += n;
    }
    let grand_mean = {
        let mut s = 0.0;
        for (i, &n) in ns.iter().enumerate() {
            s += means[i] * n as f64;
        }
        s / n_total as f64
    };
    let mut ss_between = 0.0f64;
    for (i, &n) in ns.iter().enumerate() {
        let d = means[i] - grand_mean;
        ss_between += n as f64 * d * d;
    }
    let df_between = (k - 1) as f64;
    let df_within = (n_total - k) as f64;
    if df_within <= 0.0 {
        return Err(StatisticsError::DomainError {
            what: "one_way_anova",
            details: "df_within <= 0",
        });
    }
    let ms_between = ss_between / df_between.max(1.0);
    let ms_within = ss_within / df_within;
    if ms_within == 0.0 {
        return Err(StatisticsError::DomainError {
            what: "one_way_anova",
            details: "MS_within = 0",
        });
    }
    let f_stat = ms_between / ms_within;
    let fdist = F::new(df_between as usize, df_within as usize).unwrap();
    let p_left = fdist.cdf(f_stat);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);
    // Effect size: eta^2
    let eta2 = if (ss_between + ss_within) > 0.0 {
        Some(ss_between / (ss_between + ss_within))
    } else {
        None
    };
    Ok(TestResult {
        stat: f_stat,
        p_value: p,
        df1: Some(df_between),
        df2: Some(df_within),
        ci: None,
        effect: eta2,
        n1: n_total,
        n2: Some(k),
        method: "One-way ANOVA",
        tail,
    })
}

/// ピアソン相関の検定（t分布）。rのFisher変換でCI。
pub fn correlation_t_test(
    x: &[f64],
    y: &[f64],
    tail: Tail,
    alpha: Option<f64>,
) -> Result<TestResult> {
    if x.len() != y.len() || x.len() < 3 {
        return Err(StatisticsError::InvalidParameter {
            what: "correlation::n",
            value: x.len().to_string(),
        });
    }
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (&a, &b) in x.iter().zip(y.iter()) {
        if a.is_finite() && b.is_finite() {
            xs.push(a);
            ys.push(b);
        }
    }
    let n = xs.len();
    if n < 3 {
        return Err(StatisticsError::EmptyInput);
    }
    let mean_x = xs.iter().sum::<f64>() / n as f64;
    let mean_y = ys.iter().sum::<f64>() / n as f64;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    let mut sxy = 0.0;
    for i in 0..n {
        let dx = xs[i] - mean_x;
        let dy = ys[i] - mean_y;
        sxx += dx * dx;
        syy += dy * dy;
        sxy += dx * dy;
    }
    if sxx == 0.0 || syy == 0.0 {
        return Err(StatisticsError::DomainError {
            what: "correlation",
            details: "zero variance",
        });
    }
    let r = sxy / (sxx.sqrt() * syy.sqrt());
    let df = (n - 2) as f64;
    let t = r * (df / (1.0 - r * r).max(1e-16)).sqrt();
    let tdist = T::new(n - 2).unwrap();
    let p_left = tdist.cdf(t);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);
    // Fisher z CI for r
    let ci = if let Some(a) = alpha {
        if a > 0.0 && a < 1.0 && n > 3 {
            let z = 0.5 * ((1.0 + r) / (1.0 - r)).ln();
            let se = 1.0 / (n as f64 - 3.0).sqrt();
            let zcrit = Normal::new(0.0, 1.0).unwrap().quantile(1.0 - 0.5 * a).abs();
            let zlo = z - zcrit * se;
            let zhi = z + zcrit * se;
            let rlo = (zlo.tanh()).max(-1.0);
            let rhi = (zhi.tanh()).min(1.0);
            Some((rlo, rhi))
        } else {
            None
        }
    } else {
        None
    };
    Ok(TestResult {
        stat: t,
        p_value: p,
        df1: Some(df),
        df2: None,
        ci,
        effect: Some(r),
        n1: n,
        n2: None,
        method: "Pearson correlation t-test",
        tail,
    })
}

/// Mann-Whitney U検定（独立2群）。正規近似と連続性補正（デフォルトあり）。
pub fn mann_whitney_u(x: &[f64], y: &[f64], tail: Tail, continuity: bool) -> Result<TestResult> {
    let xs = clean_finite(x);
    let ys = clean_finite(y);
    let n1 = xs.len();
    let n2 = ys.len();
    if n1 == 0 || n2 == 0 {
        return Err(StatisticsError::EmptyInput);
    }
    let mut all = Vec::with_capacity(n1 + n2);
    for (i, &v) in xs.iter().enumerate() {
        all.push((v, i, 0));
    }
    for (j, &v) in ys.iter().enumerate() {
        all.push((v, j, 1));
    }
    // ranks over combined
    // For ranks we need original order within each group; we will compute full ranks and then sum per group by mapping back via indices with group flag.
    // Build order mapping for combined
    let combined: Vec<(f64, usize)> = all
        .iter()
        .enumerate()
        .map(|(i, (v, _, _))| (*v, i))
        .collect();
    let ranks_combined = average_ranks(&combined);
    let mut r1 = 0.0;
    let mut r2 = 0.0;
    for (i, (_v, _idx, g)) in all.iter().enumerate() {
        if *g == 0 {
            r1 += ranks_combined[i];
        } else {
            r2 += ranks_combined[i];
        }
    }
    let u1 = r1 - (n1 as f64) * (n1 as f64 + 1.0) / 2.0;
    let u2 = r2 - (n2 as f64) * (n2 as f64 + 1.0) / 2.0;
    // Tie correction for variance
    let mut sorted_vals: Vec<f64> = all.iter().map(|(v, _, _)| *v).collect();
    sorted_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let counts = tie_group_counts(&sorted_vals);
    let n = n1 + n2;
    let tie_c = tie_correction(&counts, n);
    let mu = (n1 as f64) * (n2 as f64) / 2.0;
    let sigma = ((n1 as f64) * (n2 as f64) * ((n + 1) as f64) / 12.0 * tie_c).sqrt();
    let cc = if continuity { 0.5 } else { 0.0 };
    let z = if u1 < u2 {
        (u1 + cc - mu) / sigma
    } else {
        (u2 + cc - mu) / sigma
    };
    let std_norm = Normal::new(0.0, 1.0).unwrap();
    let p_left = std_norm.cdf(z);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);
    let effect_r = Some((z.abs()) / (n as f64).sqrt());
    Ok(TestResult {
        stat: z,
        p_value: p,
        df1: None,
        df2: None,
        ci: None,
        effect: effect_r,
        n1,
        n2: Some(n2),
        method: "Mann-Whitney U test (normal approx)",
        tail,
    })
}

/// Wilcoxonの符号付順位和検定（対応のある2群）。
pub fn wilcoxon_signed_rank(
    x: &[f64],
    y: &[f64],
    tail: Tail,
    continuity: bool,
) -> Result<TestResult> {
    if x.len() != y.len() || x.is_empty() {
        return Err(StatisticsError::InvalidParameter {
            what: "wilcoxon::n",
            value: x.len().to_string(),
        });
    }
    // differences excluding zeros
    let diffs: Vec<f64> = x
        .iter()
        .zip(y.iter())
        .filter_map(|(&a, &b)| {
            if a.is_finite() && b.is_finite() {
                let d = a - b;
                if d == 0.0 {
                    None
                } else {
                    Some(d)
                }
            } else {
                None
            }
        })
        .collect();
    let n = diffs.len();
    if n == 0 {
        return Err(StatisticsError::EmptyInput);
    }
    // ranks of |diff|
    let abs_with_idx: Vec<(f64, usize)> = diffs
        .iter()
        .enumerate()
        .map(|(i, d)| (d.abs(), i))
        .collect();
    let ranks = average_ranks(&abs_with_idx);
    let mut w_plus = 0.0;
    let mut w_minus = 0.0;
    for (i, d) in diffs.iter().enumerate() {
        if *d > 0.0 {
            w_plus += ranks[i];
        } else {
            w_minus += ranks[i];
        }
    }
    let w = w_plus.min(w_minus);
    let mu = n as f64 * (n as f64 + 1.0) / 4.0;
    // tie correction for |diff| ties
    let mut sorted_abs: Vec<f64> = diffs.iter().map(|d| d.abs()).collect();
    sorted_abs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let counts = tie_group_counts(&sorted_abs);
    let tie_adj: f64 = counts
        .iter()
        .filter(|&&t| t > 1)
        .map(|&t| {
            let tf = t as f64;
            (tf * tf * tf - tf) / 48.0
        })
        .sum();
    let sigma2 = n as f64 * (n as f64 + 1.0) * (2.0 * n as f64 + 1.0) / 24.0 - tie_adj;
    let sigma = sigma2.sqrt();
    let cc = if continuity { 0.5 } else { 0.0 };
    let z = (w - mu + cc) / sigma; // use +cc so that small-sample continuity matches typical convention for two-sided
    let std_norm = Normal::new(0.0, 1.0).unwrap();
    let p_left = std_norm.cdf(z);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);
    let effect_r = Some(z.abs() / (n as f64).sqrt());
    Ok(TestResult {
        stat: z,
        p_value: p,
        df1: None,
        df2: None,
        ci: None,
        effect: effect_r,
        n1: n,
        n2: None,
        method: "Wilcoxon signed-rank test (normal approx)",
        tail,
    })
}

/// Kruskal–Wallis検定（一元配置のノンパラ）。
pub fn kruskal_wallis(groups: &[&[f64]], tail: Tail) -> Result<TestResult> {
    let k = groups.len();
    if k < 2 {
        return Err(StatisticsError::InvalidParameter {
            what: "kruskal_wallis::k",
            value: k.to_string(),
        });
    }
    let mut clean_groups: Vec<Vec<f64>> = Vec::with_capacity(k);
    let mut n_total = 0usize;
    for g in groups {
        let c = clean_finite(g);
        if c.is_empty() {
            return Err(StatisticsError::EmptyInput);
        }
        n_total += c.len();
        clean_groups.push(c);
    }
    // Combine and rank
    let mut combined: Vec<(f64, usize)> = Vec::with_capacity(n_total);
    for (gi, g) in clean_groups.iter().enumerate() {
        for &v in g {
            combined.push((v, gi));
        }
    }
    let mut values_only: Vec<f64> = combined.iter().map(|(v, _)| *v).collect();
    let ranks = average_ranks(
        &combined
            .iter()
            .enumerate()
            .map(|(i, (v, _))| (*v, i))
            .collect::<Vec<_>>(),
    );
    // Sum ranks per group
    let mut rsum = vec![0.0f64; k];
    for (i, (_v, gi)) in combined.iter().enumerate() {
        rsum[*gi] += ranks[i];
    }
    // H statistic
    let n = n_total as f64;
    let mut h = 0.0f64;
    for (gi, g) in clean_groups.iter().enumerate() {
        let ni = g.len() as f64;
        let ri = rsum[gi];
        h += ri * ri / ni;
    }
    h = 12.0 / (n * (n + 1.0)) * h - 3.0 * (n + 1.0);
    // Tie correction
    values_only.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let counts = tie_group_counts(&values_only);
    let c = tie_correction(&counts, n_total);
    let hc = if c > 0.0 { h / c } else { h };
    let df = (k - 1) as f64;
    let chisq = ChiSquare::new(df as usize).unwrap();
    let p_left = chisq.cdf(hc);
    let p_right = 1.0 - p_left;
    let p = tail_p(p_left, p_right, tail);
    // Effect size: epsilon^2
    let effect = Some(((hc - (k as f64 - 1.0)) / (n - k as f64)).max(0.0));
    Ok(TestResult {
        stat: hc,
        p_value: p,
        df1: Some(df),
        df2: None,
        ci: None,
        effect,
        n1: n_total,
        n2: Some(k),
        method: "Kruskal-Wallis H test",
        tail,
    })
}
