use num_traits::ToPrimitive;

// 整数向けのトレイト（連続側と同等のAPI）
pub trait Stats<T> {
    fn mean(&self) -> Option<f64>;
    fn median(&self) -> Option<f64>;
    fn mode(&self) -> Option<Vec<T>>;
    fn variance(&self) -> Option<f64>;
    fn unbiased_variance(&self) -> Option<f64>;
    fn standard_deviation(&self) -> Option<f64>;
    fn unbiased_standard_deviation(&self) -> Option<f64>;
    fn range(&self) -> Option<f64>;
    fn quartiles(&self) -> (Option<f64>, Option<f64>);
    fn iqr(&self) -> Option<f64>;
    fn percentiles(&self, p: f64) -> Option<f64>;
    fn covariance(&self, other: &Self) -> Option<f64>;
    fn correlation_coefficient(&self, other: &Self) -> Option<f64>;
}

impl<T> Stats<T> for [T]
where
    T: Ord + Clone + Copy + std::hash::Hash + Eq + ToPrimitive,
{
    fn mean(&self) -> Option<f64> {
        if self.is_empty() {
            None
        } else {
            // 合計をf64で計算して桁あふれを防ぐ
            let sum: f64 = self.iter().map(|&x| x.to_f64().unwrap()).sum();
            Some(sum / self.len() as f64)
        }
    }

    fn median(&self) -> Option<f64> {
        if self.is_empty() {
            return None;
        }
        let mut sorted = self.to_vec();
        sorted.sort();
        let len = sorted.len();
        if len % 2 == 0 {
            let mid_right = len / 2;
            let mid_left = mid_right - 1;
            // 整数同士の平均は小数になる可能性があるのでf64で計算
            Some((sorted[mid_left].to_f64().unwrap() + sorted[mid_right].to_f64().unwrap()) / 2.0)
        } else {
            Some(sorted[len / 2].to_f64().unwrap())
        }
    }

    fn mode(&self) -> Option<Vec<T>> {
        if self.is_empty() {
            return None;
        }
        let mut counts = std::collections::HashMap::new();
        for &value in self.iter() {
            *counts.entry(value).or_insert(0) += 1;
        }
        let max_count = match counts.values().max() {
            Some(&max) => max,
            None => return Some(vec![]),
        };
        if max_count <= 1 {
            return Some(vec![]);
        }

        Some(
            counts
                .into_iter()
                .filter(|&(_, count)| count == max_count)
                .map(|(value, _)| value)
                .collect(),
        )
    }

    fn variance(&self) -> Option<f64> {
        if self.len() < 2 {
            return None;
        }
        let mean = self.mean()?;
        let sum_sq: f64 = self
            .iter()
            .map(|&x| {
                let v = x.to_f64().unwrap();
                (v - mean) * (v - mean)
            })
            .sum();
        Some(sum_sq / self.len() as f64)
    }

    fn unbiased_variance(&self) -> Option<f64> {
        if self.len() < 2 {
            return None;
        }
        let mean = self.mean()?;
        let sum_sq: f64 = self
            .iter()
            .map(|&x| {
                let v = x.to_f64().unwrap();
                (v - mean) * (v - mean)
            })
            .sum();
        Some(sum_sq / (self.len() as f64 - 1.0))
    }

    fn standard_deviation(&self) -> Option<f64> {
        self.variance().map(|v| v.sqrt())
    }
    fn unbiased_standard_deviation(&self) -> Option<f64> {
        self.unbiased_variance().map(|v| v.sqrt())
    }

    fn range(&self) -> Option<f64> {
        let mut it = self.iter();
        let first = it.next()?;
        let mut mn = *first;
        let mut mx = *first;
        for &x in it {
            if x < mn {
                mn = x;
            }
            if x > mx {
                mx = x;
            }
        }
        Some(mx.to_f64().unwrap() - mn.to_f64().unwrap())
    }

    fn quartiles(&self) -> (Option<f64>, Option<f64>) {
        (self.percentiles(25.0), self.percentiles(75.0))
    }
    fn iqr(&self) -> Option<f64> {
        let (q1, q3) = self.quartiles();
        Some(q3? - q1?)
    }

    fn percentiles(&self, p: f64) -> Option<f64> {
        if !(0.0..=100.0).contains(&p) {
            return None;
        }
        if self.is_empty() {
            return None;
        }
        let mut xs = self.to_vec();
        xs.sort();
        let rank = (p / 100.0) * (xs.len().saturating_sub(1)) as f64;
        let lo = rank.floor() as usize;
        let hi = rank.ceil() as usize;
        if lo == hi {
            Some(xs[lo].to_f64().unwrap())
        } else {
            let w = rank - lo as f64;
            Some(xs[lo].to_f64().unwrap() * (1.0 - w) + xs[hi].to_f64().unwrap() * w)
        }
    }

    fn covariance(&self, other: &Self) -> Option<f64> {
        if self.len() != other.len() || self.is_empty() {
            return None;
        }
        let mx = self.mean()?;
        let my = other.mean()?;
        let sum: f64 = self
            .iter()
            .zip(other.iter())
            .map(|(&x, &y)| (x.to_f64().unwrap() - mx) * (y.to_f64().unwrap() - my))
            .sum();
        Some(sum / self.len() as f64)
    }

    fn correlation_coefficient(&self, other: &Self) -> Option<f64> {
        let cov = self.covariance(other)?;
        let sx = self.standard_deviation()?;
        let sy = other.standard_deviation()?;
        if sx == 0.0 || sy == 0.0 {
            return None;
        }
        Some(cov / (sx * sy))
    }
}

// Vec<T> はスライス実装に委譲
impl<T> Stats<T> for Vec<T>
where
    T: Ord + Clone + Copy + std::hash::Hash + Eq + ToPrimitive,
{
    fn mean(&self) -> Option<f64> {
        self.as_slice().mean()
    }
    fn median(&self) -> Option<f64> {
        self.as_slice().median()
    }
    fn mode(&self) -> Option<Vec<T>> {
        self.as_slice().mode()
    }
    fn variance(&self) -> Option<f64> {
        self.as_slice().variance()
    }
    fn unbiased_variance(&self) -> Option<f64> {
        self.as_slice().unbiased_variance()
    }
    fn standard_deviation(&self) -> Option<f64> {
        self.as_slice().standard_deviation()
    }
    fn unbiased_standard_deviation(&self) -> Option<f64> {
        self.as_slice().unbiased_standard_deviation()
    }
    fn range(&self) -> Option<f64> {
        self.as_slice().range()
    }
    fn quartiles(&self) -> (Option<f64>, Option<f64>) {
        self.as_slice().quartiles()
    }
    fn iqr(&self) -> Option<f64> {
        self.as_slice().iqr()
    }
    fn percentiles(&self, p: f64) -> Option<f64> {
        self.as_slice().percentiles(p)
    }
    fn covariance(&self, other: &Self) -> Option<f64> {
        self.as_slice().covariance(other.as_slice())
    }
    fn correlation_coefficient(&self, other: &Self) -> Option<f64> {
        self.as_slice().correlation_coefficient(other.as_slice())
    }
}
