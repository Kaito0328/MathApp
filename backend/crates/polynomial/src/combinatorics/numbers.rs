/// 二項係数 C(n, k) を 64bit 浮動小数点で返す。
/// n, k は非負整数。k > n の場合は 0。
pub fn binom(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    if k == 0 || k == n {
        return 1.0;
    }
    let k = k.min(n - k);
    let mut num = 1.0f64;
    let mut den = 1.0f64;
    for i in 1..=k {
        num *= (n - (k - i)) as f64;
        den *= i as f64;
    }
    num / den
}

/// 第2種スターリング数 S(n, k) を動的計画法で計算。
pub fn stirling2(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    if n == 0 && k == 0 {
        return 1.0;
    }
    if n == 0 || k == 0 {
        return 0.0;
    }
    // DP: S(n, k) = k S(n-1, k) + S(n-1, k-1)
    let mut dp = vec![vec![0.0f64; k + 1]; n + 1];
    dp[0][0] = 1.0;
    for i in 1..=n {
        for j in 1..=k {
            dp[i][j] = j as f64 * dp[i - 1][j] + dp[i - 1][j - 1];
        }
    }
    dp[n][k]
}
