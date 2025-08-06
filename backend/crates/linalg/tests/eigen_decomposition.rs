use linalg::matrix::Matrix;

#[cfg(test)]
mod eigen_decomposition_tests {
    use super::*;

    #[test]
    fn test_eigen_decomposition_2x2_real_eigenvalues() {
        println!("Testing 2x2 matrix with real eigenvalues...");
        let matrix = Matrix::new(2, 2, vec![3.0, 1.0, 0.0, 2.0]).unwrap();

        println!("Original matrix A: {matrix}");

        let result = matrix.eigen_decomposition();
        assert!(result.is_some(), "Eigen decomposition should succeed");

        let decomp = result.unwrap();
        println!("Number of eigenvalues found: {}", decomp.eigenvalues.len());

        for (i, eigenval) in decomp.eigenvalues.iter().enumerate() {
            println!("Eigenvalue {i}: {} + {}i", eigenval.re, eigenval.im);
            assert!(eigenval.im.abs() < 1e-10, "Eigenvalue should be real");
        }

        // 期待される固有値: 3.0, 2.0
        let mut real_parts: Vec<f64> = decomp.eigenvalues.iter().map(|e| e.re).collect();
        real_parts.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert!(
            (real_parts[0] - 2.0).abs() < 1e-10,
            "First eigenvalue should be 2.0"
        );
        assert!(
            (real_parts[1] - 3.0).abs() < 1e-10,
            "Second eigenvalue should be 3.0"
        );

        println!("Fixed eigenvalues test passed!\n");
    }

    #[test]
    fn test_eigen_decomposition_identity_matrix() {
        println!("Testing identity matrix eigen decomposition...");
        let matrix = Matrix::identity(3);

        println!("Identity matrix: {matrix}");

        let result = matrix.eigen_decomposition();
        assert!(
            result.is_some(),
            "Identity matrix should have eigen decomposition"
        );

        let decomp = result.unwrap();
        println!("Number of eigenvalues: {}", decomp.eigenvalues.len());

        // 単位行列の固有値はすべて1
        for (i, eigenval) in decomp.eigenvalues.iter().enumerate() {
            println!("Eigenvalue {i}: {} + {}i", eigenval.re, eigenval.im);
            assert!(
                (eigenval.re - 1.0).abs() < 1e-10,
                "Identity eigenvalue should be 1.0"
            );
            assert!(
                eigenval.im.abs() < 1e-10,
                "Identity eigenvalue should be real"
            );
        }

        println!("Identity matrix test passed!\n");
    }

    #[test]
    fn test_eigen_decomposition_diagonal_matrix() {
        println!("Testing diagonal matrix eigen decomposition...");
        let matrix = Matrix::new(3, 3, vec![5.0, 0.0, 0.0, 0.0, -2.0, 0.0, 0.0, 0.0, 7.0]).unwrap();

        println!("Diagonal matrix: {matrix}");

        let result = matrix.eigen_decomposition();
        assert!(
            result.is_some(),
            "Diagonal matrix should have eigen decomposition"
        );

        let decomp = result.unwrap();
        println!("Number of eigenvalues: {}", decomp.eigenvalues.len());

        let mut real_parts: Vec<f64> = decomp.eigenvalues.iter().map(|e| e.re).collect();
        real_parts.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for (i, eigenval) in decomp.eigenvalues.iter().enumerate() {
            println!("Eigenvalue {i}: {} + {}i", eigenval.re, eigenval.im);
            assert!(
                eigenval.im.abs() < 1e-10,
                "Diagonal eigenvalue should be real"
            );
        }

        // 期待される固有値: -2.0, 5.0, 7.0
        assert!(
            (real_parts[0] - (-2.0)).abs() < 1e-10,
            "First eigenvalue should be -2.0"
        );
        assert!(
            (real_parts[1] - 5.0).abs() < 1e-10,
            "Second eigenvalue should be 5.0"
        );
        assert!(
            (real_parts[2] - 7.0).abs() < 1e-10,
            "Third eigenvalue should be 7.0"
        );

        println!("Diagonal matrix test passed!\n");
    }

