use crate::matrix::Matrix;
use crate::Vector;
use std::panic;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

fn assert_matrix_approx_eq(a: &Matrix<f64>, b: &Matrix<f64>, tol: f64) {
    assert_eq!(a.rows, b.rows, "row mismatch");
    assert_eq!(a.cols, b.cols, "col mismatch");
    for i in 0..a.rows {
        for j in 0..a.cols {
            let va = a[(i, j)];
            let vb = b[(i, j)];
            println!("compare a({i},{j})={va} vs b({i},{j})={vb}, tol={tol}");
            assert!(approx(va, vb, tol), "mismatch at ({i},{j})");
        }
    }
}

#[test]
fn ring_add_sub_mul_and_vector_mul() {
    let a = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    let b = Matrix::new(2, 2, vec![5.0, -1.0, 0.5, 2.0]).unwrap();

    let add = a.checked_add(&b).unwrap();
    let sub = a.checked_sub(&b).unwrap();
    let mul = a.checked_mul(&b).unwrap();

    println!("add={add}\nsub={sub}\nmul={mul}");

    assert_matrix_approx_eq(
        &add,
        &Matrix::new(2, 2, vec![6.0, 1.0, 3.5, 6.0]).unwrap(),
        1e-12,
    );
    assert_matrix_approx_eq(
        &sub,
        &Matrix::new(2, 2, vec![-4.0, 3.0, 2.5, 2.0]).unwrap(),
        1e-12,
    );
    assert_matrix_approx_eq(
        &mul,
        &Matrix::new(
            2,
            2,
            vec![
                1.0 * 5.0 + 2.0 * 0.5,
                -1.0 + 2.0 * 2.0,
                3.0 * 5.0 + 4.0 * 0.5,
                -3.0 + 4.0 * 2.0,
            ],
        )
        .unwrap(),
        1e-12,
    );

    // 行列×ベクトル
    let v = Vector::new(vec![1.0, -1.0]);
    let av = a.checked_mul_vector(&v).unwrap();
    println!("a*v = {:?}", av.data);
    assert!(approx(av[0], -1.0, 1e-12));
    assert!(approx(av[1], -1.0, 1e-12));
}

#[test]
fn ring_identity_diag_and_scaling_ops() {
    let i3 = Matrix::identity(3);
    println!("I3=\n{i3}");
    for k in 0..3 {
        assert!(approx(i3[(k, k)], 1.0, 1e-12));
    }

    // diag
    let dvec = Vector::new(vec![2.0, -1.0, 3.0]);
    let d = Matrix::form_diag(3, 4, &dvec);
    println!("diag=\n{d}");
    for r in 0..3 {
        for c in 0..4 {
            let expected = if r == c { dvec[r] } else { 0.0 };
            assert!(approx(d[(r, c)], expected, 1e-12));
        }
    }

    // スケーリングと加算
    let mut m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, -1.0, 0.0, 4.0]).unwrap();
    m.scale_row(0, 2.0).unwrap();
    println!("after scale_row m=\n{m}");
    assert_matrix_approx_eq(
        &m,
        &Matrix::new(2, 3, vec![2.0, 4.0, 6.0, -1.0, 0.0, 4.0]).unwrap(),
        1e-12,
    );

    m.scale_col(2, -1.0).unwrap();
    println!("after scale_col m=\n{m}");
    assert_matrix_approx_eq(
        &m,
        &Matrix::new(2, 3, vec![2.0, 4.0, -6.0, -1.0, 0.0, -4.0]).unwrap(),
        1e-12,
    );

    m.add_scaled_row_to_row(0, 1, 0.5).unwrap(); // row1 += 0.5*row0
    println!("after add_scaled_row_to_row m=\n{m}");
    assert_matrix_approx_eq(
        &m,
        &Matrix::new(2, 3, vec![2.0, 4.0, -6.0, 0.0, 2.0, -7.0]).unwrap(),
        1e-12,
    );
}

#[test]
fn ring_trace_and_operator_panic_on_dim_mismatch() {
    let m = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]).unwrap();
    let tr = m.trace().unwrap();
    println!("trace={tr}");
    assert!(approx(tr, 1.0 + 5.0 + 9.0, 1e-12));

    // 非正方で trace() は Err
    let rect = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    assert!(rect.trace().is_err(), "trace on non-square should be Err");

    // 次元不一致の演算子は panic
    let a = Matrix::new(2, 2, vec![1.0; 4]).unwrap();
    let b = Matrix::new(3, 3, vec![2.0; 9]).unwrap();
    let res = panic::catch_unwind(|| {
        let _ = &a + &b;
    });
    assert!(res.is_err(), "operator + should panic on dim mismatch");
}

#[test]
fn field_rref_rank_det_inverse() {
    let a = Matrix::new(
        3,
        3,
        vec![
            2.0, 1.0, -1.0, //
            -3.0, -1.0, 2.0, //
            -2.0, 1.0, 2.0,
        ],
    )
    .unwrap();

    // 行基本変形の結果
    let rref = a.rref().unwrap();
    println!("rref=\n{rref}");
    // ランク
    let rank = a.rank().unwrap();
    println!("rank={rank}");
    assert_eq!(rank, 3);

    // 行列式（期待値：-1）
    let det = a.determinant().unwrap();
    println!("det={det}");
    assert!(approx(det, -1.0, 1e-8));

    // 逆行列
    let inv = a.inverse().expect("invertible");
    println!("inv=\n{inv}");
    let id = &a * &inv;
    let eye = Matrix::identity(3);
    assert_matrix_approx_eq(&id, &eye, 1e-8);
}
