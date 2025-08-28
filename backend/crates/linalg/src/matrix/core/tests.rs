use crate::{matrix::Matrix, Vector};

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

fn assert_matrix_eq(a: &Matrix<f64>, b: &Matrix<f64>) {
    assert_eq!(a.rows, b.rows);
    assert_eq!(a.cols, b.cols);
    for i in 0..a.rows {
        for j in 0..a.cols {
            assert!(
                approx(a[(i, j)], b[(i, j)], 1e-12),
                "mismatch at ({}, {}): {} vs {}",
                i,
                j,
                a[(i, j)],
                b[(i, j)]
            );
        }
    }
}

#[test]
fn new_row_col_setters_getters() {
    let mut m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();

    // row/col 取得
    let r0 = m.row(0).unwrap();
    let c1 = m.col(1).unwrap();
    println!("r0={:?}, c1={:?}", r0.data, c1.data);
    assert_eq!(r0.data, vec![1.0, 2.0, 3.0]);
    assert_eq!(c1.data, vec![2.0, 5.0]);

    // set_row/set_col
    let new_r1 = Vector::new(vec![7.0, 8.0, 9.0]);
    m.set_row(1, &new_r1).unwrap();
    let new_c0 = Vector::new(vec![10.0, 11.0]);
    m.set_col(0, &new_c0).unwrap();

    let exp = Matrix::new(2, 3, vec![10.0, 2.0, 3.0, 11.0, 8.0, 9.0]).unwrap();
    println!("after set m=\n{m}");
    assert_matrix_eq(&m, &exp);
}

#[test]
fn partial_row_col_and_errors() {
    let m = Matrix::new(
        4,
        4,
        vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ],
    )
    .unwrap();

    let pr = m.partial_row(2, 1, 3).unwrap(); // row 2, cols [1..3)
    let pc = m.partial_col(1, 0, 3).unwrap(); // col 1, rows [0..3)
    println!("partial_row={:?}, partial_col={:?}", pr.data, pc.data);
    assert_eq!(pr.data, vec![10.0, 11.0]);
    assert_eq!(pc.data, vec![2.0, 6.0, 10.0]);

    // 範囲エラー
    assert!(m.partial_row(5, 0, 1).is_err());
    assert!(m.partial_col(0, 3, 2).is_err());
}

#[test]
fn transpose_swap_submatrix_and_stack() {
    let mut m = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();

    // 転置
    let mt = m.transpose();
    let exp_t = Matrix::new(3, 2, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]).unwrap();
    println!("mt=\n{mt}");
    assert_matrix_eq(&mt, &exp_t);

    // swap_rows
    m.swap_rows(0, 1).unwrap();
    let exp_sw = Matrix::new(2, 3, vec![4.0, 5.0, 6.0, 1.0, 2.0, 3.0]).unwrap();
    println!("after swap m=\n{m}");
    assert_matrix_eq(&m, &exp_sw);

    // submatrix & set_submatrix
    let sub = m.submatrix(0, 2, 1, 3);
    let exp_sub = Matrix::new(2, 2, vec![5.0, 6.0, 2.0, 3.0]).unwrap();
    assert_matrix_eq(&sub, &exp_sub);

    let mut big = Matrix::zeros(3, 4);
    big.set_submatrix(1, 1, &sub).unwrap();
    println!("big after set_submatrix=\n{big}");
    let exp_big = Matrix::new(
        3,
        4,
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 6.0, 0.0, 0.0, 2.0, 3.0, 0.0],
    )
    .unwrap();
    assert_matrix_eq(&big, &exp_big);

    // hstack/vstack
    let a = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    let b = Matrix::new(2, 1, vec![9.0, 8.0]).unwrap();
    let h = a.hstack(&b).unwrap();
    println!("hstack=\n{h}");
    let exp_h = Matrix::new(2, 3, vec![1.0, 2.0, 9.0, 3.0, 4.0, 8.0]).unwrap();
    assert_matrix_eq(&h, &exp_h);

    let v = a
        .vstack(&Matrix::new(1, 2, vec![7.0, 6.0]).unwrap())
        .unwrap();
    println!("vstack=\n{v}");
    let exp_v = Matrix::new(3, 2, vec![1.0, 2.0, 3.0, 4.0, 7.0, 6.0]).unwrap();
    assert_matrix_eq(&v, &exp_v);
}

#[test]
fn is_square_and_new_validation() {
    let a = Matrix::new(2, 2, vec![1.0, 0.0, 0.0, 1.0]).unwrap();
    println!("is_square={}", a.is_square());
    assert!(a.is_square());

    // 要素数不一致は Err
    let bad = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0]);
    assert!(bad.is_err());
}
