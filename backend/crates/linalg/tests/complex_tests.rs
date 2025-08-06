use linalg::{Matrix, Vector};

#[test]
fn test_solve_linear_equation() {
    // テスト名も具体的にする
    // A * x = b
    let a = Matrix::new(2, 2, vec![2.0, 1.0, 1.0, 3.0]).unwrap();
    let b = Vector::new(vec![5.0, 7.0]);

    // デバッグ用に残しておいても良い表示
    println!("Matrix A: {a:?}");
    println!("Vector b: {b:?}");

    // A * x = b を解く
    // a.inverse() が Option を返すので unwrap() が必要
    // Matrix * Vector の演算子を実装している前提
    let x = a.inverse().unwrap() * &b;

    println!("Solution x = {x:?}"); // 失敗時に表示されるので便利

    // 期待される結果を定義
    let expected_x: Vector<f64> = Vector::new(vec![1.6, 1.8]);

    // assert! で結果を検証（浮動小数点数の比較は注意が必要）
    // ここでは簡単のため、次元と各要素を個別に比較
    assert_eq!(x.dim(), expected_x.dim());
    // ※浮動小数点数の完全一致は危険なため、通常は許容誤差を設けて比較します
    assert!((x[0] - expected_x[0]).abs() < 1e-9);
    assert!((x[1] - expected_x[1]).abs() < 1e-9);
}

#[test]
fn test_complex_operations() {
    // 複数の操作を組み合わせたテスト
    println!("=== Complex Operations Test ===");

    let m1 = Matrix::<f64>::identity(3);
    let m2 = Matrix::<f64>::zeros(3, 3);
    println!("Identity matrix m1: {m1:?}");
    println!("Zero matrix m2: {m2:?}");

    let sum = m1 + m2;
    println!("Sum (m1 + m2): {sum:?}");

    // 単位行列 + ゼロ行列 = 単位行列
    assert!(sum.is_square());
    println!("Matrix is square: {}", sum.is_square());

    let trace = sum.trace();
    println!("Trace of sum: {trace}");
    assert_eq!(trace, 3.0);
}

#[test]
fn test_chain_operations() {
    // 連続した操作のテスト
    println!("=== Chain Operations Test ===");

    let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::new(vec![2.0, 3.0, 4.0]);
    println!("Vector v1: {v1:?}");
    println!("Vector v2: {v2:?}");

    let sum = v1 + v2;
    println!("Sum (v1 + v2): {sum:?}");

    let scaled = sum * 0.5;
    println!("Scaled by 0.5: {scaled:?}");

    let norm = scaled.norm();
    println!("Norm of scaled vector: {norm}");

    assert!(norm > 0.0);
}

#[test]
fn test_matrix_operations_chain() {
    // 行列操作の連鎖テスト
    println!("=== Matrix Operations Chain Test ===");

    let m = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    println!("Original matrix m: {m:?}");

    let transposed = m.transpose();
    println!("Transposed matrix: {transposed:?}");

    let identity = Matrix::<f64>::identity(2);
    println!("Identity matrix: {identity:?}");

    let result = transposed + identity;
    println!("Result (transposed + identity): {result:?}");

    assert_eq!(result.rows, 2);
    assert_eq!(result.cols, 2);
}

#[test]
fn test_matrix_vector_multiplication() {
    println!("=== Matrix-Vector Multiplication Test ===");

    let matrix = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]).unwrap();
    let vector = Vector::new(vec![1.0, 1.0, 1.0]);

    println!("Matrix A: {matrix:?}");
    println!("Vector x: {vector:?}");

    let result = matrix * vector;
    println!("Result (A * x): {result:?}");

    // 期待値: [6, 15, 24]
    let expected: Vector<f64> = Vector::new(vec![6.0, 15.0, 24.0]);
    println!("Expected result: {expected:?}");

    assert_eq!(result.dim(), expected.dim());
    for i in 0..result.dim() {
        assert!((result[i] - expected[i]).abs() < 1e-9);
    }
}

#[test]
fn test_matrix_determinant_and_inverse() {
    println!("=== Determinant and Inverse Test ===");

    let matrix: Matrix<f64> = Matrix::new(2, 2, vec![4.0, 7.0, 2.0, 6.0]).unwrap();
    println!("Matrix A: {matrix:?}");

    let det = matrix.determinant().unwrap();
    println!("Determinant: {det}");
    // 4*6 - 7*2 = 24 - 14 = 10
    assert!((det - 10.0).abs() < 1e-9);

    let inverse = matrix.inverse().unwrap();
    println!("Inverse matrix: {inverse:?}");

    // A * A^(-1) = I をテスト
    let product = matrix * inverse;
    println!("A * A^(-1): {product:?}");

    let identity = Matrix::<f64>::identity(2);
    println!("Expected identity: {identity:?}");

    // 単位行列との比較
    for i in 0..2 {
        for j in 0..2 {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert!((product[(i, j)] - expected).abs() < 1e-9);
        }
    }
}

