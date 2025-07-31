use linalg::{Matrix, Vector};

#[test]
fn test_matrix_vector_integration() {
    // 実際の線形代数の問題を解くテスト
    let a = Matrix::new(2, 2, vec![2.0, 1.0, 1.0, 3.0]);
    let b = Vector::new(vec![5.0, 7.0]);

    // A * x = b を解く（将来的に）
    let result = a * b;
    assert_eq!(result.dim(), 2);
}

#[test]
fn test_complex_operations() {
    // 複数の操作を組み合わせたテスト
    let m1 = Matrix::<f64>::identity(3);
    let m2 = Matrix::<f64>::zeros(3, 3);
    let sum = m1 + m2;

    // 単位行列 + ゼロ行列 = 単位行列
    assert!(sum.is_square());
    assert_eq!(sum.trace(), 3.0);
}

#[test]
fn test_chain_operations() {
    // 連続した操作のテスト
    let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::new(vec![2.0, 3.0, 4.0]);

    let sum = v1 + v2;
    let scaled = sum * 0.5;
    let norm = scaled.norm();

    assert!(norm > 0.0);
}

#[test]
fn test_matrix_operations_chain() {
    // 行列操作の連鎖テスト
    let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let transposed = m.transpose();
    let identity = Matrix::<f64>::identity(2);
    let result = transposed + identity;

    assert_eq!(result.rows, 2);
    assert_eq!(result.cols, 2);
}
