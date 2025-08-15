use linalg::matrix::numerical::svd::SvdDeComposition;
use linalg::Matrix;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

fn assert_orthogonal(a: &Matrix<f64>, tol: f64) {
    let i = Matrix::identity(a.cols);
    let a_t_a = &a.transpose() * a;
    for irow in 0..i.rows {
        for jcol in 0..i.cols {
            assert!(approx(a_t_a[(irow, jcol)], i[(irow, jcol)], tol));
        }
    }
}

#[test]
fn svd_reconstruction_random_shapes() {
    // 数ケース: 正方, 縦長, 横長, ランク落ち
    let cases = vec![
        Matrix::new(3, 3, vec![4.0, 1.0, -2.0, 2.0, 5.0, 1.0, -1.0, 0.0, 6.0]).unwrap(),
        Matrix::new(
            5,
            3,
            vec![
                2.0, -1.0, 0.0, 4.0, 1.0, -2.0, 0.0, 3.5, 1.0, -2.0, 0.0, 1.0, 1.0, 2.0, 0.0,
            ],
        )
        .unwrap(),
        Matrix::new(
            3,
            5,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
            ],
        )
        .unwrap(),
        Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 3.0, 6.0, 9.0]).unwrap(),
    ];
    for a in cases.into_iter() {
        let svd = a.svd().expect("svd failed");
        // 直交性
        assert_orthogonal(&svd.u, 1e-10);
        assert_orthogonal(&svd.v, 1e-10);
        // 復元
        let mut s = Matrix::zeros(svd.u.cols, svd.v.cols);
        for i in 0..svd.sigma.dim() {
            s[(i, i)] = svd.sigma[i];
        }
        let recon = &(&svd.u * &s) * &svd.v.transpose();
        for i in 0..a.rows {
            for j in 0..a.cols {
                assert!(
                    approx(a[(i, j)], recon[(i, j)], 1e-8),
                    "recon mismatch at ({i},{j})",
                );
            }
        }
    }
}
