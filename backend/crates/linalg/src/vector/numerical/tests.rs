use crate::Vector;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn indexing_and_mutation_behaves_consistently() {
    let mut v = Vector::new(vec![10.0, -2.0, 3.5]);
    println!("v start={:?}", v.data);

    // 読み取り
    assert!(approx(v[0], 10.0, 1e-12));
    assert!(approx(v[2], 3.5, 1e-12));

    // 書き込み（IndexMut が実装されている前提）
    v[0] = -1.0;
    v[2] = 7.0;
    println!("v after writes={:?}", v.data);
    assert!(approx(v[0], -1.0, 1e-12));
    assert!(approx(v[2], 7.0, 1e-12));
}

#[test]
fn checked_ops_dimension_mismatch_yields_error() {
    let a = Vector::new(vec![1.0, 2.0]);
    let b = Vector::new(vec![1.0, 2.0, 3.0]);

    let add = a.checked_add(&b);
    let sub = a.checked_sub(&b);
    let had = a.hadamard_product(&b);
    println!(
        "errs add.is_err={:?}, sub.is_err={:?}, had.is_err={:?}",
        add.is_err(),
        sub.is_err(),
        had.is_err()
    );

    assert!(add.is_err());
    assert!(sub.is_err());
    assert!(had.is_err());
}
