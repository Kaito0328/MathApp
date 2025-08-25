use crate::common::{
    clean_finite, percentile_linear, welford_population_variance, welford_sample_variance,
};
use num_traits::{Float, FromPrimitive};
use std::iter;

// 連続データ統計のトレイト（後方互換のため名称は維持）
pub trait Stats<T> {
    fn mean(&self) -> Option<T>;
    fn median(&self) -> Option<T>;
    fn mode(&self) -> Option<Vec<T>>;
    fn variance(&self) -> Option<T>;
    fn unbiased_variance(&self) -> Option<T>;
    fn standard_deviation(&self) -> Option<T>;
    fn unbiased_standard_deviation(&self) -> Option<T>;
    fn range(&self) -> Option<T>;
    fn quartiles(&self) -> (Option<T>, Option<T>);
    fn iqr(&self) -> Option<T>;
    fn skewness(&self) -> Option<T>;
    fn kurtosis(&self) -> Option<T>;
    fn percentiles(&self, p: f64) -> Option<T>;
    fn covariance(&self, other: &Self) -> Option<T>;
    fn correlation_coefficient(&self, other: &Self) -> Option<T>;
}

impl<T> Stats<T> for [T]
where
    T: Float + iter::Sum + FromPrimitive,
{
    fn mean(&self) -> Option<T> {
        let clean = clean_finite(self);
        if clean.is_empty() {
            return None;
        }
        let len = T::from(clean.len()).unwrap();
        Some(clean.into_iter().sum::<T>() / len)
    }

    fn median(&self) -> Option<T> {
        let mut clean = clean_finite(self);
        if clean.is_empty() {
            return None;
        }
        clean.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let len = clean.len();
        if len % 2 == 0 {
            Some((clean[len / 2 - 1] + clean[len / 2]) / T::from(2).unwrap())
        } else {
            Some(clean[len / 2])
        }
    }

    fn mode(&self) -> Option<Vec<T>> {
        let mut clean = clean_finite(self);
        if clean.is_empty() {
            return None;
        }
        clean.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = clean.len();
        let mut modes = Vec::new();
        let mut max_count = 0usize;
        let mut i = 0usize;
        while i < n {
            let mut count = 1usize;
            let mut j = i + 1;
            while j < n && clean[j] == clean[i] {
                count += 1;
                j += 1;
            }
            if count > max_count {
                max_count = count;
                modes.clear();
                modes.push(clean[i]);
            } else if count == max_count {
                modes.push(clean[i]);
            }
            i = j;
        }
        if max_count <= 1 {
            Some(vec![])
        } else {
            Some(modes)
        }
    }

    fn variance(&self) -> Option<T> {
        let clean = clean_finite(self);
        if clean.len() < 2 {
            return None;
        }
        welford_population_variance(&clean).map(|(_, v)| v)
    }

    fn unbiased_variance(&self) -> Option<T> {
        let clean = clean_finite(self);
        if clean.len() < 2 {
            return None;
        }
        welford_sample_variance(&clean).map(|(_, v)| v)
    }

    fn standard_deviation(&self) -> Option<T> {
        self.variance().map(|v| v.sqrt())
    }
    fn unbiased_standard_deviation(&self) -> Option<T> {
        self.unbiased_variance().map(|v| v.sqrt())
    }

    fn range(&self) -> Option<T> {
        let mut it = self.iter().copied().filter(|v| v.is_finite());
        let first = it.next()?;
        let (min, max) = it.fold((first, first), |(mn, mx), x| (mn.min(x), mx.max(x)));
        Some(max - min)
    }

    fn quartiles(&self) -> (Option<T>, Option<T>) {
        (self.percentiles(25.0), self.percentiles(75.0))
    }
    fn iqr(&self) -> Option<T> {
        let (q1, q3) = self.quartiles();
        Some(q3? - q1?)
    }

    fn skewness(&self) -> Option<T> {
        let clean = clean_finite(self);
        if clean.is_empty() {
            return None;
        }
        let n = T::from(clean.len()).unwrap();
        let mean = clean.iter().copied().sum::<T>() / n;
        let sd = welford_population_variance(&clean).map(|(_, v)| v.sqrt())?;
        if sd <= T::epsilon() {
            return None;
        }
        let m3: T = clean.iter().map(|&x| (x - mean).powi(3)).sum::<T>() / n;
        Some(m3 / sd.powi(3))
    }

    fn kurtosis(&self) -> Option<T> {
        let clean = clean_finite(self);
        if clean.is_empty() {
            return None;
        }
        let n = T::from(clean.len()).unwrap();
        let mean = clean.iter().copied().sum::<T>() / n;
        let sd = welford_population_variance(&clean).map(|(_, v)| v.sqrt())?;
        if sd <= T::epsilon() {
            return None;
        }
        let m4: T = clean.iter().map(|&x| (x - mean).powi(4)).sum::<T>() / n;
        Some(m4 / sd.powi(4) - T::from(3.0).unwrap())
    }

    fn percentiles(&self, p: f64) -> Option<T> {
        percentile_linear(clean_finite(self), p)
    }

    fn covariance(&self, other: &Self) -> Option<T> {
        if self.len() != other.len() || self.is_empty() {
            return None;
        }
        let pairs: Vec<(T, T)> = self
            .iter()
            .copied()
            .zip(other.iter().copied())
            .filter(|(x, y)| x.is_finite() && y.is_finite())
            .collect();
        if pairs.is_empty() {
            return None;
        }
        let n = T::from(pairs.len()).unwrap();
        let mean_x = pairs.iter().map(|(x, _)| *x).sum::<T>() / n;
        let mean_y = pairs.iter().map(|(_, y)| *y).sum::<T>() / n;
        let s: T = pairs
            .iter()
            .map(|(x, y)| (*x - mean_x) * (*y - mean_y))
            .sum();
        Some(s / n)
    }

    fn correlation_coefficient(&self, other: &Self) -> Option<T> {
        if self.len() != other.len() || self.is_empty() {
            return None;
        }
        let pairs: Vec<(T, T)> = self
            .iter()
            .copied()
            .zip(other.iter().copied())
            .filter(|(x, y)| x.is_finite() && y.is_finite())
            .collect();
        if pairs.len() < 2 {
            return None;
        }
        let xs: Vec<T> = pairs.iter().map(|(x, _)| *x).collect();
        let ys: Vec<T> = pairs.iter().map(|(_, y)| *y).collect();
        let cov = xs.as_slice().covariance(ys.as_slice())?;
        let sx = xs.as_slice().standard_deviation()?;
        let sy = ys.as_slice().standard_deviation()?;
        if sx <= T::epsilon() || sy <= T::epsilon() {
            return None;
        }
        Some(cov / (sx * sy))
    }
}

