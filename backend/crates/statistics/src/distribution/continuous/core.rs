use num_traits::Float;
use rand::Rng;

pub trait Distribution {
    type Item: Float;
    fn mean(&self) -> f64;
    fn variance(&self) -> f64;
    fn mode(&self) -> Vec<Self::Item>;
    fn pdf(&self, x: Self::Item) -> f64;
    fn cdf(&self, x: Self::Item) -> f64;
    fn quantile(&self, p: f64) -> Self::Item;
    fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Self::Item;
    fn log_pdf(&self, x: Self::Item) -> f64;
    fn skewness(&self) -> Option<f64>;
    fn kurtosis(&self) -> Option<f64>;

    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    fn sample_n<R: Rng + ?Sized>(&mut self, n: usize, rng: &mut R) -> Vec<Self::Item> {
        (0..n).map(|_| self.sample(rng)).collect()
    }
}