#[test]
fn test_vector_operations() {
    println!("=== Vector Operations Test ===");

    let v1 = Vector::new(vec![3.0, 4.0]);
    let v2 = Vector::new(vec![1.0, 2.0]);

    println!("Vector v1: {v1:?}");
    println!("Vector v2: {v2:?}");

    // ノルムのテスト
    let norm1 = v1.norm();
    println!("Norm of v1: {norm1}");
    assert!((norm1 - 5.0).abs() < 1e-9); // √(3² + 4²) = 5

    // 内積のテスト
    let dot_product = v1.dot(&v2);
    println!("Dot product (v1 · v2): {dot_product}");
    assert!((dot_product - 11.0).abs() < 1e-9); // 3*1 + 4*2 = 11

    // ベクトルの減算
    let diff = v1 - v2;
    println!("Difference (v1 - v2): {diff:?}");

    let expected_diff = Vector::new(vec![2.0, 2.0]);
    println!("Expected difference: {expected_diff:?}");

    assert_eq!(diff.dim(), expected_diff.dim());
    for i in 0..diff.dim() {
        assert!((diff[i] - expected_diff[i]).abs() < 1e-9);
    }
}

#[test]
fn test_matrix_scalar_operations() {
    println!("=== Matrix Scalar Operations Test ===");

    let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    println!("Original matrix: {matrix:?}");

    // スカラー倍
    let scaled = matrix * 2.0;
    println!("Scaled by 2.0: {scaled:?}");

    let expected: Matrix<f64> = Matrix::new(2, 3, vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0]).unwrap();
    println!("Expected scaled matrix: {expected:?}");

    assert_eq!(scaled.rows, expected.rows);
    assert_eq!(scaled.cols, expected.cols);

    for i in 0..scaled.rows {
        for j in 0..scaled.cols {
            assert!((scaled[(i, j)] - expected[(i, j)]).abs() < 1e-9);
        }
    }
}

#[test]
fn test_3d_vector_cross_product() {
    println!("=== 3D Vector Cross Product Test ===");

    let v1 = Vector::new(vec![1.0, 0.0, 0.0]);
    let v2 = Vector::new(vec![0.0, 1.0, 0.0]);

    println!("Vector v1 (i): {v1:?}");
    println!("Vector v2 (j): {v2:?}");

    let cross = v1.cross(&v2).unwrap();
    println!("Cross product (v1 × v2): {cross:?}");

    let expected: Vector<f64> = Vector::new(vec![0.0, 0.0, 1.0]);
    println!("Expected cross product (k): {expected:?}");

    assert_eq!(cross.dim(), 3);
    for i in 0..3 {
        assert!((cross[i] - expected[i]).abs() < 1e-9);
    }
}

#[test]
fn test_matrix_hstack_vstack() {
    println!("=== Matrix Stack Operations Test ===");

    let m1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    let m2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]).unwrap();

    println!("Matrix m1: {m1:?}");
    println!("Matrix m2: {m2:?}");

    // 水平結合
    let hstacked = m1.hstack(&m2).unwrap();
    println!("Horizontally stacked: {hstacked:?}");

    assert_eq!(hstacked.rows, 2);
    assert_eq!(hstacked.cols, 4);

    // 垂直結合
    let vstacked = m1.vstack(&m2).unwrap();
    println!("Vertically stacked: {vstacked:?}");

    assert_eq!(vstacked.rows, 4);
    assert_eq!(vstacked.cols, 2);
}

#[test]
fn test_matrix_submatrix() {
    println!("=== Submatrix Test ===");

    let matrix: Matrix<f64> = Matrix::new(
        4,
        4,
        vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ],
    )
    .unwrap();

    println!("Original 4x4 matrix: {matrix:?}");

    let sub = matrix.submatrix(1, 3, 1, 3);
    println!("Submatrix [1:3, 1:3]: {sub:?}");

    let expected = Matrix::new(2, 2, vec![6.0, 7.0, 10.0, 11.0]).unwrap();
    println!("Expected submatrix: {expected:?}");

    assert_eq!(sub.rows, 2);
    assert_eq!(sub.cols, 2);

    for i in 0..sub.rows {
        for j in 0..sub.cols {
            assert!((sub[(i, j)] - expected[(i, j)]).abs() < 1e-9);
        }
    }
}

#[test]
fn test_vector_normalization() {
    println!("=== Vector Normalization Test ===");

    let vector = Vector::new(vec![3.0, 4.0, 5.0]);
    println!("Original vector: {vector:?}");

    let original_norm = vector.norm();
    println!("Original norm: {original_norm}");

    let normalized = vector.normalize();
    println!("Normalized vector: {normalized:?}");

    let new_norm = normalized.norm();
    println!("New norm: {new_norm}");

    // 正規化後のノルムは1になるはず
    assert!((new_norm - 1.0).abs() < 1e-9);

    // 元のベクトルとの方向が同じかテスト（内積が元のノルムと等しい）
    let dot = vector.dot(&normalized);
    println!("Dot product with original: {dot}");
    assert!((dot - original_norm).abs() < 1e-9);
}
