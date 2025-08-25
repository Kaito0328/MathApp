use statistics::continuous_stats::Stats as CStats;
use statistics::discrete_stats::Stats as DStats;

#[test]
fn continuous_basic() {
    let xs = vec![1.0_f64, 2.0, 3.0, f64::NAN, f64::INFINITY];
    assert_eq!(xs.mean(), Some(2.0));
    assert_eq!(xs.median(), Some(2.0));
    assert_eq!(xs.range(), Some(2.0));
    assert_eq!(xs.percentiles(0.0), Some(1.0));
    assert_eq!(xs.percentiles(100.0), Some(3.0));
    let var = xs.variance().unwrap();
    assert!((var - 2.0/3.0).abs() < 1e-12);
    let s = xs.standard_deviation().unwrap();
    assert!((s - var.sqrt()).abs() < 1e-12);
}

#[test]
fn continuous_corr_cov() {
    let x = vec![1.0_f64, 2.0, 3.0, 4.0];
    let y = vec![2.0_f64, 4.0, 6.0, 8.0];
    let cov = x.covariance(&y).unwrap();
    assert!((cov - 2.5).abs() < 1e-12);
    let r = x.correlation_coefficient(&y).unwrap();
    assert!((r - 1.0).abs() < 1e-12);
}

#[test]
fn discrete_basic() {
    let xs = vec![1i32, 2, 2, 3];
    assert_eq!(xs.mean(), Some(2.0));
    assert_eq!(xs.median(), Some(2.0));
    let mut mode = xs.mode().unwrap();
    mode.sort();
    assert_eq!(mode, vec![2]);
}