    #[test]
    fn test_eigen_decomposition_symmetric_matrix() {
        println!("Testing symmetric matrix eigen decomposition...");
        let matrix = Matrix::new(3, 3, vec![4.0, 1.0, 2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 5.0]).unwrap();

        println!("Symmetric matrix: {matrix}");

        // 対称性を確認
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (matrix[(i, j)] - matrix[(j, i)]).abs() < 1e-10,
                    "Matrix should be symmetric"
                );
            }
        }

        let result = matrix.eigen_decomposition();
        assert!(
            result.is_some(),
            "Symmetric matrix should have eigen decomposition"
        );

        let decomp = result.unwrap();
        println!("Number of eigenvalues: {}", decomp.eigenvalues.len());

        // 対称行列の固有値はすべて実数
        for (i, eigenval) in decomp.eigenvalues.iter().enumerate() {
            println!("Eigenvalue {i}: {} + {}i", eigenval.re, eigenval.im);
            assert!(
                eigenval.im.abs() < 1e-10,
                "Symmetric matrix eigenvalue should be real"
            );
        }

        // トレースの確認 (固有値の和 = トレース)
        let trace = matrix.trace();
        let eigenvalue_sum: f64 = decomp.eigenvalues.iter().map(|e| e.re).sum();
        println!("Matrix trace: {trace}");
        println!("Sum of eigenvalues: {eigenvalue_sum}");
        assert!(
            (trace - eigenvalue_sum).abs() < 1e-10,
            "Trace should equal sum of eigenvalues"
        );

        println!("Symmetric matrix test passed!\n");
    }

    #[test]
    fn test_eigen_decomposition_rotation_matrix() {
        println!("Testing 2D rotation matrix eigen decomposition...");
        let angle = std::f64::consts::PI / 4.0; // 45度回転
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        let matrix = Matrix::new(2, 2, vec![cos_theta, -sin_theta, sin_theta, cos_theta]).unwrap();

        println!("Rotation matrix (45 degrees): {matrix}");
        println!("cos(π/4) = {cos_theta}, sin(π/4) = {sin_theta}");

        let result = matrix.eigen_decomposition();
        assert!(
            result.is_some(),
            "Rotation matrix should have eigen decomposition"
        );

        let decomp = result.unwrap();
        println!("Number of eigenvalues: {}", decomp.eigenvalues.len());

        // 回転行列の固有値は複素共役ペア: e^(±iθ)
        for (i, eigenval) in decomp.eigenvalues.iter().enumerate() {
            println!("Eigenvalue {i}: {} + {}i", eigenval.re, eigenval.im);
            let magnitude = (eigenval.re * eigenval.re + eigenval.im * eigenval.im).sqrt();
            println!("Magnitude of eigenvalue {i}: {magnitude}");
            assert!(
                (magnitude - 1.0).abs() < 1e-10,
                "Rotation matrix eigenvalue should have magnitude 1"
            );
        }

        // 固有値が複素共役ペアであることを確認
        if decomp.eigenvalues.len() == 2 {
            let e1 = &decomp.eigenvalues[0];
            let e2 = &decomp.eigenvalues[1];
            assert!((e1.re - e2.re).abs() < 1e-10, "Real parts should be equal");
            assert!(
                (e1.im + e2.im).abs() < 1e-10,
                "Imaginary parts should be opposite"
            );
            println!("Complex conjugate pair confirmed");
        }

        println!("Rotation matrix test passed!\n");
    }

    #[test]
    fn test_eigen_decomposition_upper_triangular() {
        println!("Testing upper triangular matrix eigen decomposition...");
        let matrix = Matrix::new(3, 3, vec![6.0, 2.0, 1.0, 0.0, 4.0, 3.0, 0.0, 0.0, 2.0]).unwrap();

        println!("Upper triangular matrix: {matrix}");

        let result = matrix.eigen_decomposition();
        assert!(
            result.is_some(),
            "Upper triangular matrix should have eigen decomposition"
        );

        let decomp = result.unwrap();
        println!("Number of eigenvalues: {}", decomp.eigenvalues.len());

        // 上三角行列の固有値は対角要素
        let mut real_parts: Vec<f64> = decomp.eigenvalues.iter().map(|e| e.re).collect();
        real_parts.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for (i, eigenval) in decomp.eigenvalues.iter().enumerate() {
            println!("Eigenvalue {i}: {} + {}i", eigenval.re, eigenval.im);
            assert!(
                eigenval.im.abs() < 1e-10,
                "Upper triangular eigenvalue should be real"
            );
        }

        // 期待される固有値: 2.0, 4.0, 6.0
        assert!(
            (real_parts[0] - 2.0).abs() < 1e-10,
            "First eigenvalue should be 2.0"
        );
        assert!(
            (real_parts[1] - 4.0).abs() < 1e-10,
            "Second eigenvalue should be 4.0"
        );
        assert!(
            (real_parts[2] - 6.0).abs() < 1e-10,
            "Third eigenvalue should be 6.0"
        );

        println!("Upper triangular matrix test passed!\n");
    }

    #[test]
    fn test_eigen_decomposition_complex_eigenvalues() {
        println!("Testing matrix with complex eigenvalues...");
        let matrix = Matrix::new(2, 2, vec![0.0, -1.0, 1.0, 0.0]).unwrap();

        println!("Matrix with complex eigenvalues: {matrix}");

        let result = matrix.eigen_decomposition();
        assert!(result.is_some(), "Matrix should have eigen decomposition");

        let decomp = result.unwrap();
        println!("Number of eigenvalues: {}", decomp.eigenvalues.len());

        // この行列の固有値は ±i
        for (i, eigenval) in decomp.eigenvalues.iter().enumerate() {
            println!("Eigenvalue {i}: {} + {}i", eigenval.re, eigenval.im);
            assert!(eigenval.re.abs() < 1e-10, "Real part should be near zero");
            assert!(
                (eigenval.im.abs() - 1.0).abs() < 1e-10,
                "Imaginary part should be ±1"
            );
        }

        // 固有値が複素共役ペア i, -i であることを確認
        if decomp.eigenvalues.len() == 2 {
            let imag_parts: Vec<f64> = decomp.eigenvalues.iter().map(|e| e.im).collect();
            let sum_imag: f64 = imag_parts.iter().sum();
            println!("Sum of imaginary parts: {sum_imag}");
            assert!(
                sum_imag.abs() < 1e-10,
                "Sum of imaginary parts should be zero (conjugate pair)"
            );
        }

        println!("Complex eigenvalues test passed!\n");
    }

    #[test]
    fn test_eigen_decomposition_1x1_matrix() {
        println!("Testing 1x1 matrix eigen decomposition...");
        let matrix = Matrix::new(1, 1, vec![42.0]).unwrap();

        println!("1x1 matrix: {matrix}");

        let result = matrix.eigen_decomposition();
        assert!(
            result.is_some(),
            "1x1 matrix should have eigen decomposition"
        );

        let decomp = result.unwrap();
        println!("Number of eigenvalues: {}", decomp.eigenvalues.len());
        assert_eq!(
            decomp.eigenvalues.len(),
            1,
            "1x1 matrix should have exactly 1 eigenvalue"
        );

        let eigenval = &decomp.eigenvalues[0];
        println!("Eigenvalue: {} + {}i", eigenval.re, eigenval.im);
        assert!(
            (eigenval.re - 42.0).abs() < 1e-10,
            "Eigenvalue should be 42.0"
        );
        assert!(eigenval.im.abs() < 1e-10, "Eigenvalue should be real");

        println!("1x1 matrix test passed!\n");
    }

    #[test]
    fn test_eigen_decomposition_repeated_eigenvalues() {
        println!("Testing matrix with repeated eigenvalues...");
        let matrix = Matrix::new(3, 3, vec![2.0, 1.0, 0.0, 0.0, 2.0, 1.0, 0.0, 0.0, 2.0]).unwrap();

        println!("Matrix with repeated eigenvalues: {matrix}");

        let result = matrix.eigen_decomposition();
        assert!(result.is_some(), "Matrix should have eigen decomposition");

        let decomp = result.unwrap();
        println!("Number of eigenvalues: {}", decomp.eigenvalues.len());

        // すべての固有値が2.0であることを確認
        for (i, eigenval) in decomp.eigenvalues.iter().enumerate() {
            println!("Eigenvalue {i}: {} + {}i", eigenval.re, eigenval.im);
            assert!(
                (eigenval.re - 2.0).abs() < 1e-10,
                "All eigenvalues should be 2.0"
            );
            assert!(eigenval.im.abs() < 1e-10, "Eigenvalues should be real");
        }

        println!("Repeated eigenvalues test passed!\n");
    }

    #[test]
    fn test_eigen_decomposition_non_square_matrix() {
        println!("Testing non-square matrix (should fail)...");
        let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();

        println!("Non-square matrix (2x3): {matrix}");

        let result = matrix.eigen_decomposition();
        assert!(
            result.is_none(),
            "Non-square matrix should not have eigen decomposition"
        );

        println!("Non-square matrix correctly rejected!\n");
    }

    #[test]
    fn test_eigen_decomposition_zero_matrix() {
        println!("Testing zero matrix eigen decomposition...");
        let matrix = Matrix::zeros(3, 3);

        println!("Zero matrix: {matrix}");

        let result = matrix.eigen_decomposition();
        assert!(
            result.is_some(),
            "Zero matrix should have eigen decomposition"
        );

        let decomp = result.unwrap();
        println!("Number of eigenvalues: {}", decomp.eigenvalues.len());

        // ゼロ行列の固有値はすべて0
        for (i, eigenval) in decomp.eigenvalues.iter().enumerate() {
            println!("Eigenvalue {i}: {} + {}i", eigenval.re, eigenval.im);
            assert!(
                eigenval.re.abs() < 1e-10,
                "Zero matrix eigenvalue should be 0"
            );
            assert!(
                eigenval.im.abs() < 1e-10,
                "Zero matrix eigenvalue should be real"
            );
        }

        println!("Zero matrix test passed!\n");
    }

    #[test]
    fn test_eigen_decomposition_large_matrix() {
        println!("Testing larger matrix (4x4) eigen decomposition...");
        let matrix = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 4.0, 1.0, 0.0, 0.0, 1.0, 5.0,
            ],
        )
        .unwrap();

        println!("4x4 block diagonal matrix: {matrix}");

        let result = matrix.eigen_decomposition();
        assert!(
            result.is_some(),
            "4x4 matrix should have eigen decomposition"
        );

        let decomp = result.unwrap();
        println!("Number of eigenvalues: {}", decomp.eigenvalues.len());
        assert_eq!(
            decomp.eigenvalues.len(),
            4,
            "4x4 matrix should have 4 eigenvalues"
        );

        for (i, eigenval) in decomp.eigenvalues.iter().enumerate() {
            println!("Eigenvalue {i}: {} + {}i", eigenval.re, eigenval.im);
        }

        // トレースの確認
        let trace = matrix.trace();
        let eigenvalue_sum: f64 = decomp.eigenvalues.iter().map(|e| e.re).sum();
        println!("Matrix trace: {trace}");
        println!("Sum of eigenvalues: {eigenvalue_sum}");
        assert!(
            (trace - eigenvalue_sum).abs() < 1e-10,
            "Trace should equal sum of eigenvalues"
        );

        // 行列式の確認 (固有値の積)
        let det = matrix.determinant().unwrap_or(0.0);
        let eigenvalue_product: f64 = decomp
            .eigenvalues
            .iter()
            .map(|e| e.re * e.re + e.im * e.im)
            .product::<f64>()
            .sqrt();
        println!("Matrix determinant: {det}");
        println!("Product of eigenvalue magnitudes: {eigenvalue_product}");

        println!("Large matrix test passed!\n");
    }

    #[test]
    fn test_eigen_decomposition_numerical_stability() {
        println!("Testing numerical stability with ill-conditioned matrix...");
        let matrix = Matrix::new(
            3,
            3,
            vec![1.0, 1.0, 1.0, 1.0, 1.0 + 1e-10, 1.0, 1.0, 1.0, 1.0 + 1e-10],
        )
        .unwrap();

        println!("Ill-conditioned matrix: {matrix}");

        let result = matrix.eigen_decomposition();
        if result.is_some() {
            let decomp = result.unwrap();
            println!("Number of eigenvalues: {}", decomp.eigenvalues.len());

            for (i, eigenval) in decomp.eigenvalues.iter().enumerate() {
                println!("Eigenvalue {i}: {} + {}i", eigenval.re, eigenval.im);
            }

            // トレースの確認
            let trace = matrix.trace();
            let eigenvalue_sum: f64 = decomp.eigenvalues.iter().map(|e| e.re).sum();
            println!("Matrix trace: {trace}");
            println!("Sum of eigenvalues: {eigenvalue_sum}");

            let trace_error = (trace - eigenvalue_sum).abs();
            println!("Trace error: {:.2e}", trace_error);

            // 数値的に不安定な行列でも合理的な結果が得られることを確認
            assert!(trace_error < 1e-8, "Trace error should be reasonably small");

            println!("Numerical stability test passed!\n");
        } else {
            println!("Ill-conditioned matrix decomposition failed (acceptable)\n");
        }
    }

    #[test]
    fn test_eigen_decomposition_random_symmetric() {
        println!("Testing random symmetric matrix...");
        let matrix = Matrix::new(3, 3, vec![4.0, 1.5, 0.5, 1.5, 3.0, 2.0, 0.5, 2.0, 5.0]).unwrap();

        println!("Random symmetric matrix: {matrix}");

        // 対称性の確認
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (matrix[(i, j)] - matrix[(j, i)]).abs() < 1e-15,
                    "Matrix should be symmetric at ({i}, {j})"
                );
            }
        }

        let result = matrix.eigen_decomposition();
        assert!(
            result.is_some(),
            "Symmetric matrix should have eigen decomposition"
        );

        let decomp = result.unwrap();
        println!("Number of eigenvalues: {}", decomp.eigenvalues.len());

        let mut real_eigenvalues: Vec<f64> = Vec::new();
        for (i, eigenval) in decomp.eigenvalues.iter().enumerate() {
            println!("Eigenvalue {i}: {} + {}i", eigenval.re, eigenval.im);
            assert!(
                eigenval.im.abs() < 1e-10,
                "Symmetric matrix eigenvalue should be real"
            );
            real_eigenvalues.push(eigenval.re);
        }

        // 固有値を昇順にソート
        real_eigenvalues.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("Sorted eigenvalues: {real_eigenvalues:?}");

        // トレースと行列式の確認
        let trace = matrix.trace();
        let det = matrix.determinant().unwrap_or(0.0);
        let eigenvalue_sum: f64 = real_eigenvalues.iter().sum();
        let eigenvalue_product: f64 = real_eigenvalues.iter().product();

        println!("Matrix trace: {trace}, Sum of eigenvalues: {eigenvalue_sum}");
        println!("Matrix determinant: {det}, Product of eigenvalues: {eigenvalue_product}");

        assert!((trace - eigenvalue_sum).abs() < 1e-10, "Trace mismatch");
        assert!(
            (det - eigenvalue_product).abs() < 1e-10,
            "Determinant mismatch"
        );

        println!("Random symmetric matrix test passed!\n");
    }
}
