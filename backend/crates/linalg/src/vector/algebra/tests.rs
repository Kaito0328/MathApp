use crate::Vector;

fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[test]
fn norm_basic_and_homogeneity_and_triangle() {
    let x = Vector::new(vec![3.0, 4.0]);
    let n = x.norm();
    println!("norm(x)={n}");
    assert!(approx(n, 5.0, 1e-12));

    // 同次性 ||αx|| = |α| ||x||
    let a = -2.5;
    let ax = x.checked_mul_scalar(a);
    let nax = ax.norm();
    println!("a={a}, norm(ax)={nax}");
    assert!(approx(nax, a.abs() * n, 1e-10));

    // 三角不等式 ||x+y|| <= ||x|| + ||y||
    let y = Vector::new(vec![1.0, -2.0]);
    let xy = x.checked_add(&y).unwrap();
    let lhs = xy.norm();
    let rhs = x.norm() + y.norm();
    println!("||x+y||={lhs}, ||x||+||y||={rhs}");
    assert!(lhs <= rhs + 1e-12);
}

#[test]
fn zero_vector_has_zero_norm() {
    let z = Vector::<f64>::zeros(5);
    let nz = z.norm();
    println!("norm(zeros)={nz}");
    assert!(approx(nz, 0.0, 1e-12));
}
