use linalg::{Matrix, Vector};
use statsmodels::{error::StatsModelsError, estimation::kalman::KalmanFilter};

#[test]
fn kalman_new_dimension_checks() {
    let n = 2;
    let m = 1;
    let x0 = Vector::new(vec![0.0, 0.0]);
    let p0 = Matrix::identity(n);
    let f = Matrix::identity(n);
    let h = Matrix::new(m, n, vec![1.0, 0.0]).unwrap();
    let q = Matrix::identity(n);
    // 間違ったR (2x2) を与えてエラーを期待
    let r_bad = Matrix::identity(n);
    let err = KalmanFilter::new(
        x0.clone(),
        p0.clone(),
        f.clone(),
        h.clone(),
        q.clone(),
        r_bad,
    )
    .err()
    .unwrap();
    match err {
        StatsModelsError::DimensionMismatch { .. } => {}
        e => panic!("unexpected error: {e:?}"),
    }

    // 正しいR (1x1) ならOK
    let r = Matrix::identity(m);
    assert!(KalmanFilter::new(x0, p0, f, h, q, r).is_ok());
}

#[test]
fn kalman_filters_toward_observation() {
    // 1次元の簡単例: 真値は0として、観測0.2に近づくことを確認
    let n = 1;
    let m = 1;
    let f = Matrix::identity(n);
    let h = Matrix::identity(m);
    let q = Matrix::new(n, n, vec![1e-3]).unwrap();
    let r = Matrix::new(m, m, vec![1e-2]).unwrap();
    let x0 = Vector::new(vec![0.0]);
    let p0 = Matrix::new(n, n, vec![1.0]).unwrap();
    let mut kf = KalmanFilter::new(x0, p0, f, h, q, r).unwrap();

    let z = Vector::new(vec![0.2]);
    kf.predict().unwrap();
    kf.update(&z).unwrap();
    let est = kf.state()[0];
    assert!(
        est > 0.0 && est < 0.2,
        "estimate should move toward observation: {est}"
    );
}