// Vec<T> はスライス実装に委譲
impl<T> Stats<T> for Vec<T>
where
    T: Float + iter::Sum + FromPrimitive,
{
    fn mean(&self) -> Option<T> {
        self.as_slice().mean()
    }
    fn median(&self) -> Option<T> {
        self.as_slice().median()
    }
    fn mode(&self) -> Option<Vec<T>> {
        self.as_slice().mode()
    }
    fn variance(&self) -> Option<T> {
        self.as_slice().variance()
    }
    fn unbiased_variance(&self) -> Option<T> {
        self.as_slice().unbiased_variance()
    }
    fn standard_deviation(&self) -> Option<T> {
        self.as_slice().standard_deviation()
    }
    fn unbiased_standard_deviation(&self) -> Option<T> {
        self.as_slice().unbiased_standard_deviation()
    }
    fn range(&self) -> Option<T> {
        self.as_slice().range()
    }
    fn quartiles(&self) -> (Option<T>, Option<T>) {
        self.as_slice().quartiles()
    }
    fn iqr(&self) -> Option<T> {
        self.as_slice().iqr()
    }
    fn skewness(&self) -> Option<T> {
        self.as_slice().skewness()
    }
    fn kurtosis(&self) -> Option<T> {
        self.as_slice().kurtosis()
    }
    fn percentiles(&self, p: f64) -> Option<T> {
        self.as_slice().percentiles(p)
    }
    fn covariance(&self, other: &Self) -> Option<T> {
        self.as_slice().covariance(other.as_slice())
    }
    fn correlation_coefficient(&self, other: &Self) -> Option<T> {
        self.as_slice().correlation_coefficient(other.as_slice())
    }
}
