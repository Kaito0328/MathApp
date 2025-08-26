use crate::{
    distribution::continuous::{chi_square::ChiSquare, core::Distribution, utils::calc_quantile_newton},
    error::{Result, StatisticsError},
};

use special_functions::{
    beta::{log_beta, regularized_beta},
    erf::calc_quantile_acklam,
};

pub struct F {
    m: usize,
    n: usize,
}

impl F {
    pub fn new(m: usize, n: usize) -> Result<Self> {
        if m == 0 || n == 0 { return Err(StatisticsError::InvalidParameter { what: "F::(m,n)", value: format!("({m},{n})") }); }
        Ok(F { m, n })
    }
}

impl Distribution for F {
    type Item = f64;
    fn mean(&self) -> f64 {
        if self.n <= 2 {
            return f64::NAN;
        }
        self.n as f64 / (self.n as f64 - 2.0)
    }

    fn variance(&self) -> f64 {
        let m = self.m as f64;
        let n = self.n as f64;

        if n < 4.0 {
            return f64::NAN;
        }

        2.0 * n * n * (m + n - 2.0) / (m * (n - 2.0) * (n - 2.0) * (n - 4.0))
    }

    fn mode(&self) -> Vec<Self::Item> {
        if self.m > 2 {
            vec![(self.m as f64 - 2.0) * self.n as f64 / (self.m as f64 * (self.n as f64 + 2.0))]
        } else {
            vec![0.0]
        }
    }

    fn pdf(&self, x: Self::Item) -> f64 {
        self.log_pdf(x).exp()
    }

    fn log_pdf(&self, x: Self::Item) -> f64 {
        let m = self.m as f64;
        let n = self.n as f64;

        if x < 0.0 {
            return f64::NEG_INFINITY;
        }

        0.5 * (m * m.ln() + n * n.ln() + (m - 2.0) * x.ln())
            - log_beta(m * 0.5, n * 0.5)
            - 0.5 * (m + n) * (m * x + n).ln()
    }

    fn cdf(&self, x: Self::Item) -> f64 {
        let m = self.m as f64;
        let n = self.n as f64;

        if x < 0.0 {
            return 0.0;
        }

        regularized_beta(m * 0.5, n * 0.5, m * x / (m * x + n))
    }

    fn quantile(&self, p: f64) -> Self::Item {
        if !(0.0..=1.0).contains(&p) || p.is_nan() {
            return f64::NAN;
        }
        if p == 0.0 {
            return 0.0;
        }
        if p == 1.0 {
            return f64::INFINITY;
        }

        let m = self.m as f64;
        let n = self.n as f64;

        // ステップ1: 対応する標準正規分布の分位点を求める
        let z_p = calc_quantile_acklam(p);

        // ステップ2: フィッシャーのz変換を応用した近似式で初期値を計算
        let mu_z = 0.5 * (1.0 / n - 1.0 / m);
        let sigma_z = (0.5 * (1.0 / m + 1.0 / n)).sqrt();
        let z_guess = mu_z + z_p * sigma_z;
        let x_guess = (2.0 * z_guess).exp();

        // ステップ3: 汎用ニュートン法で精密化
        calc_quantile_newton(x_guess, p, self)
    }

    fn sample<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) -> Self::Item {
    let mut chi_square_m = ChiSquare::new(self.m).unwrap();
    let mut chi_square_n = ChiSquare::new(self.n).unwrap();

        let x = chi_square_m.sample(rng);
        let y = chi_square_n.sample(rng);

        (x / self.m as f64) / (y / self.n as f64)
    }

    fn kurtosis(&self) -> Option<f64> {
        if self.n <= 8 {
            return None;
        }
        let m = self.m as f64;
        let n = self.n as f64;
        let numerator =
            12.0 * (m * (5.0 * n - 22.0) * (m + n - 2.0) + (n - 4.0) * (n - 2.0) * (n - 2.0));
        let denominator = m * (n - 6.0) * (n - 8.0) * (m + n - 2.0);
        if denominator == 0.0 {
            return None;
        }
        Some(numerator / denominator)
    }
    fn skewness(&self) -> Option<f64> {
        if self.n <= 6 {
            return None;
        } // 条件は n > 6

        let m = self.m as f64;
        let n = self.n as f64;

        let numerator = (2.0 * m + n - 2.0) * (8.0 * (n - 4.0)).sqrt();
        let denominator = (n - 6.0) * (m * (m + n - 2.0)).sqrt();

        Some(numerator / denominator)
    }
}
