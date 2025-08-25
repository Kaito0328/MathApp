use num_traits::{Float, FromPrimitive};
use std::iter;

pub fn clean_finite<T: Float + Copy>(xs: &[T]) -> Vec<T> {
    xs.iter().copied().filter(|x| x.is_finite()).collect()
}

// Returns (mean, population variance)
pub fn welford_population_variance<T>(xs: &[T]) -> Option<(T, T)>
where
    T: Float + FromPrimitive + iter::Sum + Copy,
{
    let mut count: usize = 0;
    let mut mean = T::zero();
    let mut m2 = T::zero();
    for x in xs.iter().copied().filter(|v| v.is_finite()) {
        count += 1;
        let delta = x - mean;
        mean = mean + delta / T::from(count).unwrap();
        let delta2 = x - mean;
        m2 = m2 + delta * delta2;
    }
    if count == 0 {
        None
    } else if count == 1 {
        Some((mean, T::zero()))
    } else {
        Some((mean, m2 / T::from(count).unwrap()))
    }
}

// Returns (mean, sample variance)
pub fn welford_sample_variance<T>(xs: &[T]) -> Option<(T, T)>
where
    T: Float + FromPrimitive + iter::Sum + Copy,
{
    let mut count: usize = 0;
    let mut mean = T::zero();
    let mut m2 = T::zero();
    for x in xs.iter().copied().filter(|v| v.is_finite()) {
        count += 1;
        let delta = x - mean;
        mean = mean + delta / T::from(count).unwrap();
        let delta2 = x - mean;
        m2 = m2 + delta * delta2;
    }
    if count < 2 {
        None
    } else {
        Some((mean, m2 / T::from(count - 1).unwrap()))
    }
}

// 通常の線形補間によるパーセンタイル
pub fn percentile_linear<T>(mut xs: Vec<T>, p: f64) -> Option<T>
where
    T: Float + FromPrimitive + Copy,
{
    if !(0.0..=100.0).contains(&p) {
        return None;
    }
    xs.retain(|v| v.is_finite());
    if xs.is_empty() {
        return None;
    }
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let rank = (p / 100.0) * (xs.len().saturating_sub(1)) as f64;
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    if lo == hi {
        Some(xs[lo])
    } else {
        let w = T::from(rank - lo as f64).unwrap();
        Some(xs[lo] * (T::one() - w) + xs[hi] * w)
    }
}
