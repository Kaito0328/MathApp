use special_functions::{beta::regularized_beta, gamma::log_gamma};

use crate::{distribution::discrete::{bernoulli, core::Distribution}, error::{Result, StatisticsError}};

pub struct Binomial {
    n: u64,
    p: f64,
}

impl Binomial {
    pub fn new(n: u64, p: f64) -> Result<Self> {
        if n == 0 || !(0.0..=1.0).contains(&p) || p.is_nan() {
            return Err(StatisticsError::InvalidParameter { what: "Binomial::(n,p)", value: format!("({n},{p})") });
        }
        Ok(Self { n, p })
    }
}

impl Distribution for Binomial {
    type Item = u64;
    fn mean(&self) -> f64 {
        self.n as f64 * self.p
    }

    fn variance(&self) -> f64 {
        self.n as f64 * self.p * (1.0 - self.p)
    }
    fn mode(&self) -> Vec<Self::Item> {
        let mode = (self.n + 1) as f64 * self.p;
        if mode == 0.0 {
            return vec![0];
        } // modeが0なら最頻値は0のみ

        let mode_floor = mode.floor() as Self::Item;
        if mode.fract() == 0.0 && mode_floor > 0 {
            // mode_floor > 0 を追加
            vec![mode_floor - 1, mode_floor] // 順序を昇順に
        } else {
            vec![mode_floor]
        }
    }

    fn pmf(&self, k: Self::Item) -> f64 {
        self.log_pmf(k).exp()
    }

    fn log_pmf(&self, k: Self::Item) -> f64 {
        let n_factorial_ln = log_gamma((self.n + 1) as f64);
        let k_factorial_ln = log_gamma((k + 1) as f64);
        let n_minus_k_factorial_ln = log_gamma((self.n - k + 1) as f64);

        n_factorial_ln - k_factorial_ln - n_minus_k_factorial_ln
            + (k as f64 * self.p.ln())
            + ((self.n - k) as f64 * (1.0 - self.p).ln())
    }

    fn cdf(&self, k: Self::Item) -> f64 {
        regularized_beta((self.n - k) as f64, (k + 1) as f64, 1.0 - self.p)
    }

    fn quantile(&self, p: f64) -> Self::Item {
        if !(0.0..=1.0).contains(&p) {
            return 0;
        } // or panic
        if p == 0.0 {
            return 0;
        }
        if p == 1.0 {
            return self.n;
        }

        // 二分探索の範囲を設定 [low, high]
        let mut low = 0;
        let mut high = self.n;
        let mut result = 0;

        while low <= high {
            let mid = low + (high - low) / 2;
            if self.cdf(mid) >= p {
                // midは候補。さらに小さいkを探す
                result = mid;
                if mid == 0 {
                    break;
                } // midが0ならこれ以上小さいkはない
                high = mid - 1;
            } else {
                // midは小さすぎるので、範囲を右にずらす
                low = mid + 1;
            }
        }
        result
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::Item {
    let bernoulli = bernoulli::Bernoulli::new(self.p).unwrap();
        let mut k = 0;
        for _ in 0..self.n {
            if bernoulli.sample(rng) == 1 {
                k += 1;
            }
        }

        k
    }

    fn skewness(&self) -> Option<f64> {
        if self.n == 0 || self.p == 0.0 || self.p == 1.0 {
            None
        } else {
            Some((1.0 - 2.0 * self.p) / ((self.n as f64 * self.p * (1.0 - self.p)).sqrt()))
        }
    }

    fn kurtosis(&self) -> Option<f64> {
        if self.n == 0 || self.p == 0.0 || self.p == 1.0 {
            None
        } else {
            Some((1.0 - 6.0 * self.p * (1.0 - self.p)) / (self.n as f64 * self.p * (1.0 - self.p)))
        }
    }
}
