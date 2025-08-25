use crate::{
    distribution::{core::Distribution, normal::Normal},
    erf::calc_quantile_acklam,
    gamma::{log_gamma, regularized_gamma},
};

pub struct ChiSquare {
    k: usize,
}

impl ChiSquare {
    pub fn new(k: usize) -> Self {
        assert!(k > 0, "Invalid degrees of freedom parameter");
        Self { k }
    }
}

impl Distribution for ChiSquare {
    fn mean(&self) -> f64 {
        self.k as f64
    }

    fn variance(&self) -> f64 {
        2.0 * self.k as f64
    }

    fn mode(&self) -> Option<f64> {
        if self.k > 2 {
            Some((self.k - 2) as f64)
        } else {
            Some(0.0)
        }
    }

    fn pdf(&self, x: f64) -> f64 {
        self.log_pdf(x).exp()
    }

    fn cdf(&self, x: f64) -> f64 {
        regularized_gamma(self.k as f64 / 2.0, x / 2.0)
    }

    fn quantile(&self, p: f64) -> f64 {
        let z = calc_quantile_acklam(p);
        let x_guess = self.k as f64
            * (1.0 - 2.0 / (9.0 * self.k as f64) + z * (2.0 / (9.0 * self.k as f64)).sqrt())
                .powi(3);

        const MAX_ITER: usize = 100;
        const TOL: f64 = 1e-10;

        let mut x = x_guess.max(0.0);

        for _ in 0..MAX_ITER {
            let fx = self.cdf(x) - p;
            // 終了条件1: 元の関数の値がほぼゼロ
            if fx.abs() < TOL {
                break;
            }
            let dfx = self.pdf(x);
            // ...
            let step = fx / dfx;
            x -= step;
            // 終了条件2: 更新量が非常に小さい
            if step.abs() < TOL {
                break;
            }
        }

        x
    }

    fn log_pdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            f64::NEG_INFINITY
        } else {
            let half_k = self.k as f64 / 2.0;
            (half_k - 1.0) * x.ln() - (x / 2.0) - half_k * 2.0_f64.ln() - log_gamma(half_k)
        }
    }

    fn sample<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) -> f64 {
        let mut normal = Normal::new(0.0, 1.0);

        (0..self.k).map(|_| normal.sample(rng).powi(2)).sum()
    }

    fn skewness(&self) -> Option<f64> {
        Some((8.0 / self.k as f64).sqrt())
    }

    fn kurtosis(&self) -> Option<f64> {
        Some(12.0 / self.k as f64)
    }
}
