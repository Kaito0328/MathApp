use linalg::{Matrix, Vector};
use statsmodels::linear_model::ols::solve_linear_system;

fn main() {
    // 過剰決定系 Ax=b（最小二乗解）
    let a = Matrix::new(4, 2, vec![1.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 3.0]).unwrap();
    let b = Vector::new(vec![0.0, 1.0, 2.0, 3.0]);

    let x = solve_linear_system(&a, &b).unwrap();
    println!("pinv solution x: {x}");
    let fit = &a * &x;
    println!("fitted Ax: {fit}");
}
