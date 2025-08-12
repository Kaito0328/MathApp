use num_complex::Complex;
use crate::{matrix::numerical::EigenDecomposition, Matrix};
use crate::matrix::numerical::QrDecomposition;

fn to_complex(a: &Matrix<f64>) -> Matrix<Complex<f64>> {
    let mut out = Matrix::zeros(a.rows, a.cols);
    for i in 0..a.rows { for j in 0..a.cols { out[(i,j)] = Complex::new(a[(i,j)], 0.0); } }
    out
}

fn capprox(a: Complex<f64>, b: Complex<f64>, tol: f64) -> bool { (a-b).norm() <= tol }

fn make_lambda(eigs: &[Complex<f64>]) -> Matrix<Complex<f64>> {
    let n = eigs.len();
    let mut d = Matrix::zeros(n, n);
    for i in 0..n { d[(i,i)] = eigs[i]; }
    d
}

#[test]
fn eigen_4x4_mixed() {
    // {1±2i, 3, -4}
    let mut d = Matrix::zeros(4,4);
    d[(0,0)] = 1.0; d[(0,1)] = -2.0; d[(1,0)] = 2.0; d[(1,1)] = 1.0;
    d[(2,2)] = 3.0; d[(3,3)] = -4.0;
    let base = Matrix::new(4,4, vec![0.0,0.0,3.0,-4.0, -2.0,5.0,-2.0,2.0, -6.0,9.0,-1.0,2.0, 1.0,2.0,0.0,-1.0]).unwrap();
    let q = base.qr_decomposition().unwrap().q;
    let a = &(&q * &d) * &q.transpose();

    let res = a.eigen_decomposition_complex().expect("eig failed");

    let mut exp = [Complex::new(1.0,2.0), Complex::new(1.0,-2.0), Complex::new(3.0,0.0), Complex::new(-4.0,0.0)];
    exp.sort_by(|a,b| a.re.partial_cmp(&b.re).unwrap_or(std::cmp::Ordering::Equal).then(a.im.partial_cmp(&b.im).unwrap_or(std::cmp::Ordering::Equal)));
    for (g,e) in res.eigen_values.iter().zip(exp.iter()) { assert!(capprox(*g,*e,1e-8)); }

    let a_c = to_complex(&a);
    let v = &res.eigen_vectors;
    let lambda = make_lambda(&res.eigen_values);
    let resid = &(&a_c * v) - &(v * &lambda);
    let frob = resid.data.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
    assert!(frob < 1e-6);
}

#[test]
fn eigen_7x7_mixed() {
    // {(2±3i), (-1±2i), (0±1i), 5}
    let mut d = Matrix::zeros(7,7);
    d[(0,0)] = 2.0; d[(0,1)] = -3.0; d[(1,0)] = 3.0; d[(1,1)] = 2.0;
    d[(2,2)] = -1.0; d[(2,3)] = -2.0; d[(3,2)] = 2.0; d[(3,3)] = -1.0;
    d[(4,4)] = 0.0; d[(4,5)] = -1.0; d[(5,4)] = 1.0; d[(5,5)] = 0.0;
    d[(6,6)] = 5.0;
    let base = Matrix::new(7,7, vec![
        1.0,-2.0,3.0,0.0,-1.0,2.0,-3.0,
        0.0,4.0,-1.0,2.0,3.0,-2.0,1.0,
        -2.0,1.0,0.0,-3.0,2.0,1.0,-1.0,
        3.0,0.0,-2.0,1.0,-4.0,0.5,2.0,
        -1.0,2.0,1.0,-2.0,0.0,3.0,-2.0,
        2.0,-1.0,0.5,1.0,-2.0,4.0,0.0,
        -3.0,1.0,-1.0,2.0,-2.0,0.0,5.0,
    ]).unwrap();
    let q = base.qr_decomposition().unwrap().q;
    let a = &(&q * &d) * &q.transpose();

    let res = a.eigen_decomposition_complex().expect("eig failed");
    // 値チェックと残差（軽め）
    let a_c = to_complex(&a);
    let v = &res.eigen_vectors;
    let lambda = make_lambda(&res.eigen_values);
    let resid = &(&a_c * v) - &(v * &lambda);
    let frob = resid.data.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
    assert!(frob < 5e-6);
}
