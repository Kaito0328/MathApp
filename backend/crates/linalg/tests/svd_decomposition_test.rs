use linalg::Matrix;

#[cfg(test)]
mod svd_tests {
    use super::*;

    #[test]
    fn test_svd_2x2_matrix() {
        println!("=== SVD Decomposition 2x2 Matrix Test ===");

        let matrix = Matrix::new(2, 2, vec![3.0, 2.0, 2.0, 3.0]).unwrap();
        println!("Original matrix A: {matrix}");

        match matrix.svd() {
            Some(svd_result) => {
                println!("SVD decomposition successful!");
                println!("U matrix: {}", svd_result.u);
                println!("Singular values (sigma): {:?}", svd_result.sigma);
                println!("V matrix: {}", svd_result.v);

                // 特異値から対角行列Sを作成
                let mut s_matrix = Matrix::zeros(matrix.rows, matrix.cols);
                for i in 0..svd_result.sigma.dim() {
                    if i < matrix.rows.min(matrix.cols) {
                        s_matrix[(i, i)] = svd_result.sigma[i];
                    }
                }
                println!("S matrix (diagonal): {s_matrix}");

                // SVDの性質をテスト: A = U * S * V^T
                let v_transpose = svd_result.v.transpose();
                let reconstructed = &svd_result.u * &s_matrix * &v_transpose;
                println!("Reconstructed matrix (U * S * V^T): {reconstructed}");

                // 元の行列と再構成された行列の比較
                assert_eq!(reconstructed.rows, matrix.rows);
                assert_eq!(reconstructed.cols, matrix.cols);

                for i in 0..matrix.rows {
                    for j in 0..matrix.cols {
                        let diff = (reconstructed[(i, j)] - matrix[(i, j)]).abs();
                        println!(
                            "Element ({i}, {j}): original = {}, reconstructed = {}, diff = {}",
                            matrix[(i, j)],
                            reconstructed[(i, j)],
                            diff
                        );
                        assert!(diff < 1e-10, "Reconstruction error too large at ({i}, {j})");
                    }
                }

                println!("SVD reconstruction verified successfully!");
            }
            None => {
                println!("SVD decomposition failed");
                panic!("SVD should succeed for this matrix");
            }
        }
    }

