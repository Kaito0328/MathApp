use crate::Vector;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn new_dim_and_indexing_mutation() {
    let mut v = Vector::new(vec![1.0, 2.0, 3.0]);
    println!("v initial={:?}", v.data);
    assert_eq!(v.dim(), 3);
    assert!(approx(v[1], 2.0, 1e-12));

    // 変更（IndexMut 実装がある場合）
    v[1] = -5.0;
    println!("v after mutation={:?}", v.data);
    assert!(approx(v[1], -5.0, 1e-12));
}

#[test]
fn empty_vector_supported() {
    let v = Vector::<f64>::new(vec![]);
    println!("empty vec dim={}", v.dim());
    assert_eq!(v.dim(), 0);
}
