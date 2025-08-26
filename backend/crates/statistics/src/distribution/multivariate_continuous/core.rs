use linalg::{Matrix, Vector};
use rand::Rng;

pub trait MultivariateDistribution {
    type Item;
    fn mean(&self) -> Vector<f64>;
    fn covariance(&self) -> Matrix<f64>;
    fn mode(&self) -> Option<Self::Item>;
    fn pdf(&self, x: &Self::Item) -> f64;
    fn log_pdf(&self, x: &Self::Item) -> f64;
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::Item;
    fn sample_n<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Vec<Self::Item> {
        (0..n).map(|_| self.sample(rng)).collect()
    }
}
