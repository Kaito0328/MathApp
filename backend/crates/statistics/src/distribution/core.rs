use rand::Rng;

pub trait Distribution {
    fn mean(&self) -> f64;
    fn variance(&self) -> f64;
    fn mode(&self) -> Option<f64>;

    fn pdf(&self, x: f64) -> f64;
    fn cdf(&self, x: f64) -> f64;
    fn quantile(&self, p: f64) -> f64;
    fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> f64;
    fn log_pdf(&self, x: f64) -> f64;
    fn skewness(&self) -> Option<f64>;
    fn kurtosis(&self) -> Option<f64>;

    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    fn sample_n<R: Rng + ?Sized>(&mut self, n: usize, rng: &mut R) -> Vec<f64> {
        (0..n).map(|_| self.sample(rng)).collect()
    }
}
