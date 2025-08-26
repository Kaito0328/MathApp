use rand::Rng;

pub trait Distribution {
    /// この分布が生成する値の型（例: usize, u64）
    type Item: Copy + Eq + std::hash::Hash;

    /// 平均値 (f64を返す)
    fn mean(&self) -> f64;
    /// 分散 (f64を返す)
    fn variance(&self) -> f64;
    /// 最頻値 (複数ある場合を考慮してVecで返す)
    fn mode(&self) -> Vec<Self::Item>;

    /// 確率質量関数 (PMF)
    fn pmf(&self, k: Self::Item) -> f64;
    /// 対数確率質量関数
    fn log_pmf(&self, k: Self::Item) -> f64;

    /// 累積分布関数 (CDF)
    fn cdf(&self, k: Self::Item) -> f64;
    /// 分位点関数
    fn quantile(&self, p: f64) -> Self::Item;

    /// サンプルを1つ生成する
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::Item;

    fn skewness(&self) -> Option<f64>;
    fn kurtosis(&self) -> Option<f64>;

    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    // デフォルト実装
    fn sample_n<R: Rng + ?Sized>(&self, n: usize, rng: &mut R) -> Vec<Self::Item> {
        (0..n).map(|_| self.sample(rng)).collect()
    }
}
