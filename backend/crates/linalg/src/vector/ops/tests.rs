use super::*;
use crate::matrix::Matrix;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn zeros_ones_length_and_values() {
    let z = Vector::<f64>::zeros(4);
    let o = Vector::<f64>::ones(3);
    println!("zeros={:?}, ones={:?}", z.data, o.data);
    assert_eq!(z.dim(), 4);
    assert!(z.data.iter().all(|&x| approx(x, 0.0, 1e-12)));
    assert_eq!(o.dim(), 3);
    assert!(o.data.iter().all(|&x| approx(x, 1.0, 1e-12)));
}

#[test]
fn add_sub_ok_and_err() {
    let a = Vector::new(vec![1.0, 2.0, 3.0]);
    let b = Vector::new(vec![4.0, -1.0, 0.5]);

    let add = a.checked_add(&b).unwrap();
    let sub = a.checked_sub(&b).unwrap();
    println!("add={:?}, sub={:?}", add.data, sub.data);

    assert!(approx(add[0], 5.0, 1e-12));
    assert!(approx(sub[2], 2.5, 1e-12));

    let bad = Vector::new(vec![1.0]);
    assert!(a.checked_add(&bad).is_err());
    assert!(a.checked_sub(&bad).is_err());
}

#[test]
fn scalar_ops_add_sub_mul_and_neg() {
    let a = Vector::new(vec![1.0, -2.0, 3.0]);

    let add = a.checked_add_scalar(2.0);
    let sub = a.checked_sub_scalar(1.5);
    let mul = a.checked_mul_scalar(-3.0);
    let neg = a.checked_neg();
    println!(
        "add={:?}, sub={:?}, mul={:?}, neg={:?}",
        add.data, sub.data, mul.data, neg.data
    );

    assert!(approx(add[0], 3.0, 1e-12));
    assert!(approx(sub[1], -3.5, 1e-12));
    assert!(approx(mul[2], -9.0, 1e-12));
    assert!(approx(neg[1], 2.0, 1e-12));
}

#[test]
fn dot_and_hadamard_and_cross() {
    let a = Vector::new(vec![1.0, 2.0, 3.0]);
    let b = Vector::new(vec![4.0, -1.0, 0.5]);

    // dot
    let d = a.dot(&b);
    println!("dot={d}");
    assert!(approx(d, 1.0 * 4.0 - 2.0 + 3.0 * 0.5, 1e-12));

    // hadamard
    let h = a.hadamard_product(&b).unwrap();
    println!("hadamard={:?}", h.data);
    assert_eq!(h.data, vec![4.0, -2.0, 1.5]);

    let bad = Vector::new(vec![1.0]);
    assert!(a.hadamard_product(&bad).is_err());

    // cross (3D only)
    let c = a.cross(&Vector::new(vec![0.0, 1.0, 0.0])).unwrap();
    println!("cross={:?}", c.data);
    assert_eq!(c.data, vec![-3.0, 0.0, 1.0]);

    let not3 = Vector::new(vec![1.0, 2.0]);
    assert!(not3.cross(&not3).is_err());
}

#[test]
fn mul_matrix_outer_product_like() {
    // self (dim=2) * (1 x 3) row-matrix -> (2 x 3) outer-like product
    let v = Vector::new(vec![2.0, -1.0]);
    let row = Matrix::new(1, 3, vec![3.0, -2.0, 4.0]).unwrap();
    let m = v.checked_mul_matrix(&row).unwrap();
    println!("result outer-like matrix:\n{m}");
    assert_eq!(m.rows, 2);
    assert_eq!(m.cols, 3);
    assert!(approx(m[(0, 0)], 6.0, 1e-12));
    assert!(approx(m[(1, 2)], -4.0, 1e-12));
}
