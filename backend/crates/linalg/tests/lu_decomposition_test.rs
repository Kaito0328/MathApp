#[cfg(test)]
mod tests {
    use linalg::Matrix;

    #[test]
    fn test_lu_decomposition_basic() {
        let matrix = Matrix::new(3, 3, vec![2.0, 1.0, 1.0, 4.0, 3.0, 3.0, 8.0, 7.0, 9.0]).unwrap();

        let result = matrix.lu_decomposition();
        assert!(result.is_some());

        let (p, l, u) = result.unwrap();

        // 行列の表示
        println!("L行列: {l:?}");
        println!("U行列: {u:?}");
        println!("P行列: {p:?}");

        // PA = LU が成り立つことを確認
        let pa = &p * &matrix;
        let lu = &l * &u;

        for i in 0..3 {
            for j in 0..3 {
                assert!((pa[(i, j)] - lu[(i, j)]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_lu_decomposition_identity() {
        let matrix: Matrix<f64> = Matrix::identity(3);
        let result = matrix.lu_decomposition();
        assert!(result.is_some());

        let (p, l, u) = result.unwrap();
        // 行列の表示
        println!("L行列: {l:?}");
        println!("U行列: {u:?}");
        println!("P行列: {p:?}");

        // 単位行列の場合、P = L = U = I
        let identity: Matrix<f64> = Matrix::identity(3);
        for i in 0..3 {
            for j in 0..3 {
                assert!((p[(i, j)] - identity[(i, j)]).abs() < 1e-10);
                assert!((l[(i, j)] - identity[(i, j)]).abs() < 1e-10);
                assert!((u[(i, j)] - identity[(i, j)]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_lu_decomposition_singular_matrix() {
        // 特異行列（ランクが足りない）
        let matrix = Matrix::new(3, 3, vec![1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 1.0, 2.0, 3.0]).unwrap();

        let result = matrix.lu_decomposition();
        assert!(result.is_none());
        // 特異行列の場合、LU分解はできないため、Noneが返ることを確認
        println!("特異行列のLU分解はできませんでした");
    }

    #[test]
    fn test_lu_decomposition_non_square() {
        let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();

        let result = matrix.lu_decomposition();
        assert!(result.is_none());
        // 非正方行列の場合、LU分解はできないため、Noneが返ることを確認
        println!("非正方行列のLU分解はできませんでした");
    }

    #[test]
    fn test_lu_decomposition_upper_triangular() {
        let matrix = Matrix::new(3, 3, vec![2.0, 1.0, 3.0, 0.0, 4.0, 2.0, 0.0, 0.0, 5.0]).unwrap();

        let result = matrix.lu_decomposition();
        assert!(result.is_some());

        let (p, l, u) = result.unwrap();
        // 行列の表示
        println!("L行列: {l:?}");
        println!("U行列: {u:?}");
        println!("P行列: {p:?}");

        // PA = LU の確認
        let pa = &p * &matrix;
        let lu = &l * &u;

        for i in 0..3 {
            for j in 0..3 {
                assert!((pa[(i, j)] - lu[(i, j)]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_lu_decomposition_lower_triangular() {
        let matrix = Matrix::new(3, 3, vec![2.0, 0.0, 0.0, 1.0, 3.0, 0.0, 4.0, 2.0, 5.0]).unwrap();

        let result = matrix.lu_decomposition();
        assert!(result.is_some());

        let (p, l, u) = result.unwrap();
        // 行列の表示
        println!("L行列: {l}");
        println!("U行列: {u}");
        println!("P行列: {p}");

        // PA = LU の確認
        let pa = &p * &matrix;
        let lu = &l * &u;

        for i in 0..3 {
            for j in 0..3 {
                assert!((pa[(i, j)] - lu[(i, j)]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_lu_decomposition_1x1() {
        let matrix = Matrix::new(1, 1, vec![5.0]).unwrap();
        let result = matrix.lu_decomposition();
        assert!(result.is_some());

        let (p, l, u) = result.unwrap();
        // 行列の表示
        println!("L行列: {l}");
        println!("U行列: {u}");
        println!("P行列: {p}");

        assert_eq!(p[(0, 0)], 1.0);
        assert_eq!(l[(0, 0)], 1.0);
        assert_eq!(u[(0, 0)], 5.0);
    }

    #[test]
    fn test_lu_decomposition_zero_diagonal() {
        // 対角要素にゼロがある行列（ピボッティングで解決可能）
        let matrix = Matrix::new(2, 2, vec![0.0, 1.0, 1.0, 0.0]).unwrap();

        let result = matrix.lu_decomposition();
        assert!(result.is_some());

        let (p, l, u) = result.unwrap();
        // 行列の表示
        println!("L行列: {l:?}");
        println!("U行列: {u:?}");
        println!("P行列: {p:?}");

        // PA = LU の確認
        let pa = &p * &matrix;
        let lu = &l * &u;

        for i in 0..2 {
            for j in 0..2 {
                assert!((pa[(i, j)] - lu[(i, j)]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_lu_decomposition_large_matrix() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 5.0, 6.0, 7.0, 3.0, 6.0, 10.0, 11.0, 4.0, 7.0, 11.0, 15.0,
            ],
        )
        .unwrap();

        let result = matrix.lu_decomposition();
        assert!(result.is_some());

        let (p, l, u) = result.unwrap();
        // 行列の表示
        println!("L行列: {l:?}");
        println!("U行列: {u:?}");
        println!("P行列: {p:?}");

        // PA = LU の確認
        let pa = &p * &matrix;
        let lu = &l * &u;

        for i in 0..4 {
            for j in 0..4 {
                assert!((pa[(i, j)] - lu[(i, j)]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_lu_decomposition_negative_values() {
        let matrix =
            Matrix::new(3, 3, vec![-2.0, 1.0, 3.0, 4.0, -5.0, 6.0, -7.0, 8.0, -9.0]).unwrap();

        let result = matrix.lu_decomposition();
        assert!(result.is_some());

        let (p, l, u) = result.unwrap();

        // 行列の表示
        println!("L行列: {l:?}");
        println!("U行列: {u:?}");
        println!("P行列: {p:?}");

        // PA = LU の確認
        let pa = &p * &matrix;
        let lu = &l * &u;

        for i in 0..3 {
            for j in 0..3 {
                assert!((pa[(i, j)] - lu[(i, j)]).abs() < 1e-10);
            }
        }
    }
}