    #[test]
    fn test_svd_3x3_matrix() {
        println!("=== SVD Decomposition 3x3 Matrix Test ===");

        let matrix = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]).unwrap();

        println!("Original 3x3 matrix A: {matrix}");

        match matrix.svd() {
            Some(svd_result) => {
                println!("SVD decomposition successful!");
                println!(
                    "U matrix ({}x{}): {}",
                    svd_result.u.rows, svd_result.u.cols, svd_result.u
                );
                println!("Singular values: {:?}", svd_result.sigma);
                println!(
                    "V matrix ({}x{}): {}",
                    svd_result.v.rows, svd_result.v.cols, svd_result.v
                );

                // 次元のチェック
                assert_eq!(svd_result.u.rows, matrix.rows);
                assert_eq!(svd_result.u.cols, matrix.cols); // U should match matrix size
                assert_eq!(svd_result.v.rows, matrix.cols);
                assert_eq!(svd_result.v.cols, matrix.cols); // V should be square
                assert_eq!(svd_result.sigma.dim(), matrix.rows.min(matrix.cols));

                println!("Matrix dimensions verified:");
                println!("  U: {}x{}", svd_result.u.rows, svd_result.u.cols);
                println!("  Singular values count: {}", svd_result.sigma.dim());
                println!("  V: {}x{}", svd_result.v.rows, svd_result.v.cols);

                // 特異値から対角行列Sを作成
                let mut s_matrix = Matrix::zeros(matrix.rows, matrix.cols);
                for i in 0..svd_result.sigma.dim() {
                    s_matrix[(i, i)] = svd_result.sigma[i];
                }

                // 再構成テスト: A = U * S * V^T
                let v_transpose = svd_result.v.transpose();
                let reconstructed = &svd_result.u * &s_matrix * &v_transpose;
                println!("Reconstructed matrix: {reconstructed}");

                for i in 0..matrix.rows {
                    for j in 0..matrix.cols {
                        let diff = (reconstructed[(i, j)] - matrix[(i, j)]).abs();
                        println!("Element ({i}, {j}): original = {:.6}, reconstructed = {:.6}, diff = {:.2e}", 
                                matrix[(i, j)], reconstructed[(i, j)], diff);
                        assert!(diff < 1e-8); // 3x3行列では精度が少し落ちる可能性がある
                    }
                }

                println!("3x3 SVD reconstruction verified!");
            }
            None => {
                println!("SVD decomposition failed for 3x3 matrix");
                panic!("SVD should handle 3x3 matrices");
            }
        }
    }

    #[test]
    fn test_svd_rectangular_matrix_tall() {
        println!("=== SVD Decomposition Rectangular Matrix (Tall) Test ===");

        let matrix = Matrix::new(3, 2, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();

        println!("Original tall matrix A (3x2): {matrix}");

        match matrix.svd() {
            Some(svd_result) => {
                println!("SVD decomposition successful for tall matrix!");
                println!("U matrix: {}", svd_result.u);
                println!("Singular values: {:?}", svd_result.sigma);
                println!("V matrix: {}", svd_result.v);

                // 次元チェック
                println!("Dimension check:");
                println!("  Original: {}x{}", matrix.rows, matrix.cols);
                println!("  U: {}x{}", svd_result.u.rows, svd_result.u.cols);
                println!("  Singular values count: {}", svd_result.sigma.dim());
                println!("  V: {}x{}", svd_result.v.rows, svd_result.v.cols);

                // 特異値から対角行列Sを作成
                let mut s_matrix = Matrix::zeros(matrix.rows, matrix.cols);
                for i in 0..svd_result.sigma.dim() {
                    s_matrix[(i, i)] = svd_result.sigma[i];
                }

                // 再構成
                let v_transpose = svd_result.v.transpose();
                let reconstructed = &svd_result.u * &s_matrix * &v_transpose;
                println!("Reconstructed matrix: {reconstructed}");

                for i in 0..matrix.rows {
                    for j in 0..matrix.cols {
                        let diff = (reconstructed[(i, j)] - matrix[(i, j)]).abs();
                        println!(
                            "Element ({i}, {j}): original = {}, reconstructed = {}, diff = {:.2e}",
                            matrix[(i, j)],
                            reconstructed[(i, j)],
                            diff
                        );
                        assert!(diff < 1e-8);
                    }
                }

                println!("Tall matrix SVD verified!");
            }
            None => {
                println!("SVD failed for tall rectangular matrix");
                panic!("SVD should handle rectangular matrices");
            }
        }
    }

    #[test]
    fn test_svd_rectangular_matrix_wide() {
        println!("=== SVD Decomposition Rectangular Matrix (Wide) Test ===");

        let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();

        println!("Original wide matrix A (2x3): {matrix}");

        match matrix.svd() {
            Some(svd_result) => {
                println!("SVD decomposition successful for wide matrix!");
                println!("U matrix: {}", svd_result.u);
                println!("Singular values: {:?}", svd_result.sigma);
                println!("V matrix: {}", svd_result.v);

                println!("Dimension verification:");
                println!("  Original: {}x{}", matrix.rows, matrix.cols);
                println!("  U: {}x{}", svd_result.u.rows, svd_result.u.cols);
                println!("  Singular values count: {}", svd_result.sigma.dim());
                println!("  V: {}x{}", svd_result.v.rows, svd_result.v.cols);

                // 特異値から対角行列Sを作成
                let mut s_matrix = Matrix::zeros(matrix.rows, matrix.cols);
                for i in 0..svd_result.sigma.dim() {
                    s_matrix[(i, i)] = svd_result.sigma[i];
                }

                let v_transpose = svd_result.v.transpose();
                let reconstructed = &svd_result.u * &s_matrix * &v_transpose;
                println!("Reconstructed matrix: {reconstructed}");

                for i in 0..matrix.rows {
                    for j in 0..matrix.cols {
                        let diff = (reconstructed[(i, j)] - matrix[(i, j)]).abs();
                        println!(
                            "Element ({i}, {j}): original = {}, reconstructed = {}, diff = {:.2e}",
                            matrix[(i, j)],
                            reconstructed[(i, j)],
                            diff
                        );
                        assert!(diff < 1e-8);
                    }
                }

                println!("Wide matrix SVD verified!");
            }
            None => {
                println!("SVD failed for wide rectangular matrix");
                panic!("SVD should handle wide matrices");
            }
        }
    }

    #[test]
    fn test_svd_singular_values_ordering() {
        println!("=== SVD Singular Values Ordering Test ===");

        let matrix = Matrix::new(3, 3, vec![4.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 2.0]).unwrap();

        println!("Diagonal matrix A: {matrix}");

        match matrix.svd() {
            Some(svd_result) => {
                println!("SVD decomposition successful!");
                println!("U matrix: {}", svd_result.u);
                println!("Singular values: {:?}", svd_result.sigma);
                println!("V matrix: {}", svd_result.v);

                // 特異値を表示
                for i in 0..svd_result.sigma.dim() {
                    println!("Singular value σ{i}: {}", svd_result.sigma[i]);
                }

                // 特異値が降順にソートされているかチェック
                for i in 1..svd_result.sigma.dim() {
                    assert!(
                        svd_result.sigma[i - 1] >= svd_result.sigma[i],
                        "Singular values should be in descending order: {} >= {}",
                        svd_result.sigma[i - 1],
                        svd_result.sigma[i]
                    );
                    println!(
                        "Order check: σ{} = {} >= σ{} = {} ✓",
                        i - 1,
                        svd_result.sigma[i - 1],
                        i,
                        svd_result.sigma[i]
                    );
                }

                println!("Singular values are properly ordered!");
            }
            None => {
                println!("SVD failed for diagonal matrix");
                panic!("SVD should work for diagonal matrices");
            }
        }
    }

    #[test]
    fn test_svd_orthogonality_properties() {
        println!("=== SVD Orthogonality Properties Test ===");

        let matrix = Matrix::new(3, 3, vec![2.0, 1.0, 0.0, 1.0, 2.0, 1.0, 0.0, 1.0, 2.0]).unwrap();

        println!("Symmetric matrix A: {matrix}");

        match matrix.svd() {
            Some(svd_result) => {
                println!("SVD successful!");
                println!("U matrix: {}", svd_result.u);
                println!("V matrix: {}", svd_result.v);

                // U^T * U = I をテスト
                let u_transpose = svd_result.u.transpose();
                let u_t_u = &u_transpose * &svd_result.u;
                println!("U^T * U: {u_t_u}");

                let identity_u = Matrix::<f64>::identity(svd_result.u.cols);
                println!("Expected identity for U: {identity_u}");

                for i in 0..svd_result.u.cols {
                    for j in 0..svd_result.u.cols {
                        let expected = if i == j { 1.0 } else { 0.0 };
                        let diff = (u_t_u[(i, j)] - expected).abs();
                        println!(
                            "U^T*U element ({i}, {j}): {:.6}, expected: {}, diff: {:.2e}",
                            u_t_u[(i, j)],
                            expected,
                            diff
                        );
                        assert!(diff < 1e-8, "U should be orthogonal");
                    }
                }

                // V^T * V = I をテスト
                let v_transpose = svd_result.v.transpose();
                let v_t_v = &v_transpose * &svd_result.v;
                println!("V^T * V: {v_t_v}");

                for i in 0..v_t_v.rows {
                    for j in 0..v_t_v.cols {
                        let expected = if i == j { 1.0 } else { 0.0 };
                        let diff = (v_t_v[(i, j)] - expected).abs();
                        println!(
                            "V^T*V element ({i}, {j}): {:.6}, expected: {}, diff: {:.2e}",
                            v_t_v[(i, j)],
                            expected,
                            diff
                        );
                        assert!(diff < 1e-8, "V should be orthogonal");
                    }
                }

                println!("Orthogonality properties verified!");
            }
            None => {
                println!("SVD failed");
                panic!("SVD should work for symmetric matrices");
            }
        }
    }

    #[test]
    fn test_svd_rank_deficient_matrix() {
        println!("=== SVD Rank Deficient Matrix Test ===");

        // ランクが2の3x3行列を作成（第3行は第1行と第2行の線形結合）
        let matrix = Matrix::new(
            3,
            3,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 5.0, 7.0, 9.0, // row3 = row1 + row2
            ],
        )
        .unwrap();

        println!("Rank deficient matrix A: {matrix}");
        println!("Note: Row 3 = Row 1 + Row 2, so rank should be 2");

        match matrix.svd() {
            Some(svd_result) => {
                println!("SVD successful for rank deficient matrix!");
                println!("U matrix: {}", svd_result.u);
                println!("Singular values: {:?}", svd_result.sigma);
                println!("V matrix: {}", svd_result.v);

                // 特異値をチェック
                for i in 0..svd_result.sigma.dim() {
                    println!("Singular value σ{i}: {:.10}", svd_result.sigma[i]);
                }

                // ランク不足の場合、いくつかの特異値が0に近いはず
                let tolerance = 1e-10;
                let mut non_zero_count = 0;
                for i in 0..svd_result.sigma.dim() {
                    let sv = svd_result.sigma[i];
                    if sv > tolerance {
                        non_zero_count += 1;
                        println!("σ{i} = {sv:.10} > {tolerance:.2e} (non-zero)");
                    } else {
                        println!("σ{i} = {sv:.10} ≤ {tolerance:.2e} (zero)");
                    }
                }

                println!("Number of non-zero singular values: {non_zero_count}");
                println!("Matrix rank (estimated): {non_zero_count}");

                // 特異値から対角行列Sを作成
                let mut s_matrix = Matrix::zeros(matrix.rows, matrix.cols);
                for i in 0..svd_result.sigma.dim() {
                    s_matrix[(i, i)] = svd_result.sigma[i];
                }

                // 再構成テスト
                let v_transpose = svd_result.v.transpose();
                let reconstructed = &svd_result.u * &s_matrix * &v_transpose;
                println!("Reconstructed matrix: {reconstructed}");

                for i in 0..matrix.rows {
                    for j in 0..matrix.cols {
                        let diff = (reconstructed[(i, j)] - matrix[(i, j)]).abs();
                        println!("Element ({i}, {j}): original = {:.6}, reconstructed = {:.6}, diff = {:.2e}", 
                                matrix[(i, j)], reconstructed[(i, j)], diff);
                        assert!(diff < 1e-6); // ランク不足行列では精度が落ちる
                    }
                }

                println!("Rank deficient matrix SVD verified!");
            }
            None => {
                println!("SVD failed for rank deficient matrix");
                panic!("SVD should handle rank deficient matrices");
            }
        }
    }

    #[test]
    fn test_svd_zero_matrix() {
        println!("=== SVD Zero Matrix Test ===");

        let matrix = Matrix::<f64>::zeros(3, 3);
        println!("Zero matrix A: {matrix}");

        match matrix.svd() {
            Some(svd_result) => {
                println!("SVD successful for zero matrix!");
                println!("U matrix: {}", svd_result.u);
                println!("Singular values: {:?}", svd_result.sigma);
                println!("V matrix: {}", svd_result.v);

                // 全ての特異値が0であることをテスト
                for i in 0..svd_result.sigma.dim() {
                    println!("Singular value σ{i}: {:.15}", svd_result.sigma[i]);
                    assert!(
                        svd_result.sigma[i].abs() < 1e-15,
                        "All singular values of zero matrix should be zero"
                    );
                }

                // 特異値から対角行列Sを作成
                let mut s_matrix = Matrix::zeros(matrix.rows, matrix.cols);
                for i in 0..svd_result.sigma.dim() {
                    s_matrix[(i, i)] = svd_result.sigma[i];
                }

                // 再構成は元の零行列になるはず
                let v_transpose = svd_result.v.transpose();
                let reconstructed = &svd_result.u * &s_matrix * &v_transpose;
                println!("Reconstructed matrix: {reconstructed}");

                for i in 0..matrix.rows {
                    for j in 0..matrix.cols {
                        let val = reconstructed[(i, j)];
                        println!("Reconstructed element ({i}, {j}): {val:.15}");
                        assert!(
                            val.abs() < 1e-15,
                            "Reconstructed zero matrix should be zero"
                        );
                    }
                }

                println!("Zero matrix SVD verified!");
            }
            None => {
                println!("SVD failed for zero matrix");
                panic!("SVD should handle zero matrices");
            }
        }
    }

    #[test]
    fn test_svd_identity_matrix() {
        println!("=== SVD Identity Matrix Test ===");

        let matrix = Matrix::<f64>::identity(3);
        println!("Identity matrix A: {matrix}");

        match matrix.svd() {
            Some(svd_result) => {
                println!("SVD successful for identity matrix!");
                println!("U matrix: {}", svd_result.u);
                println!("Singular values: {:?}", svd_result.sigma);
                println!("V matrix: {}", svd_result.v);

                // 単位行列の特異値は全て1であるはず
                for i in 0..svd_result.sigma.dim() {
                    println!("Singular value σ{i}: {:.10}", svd_result.sigma[i]);
                    assert!(
                        (svd_result.sigma[i] - 1.0).abs() < 1e-10,
                        "Singular values of identity matrix should be 1"
                    );
                }

                // 特異値から対角行列Sを作成
                let mut s_matrix = Matrix::zeros(matrix.rows, matrix.cols);
                for i in 0..svd_result.sigma.dim() {
                    s_matrix[(i, i)] = svd_result.sigma[i];
                }

                // 再構成テスト
                let v_transpose = svd_result.v.transpose();
                let reconstructed = &svd_result.u * &s_matrix * &v_transpose;
                println!("Reconstructed matrix: {reconstructed}");

                for i in 0..matrix.rows {
                    for j in 0..matrix.cols {
                        let expected = if i == j { 1.0 } else { 0.0 };
                        let diff = (reconstructed[(i, j)] - expected).abs();
                        println!("Element ({i}, {j}): reconstructed = {:.10}, expected = {}, diff = {:.2e}", 
                                reconstructed[(i, j)], expected, diff);
                        assert!(diff < 1e-10);
                    }
                }

                println!("Identity matrix SVD verified!");
            }
            None => {
                println!("SVD failed for identity matrix");
                panic!("SVD should handle identity matrices");
            }
        }
    }
}
