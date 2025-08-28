use crate::matrix::Matrix;
use crate::Vector;
use std::panic;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn operators_add_sub_mul_and_scalar_ops() {
    let a = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    let b = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]).unwrap();

    // & + &
    let c = &a + &b;
    println!("a+b=\n{c}");
    assert_eq!(c[(0, 0)], 6.0);

    // 所有権の組み合わせもテスト
    let _ = &a + b.clone();
    let _ = a.clone() + &b;
    let _ = a.clone() + b.clone();

    // 乗算
    let m = &a * &b;
    println!("a*b=\n{m}");
    assert!(approx(m[(0, 0)], 1.0 * 5.0 + 2.0 * 7.0, 1e-12));

    // ベクトル
    let v = Vector::new(vec![1.0, -1.0]);
    let av = &a * &v;
    println!("a*v={:?}", av.data);
    assert!(approx(av[0], -1.0, 1e-12));

    // スカラー
    let s1 = &a * 2.0;
    let s2 = &a + 1.0;
    let s3 = &a - 1.0;
    println!("a*2=\n{s1}\na+1=\n{s2}\na-1=\n{s3}");
    assert!(approx(s1[(1, 1)], 8.0, 1e-12));
    assert!(approx(s2[(0, 0)], 2.0, 1e-12));
    assert!(approx(s3[(0, 0)], 0.0, 1e-12));
}

#[test]
fn neg_and_indexing() {
    let a = Matrix::new(2, 2, vec![1.0, -2.0, 3.0, -4.0]).unwrap();
    let n1 = -&a;
    let n2 = -a.clone();
    println!("neg_ref=\n{n1}\nneg_move=\n{n2}");
    assert!(approx(n1[(0, 1)], 2.0, 1e-12));
    assert!(approx(n2[(1, 1)], 4.0, 1e-12));

    // インデックス参照
    assert!(approx(a[(0, 0)], 1.0, 1e-12));
    assert!(approx(a[(1, 0)], 3.0, 1e-12));
}

#[test]
fn display_format_rounding_and_shape_line() {
    let m = Matrix::new(2, 2, vec![1.23456, -0.00001, 0.33333, 10.0]).unwrap();
    let s = format!("{m}");
    println!("display:\n{s}");
    // 先頭行
    assert!(s.contains("rows: 2, cols: 2"));
    // 丸め（4桁）
    assert!(s.contains("1.2346"));
    assert!(s.contains("0.3333"));
    // -0 は 0 として表示される
    assert!(!s.contains("-0"));
}

#[test]
fn operators_panic_on_dimension_mismatch() {
    let a = Matrix::new(2, 2, vec![1.0; 4]).unwrap();
    let b = Matrix::new(3, 3, vec![2.0; 9]).unwrap();

    let res = panic::catch_unwind(|| {
        let _ = &a + &b;
    });
    println!("panic_add_mismatch={:?}", res.is_err());
    assert!(res.is_err());

    let res = panic::catch_unwind(|| {
        let _ = &a * &b;
    });
    println!("panic_mul_mismatch={:?}", res.is_err());
    assert!(res.is_err());
}
