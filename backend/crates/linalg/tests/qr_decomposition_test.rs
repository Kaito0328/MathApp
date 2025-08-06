#[cfg(test)]
mod qr_tests {
    use linalg::Matrix;
    #[test]
    fn test_qr_decomposition_basic() {
        println!("Testing basic QR decomposition...");
        let matrix = Matrix::new(
            3,
            3,
            vec![12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0],
        )
        .unwrap();

        println!("Original matrix A:");
        println!("{matrix}");

        let result = matrix.qr_decomposition();
        assert!(result.is_some());

        let (q, r) = result.unwrap();

        println!("Q matrix:");
        println!("{q}");
        println!("R matrix:");
        println!("{r}");

        // A = QR が成り立つことを確認
        let qr = &q * &r;
        println!("QR product:");
        println!("{qr}");

        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (matrix[(i, j)] - qr[(i, j)]).abs() < 1e-10,
                    "A != QR at ({}, {}): {} vs {}",
                    i,
                    j,
                    matrix[(i, j)],
                    qr[(i, j)]
                );
            }
        }

        // Q が直交行列であることを確認 (Q^T * Q = I)
        let qt = q.transpose();
        let qtq = &qt * &q;
        println!("Q^T * Q (should be identity):");
        println!("{qtq}");

        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (qtq[(i, j)] - expected).abs() < 1e-10,
                    "Q^T * Q is not identity at ({}, {}): {}",
                    i,
                    j,
                    qtq[(i, j)]
                );
            }
        }

        // R が上三角行列であることを確認
        for i in 1..3 {
            for j in 0..i {
                assert!(
                    r[(i, j)].abs() < 1e-10,
                    "R is not upper triangular at ({}, {}): {}",
                    i,
                    j,
                    r[(i, j)]
                );
            }
        }
        println!("Basic QR decomposition test passed!\n");
    }

    #[test]
    fn test_qr_decomposition_identity() {
        println!("Testing QR decomposition of identity matrix...");
        let matrix = Matrix::identity(3);

        println!("Identity matrix:");
        println!("{matrix}");

        let result = matrix.qr_decomposition();
        assert!(result.is_some());

        let (q, r) = result.unwrap();

        println!("Q matrix:");
        println!("{q}");
        println!("R matrix:");
        println!("{r}");

        // 単位行列の場合、Q = I, R = I
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (q[(i, j)] - expected).abs() < 1e-10,
                    "Q should be identity at ({}, {}): {}",
                    i,
                    j,
                    q[(i, j)]
                );
                assert!(
                    (r[(i, j)] - expected).abs() < 1e-10,
                    "R should be identity at ({}, {}): {}",
                    i,
                    j,
                    r[(i, j)]
                );
            }
        }
        println!("Identity matrix QR decomposition test passed!\n");
    }

    #[test]
    fn test_qr_decomposition_tall_matrix() {
        println!("Testing QR decomposition of tall matrix (4x3)...");
        let matrix = Matrix::new(
            4,
            3,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
            ],
        )
        .unwrap();

        println!("Original tall matrix A:");
        println!("{matrix}");

        let result = matrix.qr_decomposition();
        assert!(result.is_some());

        let (q, r) = result.unwrap();

        println!("Q matrix (4x4):");
        println!("{q}");
        println!("R matrix (4x3):");
        println!("{r}");

        // A = QR が成り立つことを確認
        let qr = &q * &r;
        println!("QR product:");
        println!("{qr}");

        for i in 0..4 {
            for j in 0..3 {
                assert!(
                    (matrix[(i, j)] - qr[(i, j)]).abs() < 1e-10,
                    "A != QR at ({}, {}): {} vs {}",
                    i,
                    j,
                    matrix[(i, j)],
                    qr[(i, j)]
                );
            }
        }

        // Q が直交行列であることを確認
        let qt = q.transpose();
        let qtq = &qt * &q;
        println!("Q^T * Q (should be identity):");
        println!("{qtq}");

        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (qtq[(i, j)] - expected).abs() < 1e-10,
                    "Q^T * Q is not identity at ({}, {}): {}",
                    i,
                    j,
                    qtq[(i, j)]
                );
            }
        }
        println!("Tall matrix QR decomposition test passed!\n");
    }

    #[test]
    fn test_qr_decomposition_wide_matrix() {
        println!("Testing QR decomposition of wide matrix (should fail)...");
        let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();

        println!("Wide matrix (2x3):");
        println!("{matrix}");

        let result = matrix.qr_decomposition();

        assert!(result.is_some());

        let (q, r) = result.unwrap();

        println!("Q matrix (2x2):");
        println!("{q}");
        println!("R matrix (2x3):");
        println!("{r}");

        // A = QR が成り立つことを確認
        let qr = &q * &r;
        println!("QR product:");
        println!("{qr}");

        for i in 0..2 {
            for j in 0..3 {
                assert!(
                    (matrix[(i, j)] - qr[(i, j)]).abs() < 1e-10,
                    "A != QR at ({}, {}): {} vs {}",
                    i,
                    j,
                    matrix[(i, j)],
                    qr[(i, j)]
                );
            }
        }

        // Q が直交行列であることを確認
        let qt = q.transpose();
        let qtq = &qt * &q;
        println!("Q^T * Q (should be identity):");
        println!("{qtq}");

        for i in 0..2 {
            for j in 0..2 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (qtq[(i, j)] - expected).abs() < 1e-10,
                    "Q^T * Q is not identity at ({}, {}): {}",
                    i,
                    j,
                    qtq[(i, j)]
                );
            }
        }
        println!("Tall matrix QR decomposition test passed!\n");
    }

    #[test]
    fn test_qr_decomposition_orthogonal_columns() {
        println!("Testing QR decomposition of matrix with orthogonal columns...");
        let matrix = Matrix::new(3, 3, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]).unwrap();

        println!("Matrix with orthogonal columns:");
        println!("{matrix}");

        let result = matrix.qr_decomposition();
        assert!(result.is_some());

        let (q, r) = result.unwrap();

        println!("Q matrix:");
        println!("{q}");
        println!("R matrix:");
        println!("{r}");

        // 直交列を持つ行列の場合、Qは単位行列に近く、Rは対角行列
        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    assert!(r[(i, j)].abs() > 0.5, "Diagonal element should be non-zero");
                } else if i > j {
                    assert!(r[(i, j)].abs() < 1e-10, "Lower triangular should be zero");
                }
            }
        }
        println!("Orthogonal columns QR decomposition test passed!\n");
    }

    #[test]
    fn test_qr_decomposition_negative_values() {
        println!("Testing QR decomposition with negative values...");
        let matrix =
            Matrix::new(3, 3, vec![-2.0, 1.0, 3.0, 4.0, -5.0, 6.0, -7.0, 8.0, -9.0]).unwrap();

        println!("Matrix with negative values:");
        println!("{matrix}");

        let result = matrix.qr_decomposition();
        assert!(result.is_some());

        let (q, r) = result.unwrap();

        println!("Q matrix:");
        println!("{q}");
        println!("R matrix:");
        println!("{r}");

        // A = QR の確認
        let qr = &q * &r;
        println!("QR product:");
        println!("{qr}");

        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (matrix[(i, j)] - qr[(i, j)]).abs() < 1e-10,
                    "A != QR at ({}, {}): {} vs {}",
                    i,
                    j,
                    matrix[(i, j)],
                    qr[(i, j)]
                );
            }
        }
        println!("Negative values QR decomposition test passed!\n");
    }

    #[test]
    fn test_qr_decomposition_1x1() {
        println!("Testing QR decomposition of 1x1 matrix...");
        let matrix = Matrix::new(1, 1, vec![5.0]).unwrap();

        println!("1x1 matrix:");
        println!("{matrix}");

        let result = matrix.qr_decomposition();
        assert!(result.is_some());

        let (q, r) = result.unwrap();

        println!("Q matrix:");
        println!("{q}");
        println!("R matrix:");
        println!("{r}");

        assert!(
            (q[(0, 0)] - 1.0).abs() < 1e-10 || (q[(0, 0)] + 1.0).abs() < 1e-10,
            "Q should be ±1 for 1x1 case"
        );
        assert!((r[(0, 0)]).abs() > 1e-10, "R should be non-zero");
        assert!(
            (matrix[(0, 0)] - (q[(0, 0)] * r[(0, 0)])).abs() < 1e-10,
            "A should equal QR"
        );
        println!("1x1 matrix QR decomposition test passed!\n");
    }

    #[test]
    fn test_qr_decomposition_zero_column() {
        println!("Testing QR decomposition with zero column...");
        let matrix = Matrix::new(3, 3, vec![1.0, 0.0, 3.0, 2.0, 0.0, 6.0, 3.0, 0.0, 9.0]).unwrap();

        println!("Matrix with zero column:");
        println!("{matrix}");

        let result = matrix.qr_decomposition();
        assert!(result.is_some());

        let (q, r) = result.unwrap();

        println!("Q matrix:");
        println!("{q}");
        println!("R matrix:");
        println!("{r}");

        // A = QR の確認
        let qr = &q * &r;
        println!("QR product:");
        println!("{qr}");

        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (matrix[(i, j)] - qr[(i, j)]).abs() < 1e-10,
                    "A != QR at ({}, {}): {} vs {}",
                    i,
                    j,
                    matrix[(i, j)],
                    qr[(i, j)]
                );
            }
        }

        // ゼロ列に対応するRの列もゼロであることを確認
        for i in 0..3 {
            assert!(
                r[(i, 1)].abs() < 1e-10,
                "R column 1 should be zero at row {}: {}",
                i,
                r[(i, 1)]
            );
        }
        println!("Zero column QR decomposition test passed!\n");
    }

    #[test]
    fn test_qr_decomposition_householder_stability() {
        println!("Testing QR decomposition numerical stability...");
        let matrix = Matrix::new(
            4,
            4,
            vec![
                1e-10, 1.0, 0.0, 0.0, 0.0, 1e-10, 1.0, 0.0, 0.0, 0.0, 1e-10, 1.0, 0.0, 0.0, 0.0,
                1e-10,
            ],
        )
        .unwrap();

        println!("Matrix with small diagonal elements:");
        println!("{matrix}");

        let result = matrix.qr_decomposition();
        assert!(result.is_some());

        let (q, r) = result.unwrap();

        println!("Q matrix:");
        println!("{q}");
        println!("R matrix:");
        println!("{r}");

        // A = QR の確認
        let qr = &q * &r;
        println!("QR product:");
        println!("{qr}");

        for i in 0..4 {
            for j in 0..4 {
                assert!(
                    (matrix[(i, j)] - qr[(i, j)]).abs() < 1e-8,
                    "A != QR at ({}, {}): {} vs {}",
                    i,
                    j,
                    matrix[(i, j)],
                    qr[(i, j)]
                );
            }
        }

        // Q の直交性確認
        let qt = q.transpose();
        let qtq = &qt * &q;

        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (qtq[(i, j)] - expected).abs() < 1e-8,
                    "Q^T * Q is not identity at ({}, {}): {}",
                    i,
                    j,
                    qtq[(i, j)]
                );
            }
        }
        println!("Numerical stability test passed!\n");
    }

    #[test]
    fn test_qr_decomposition_large_matrix() {
        println!("Testing QR decomposition of larger matrix (5x5)...");
        let matrix = Matrix::new(
            5,
            5,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 2.0, 4.0, 6.0, 8.0, 10.0, 3.0, 6.0, 9.0, 12.0, 15.0, 4.0,
                8.0, 12.0, 16.0, 20.0, 5.0, 10.0, 15.0, 20.0, 25.0,
            ],
        )
        .unwrap();

        println!("5x5 matrix:");
        println!("{matrix}");

        let result = matrix.qr_decomposition();
        assert!(result.is_some());

        let (q, r) = result.unwrap();

        println!("Q matrix (first 3 rows shown):");
        for i in 0..3 {
            for j in 0..5 {
                print!("{:.6}\t", q[(i, j)]);
            }
            println!();
        }
        println!("...");

        println!("R matrix (first 3 rows shown):");
        for i in 0..3 {
            for j in 0..5 {
                print!("{:.6}\t", r[(i, j)]);
            }
            println!();
        }
        println!("...");

        // A = QR の確認（部分的に）
        let qr = &q * &r;
        let mut max_error = 0.0;
        for i in 0..5 {
            for j in 0..5 {
                let error = (matrix[(i, j)] - qr[(i, j)]).abs();
                if error > max_error {
                    max_error = error;
                }
            }
        }

        println!("Maximum reconstruction error: {max_error:.2e}");
        assert!(
            max_error < 1e-10,
            "Reconstruction error too large: {max_error}",
        );
        println!("Large matrix QR decomposition test passed!\n");
    }

    #[test]
    fn test_qr_decomposition_random_case() {
        println!("Testing QR decomposition with specific numerical case...");
        let matrix = Matrix::new(
            3,
            3,
            vec![
                0.8147, 0.9134, 0.2785, 0.9058, 0.6324, 0.5469, 0.1270, 0.0975, 0.9575,
            ],
        )
        .unwrap();

        println!("Test matrix:");
        println!("{matrix}");

        let result = matrix.qr_decomposition();
        assert!(result.is_some());

        let (q, r) = result.unwrap();

        println!("Q matrix:");
        println!("{q}");
        println!("R matrix:");
        println!("{r}");

        // A = QR の確認
        let qr = &q * &r;
        println!("QR product:");
        println!("{qr}");

        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (matrix[(i, j)] - qr[(i, j)]).abs() < 1e-10,
                    "A != QR at ({}, {}): {} vs {}",
                    i,
                    j,
                    matrix[(i, j)],
                    qr[(i, j)]
                );
            }
        }

        // Q の各列のノルムが1であることを確認
        for j in 0..3 {
            let col = q.col(j).unwrap();
            let norm = col.norm();
            println!("Column {j} norm: {norm:.10}");
            assert!(
                (norm - 1.0).abs() < 1e-10,
                "Q column {j} norm is not 1: {norm}",
            );
        }

        // Q の列が互いに直交していることを確認
        for i in 0..3 {
            for j in i + 1..3 {
                let col_i = q.col(i).unwrap();
                let col_j = q.col(j).unwrap();
                let dot_product = col_i.dot(&col_j);
                println!("Dot product of columns {i} and {j}: {dot_product:.2e}",);
                assert!(
                    dot_product.abs() < 1e-10,
                    "Q columns {i} and {j} are not orthogonal: {dot_product}"
                );
            }
        }
        println!("Random case QR decomposition test passed!\n");
    }
}
