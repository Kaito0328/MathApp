use linalg::{Matrix, Vector};

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let matrix = Matrix::new(2, 2, data);
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 2);
        assert_eq!(matrix.data.len(), 4);
    }

    #[test]
    fn test_matrix_zeros() {
        let matrix = Matrix::zeros(3, 3);
        assert_eq!(matrix.rows, 3);
        assert_eq!(matrix.cols, 3);
        for &val in &matrix.data {
            assert_eq!(val, 0.0);
        }
    }

    #[test]
    fn test_matrix_identity() {
        let matrix = Matrix::identity(3);
        assert_eq!(matrix.rows, 3);
        assert_eq!(matrix.cols, 3);
        // 対角成分は1、それ以外は0であることをテスト
        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    assert_eq!(matrix[(i, j)], 1.0);
                } else {
                    assert_eq!(matrix[(i, j)], 0.0);
                }
            }
        }
    }

    #[test]
    fn test_matrix_transpose() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let matrix = Matrix::new(2, 3, data);
        let transposed = matrix.transpose();
        assert_eq!(transposed.rows, 3);
        assert_eq!(transposed.cols, 2);
    }

    #[test]
    fn test_matrix_addition() {
        let data1 = vec![1.0, 2.0, 3.0, 4.0];
        let data2 = vec![5.0, 6.0, 7.0, 8.0];
        let matrix1 = Matrix::new(2, 2, data1);
        let matrix2 = Matrix::new(2, 2, data2);
        let result = matrix1 + matrix2;
        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 2);
    }

    #[test]
    fn test_matrix_multiplication() {
        let data1 = vec![1.0, 2.0, 3.0, 4.0];
        let data2 = vec![5.0, 6.0, 7.0, 8.0];
        let matrix1 = Matrix::new(2, 2, data1);
        let matrix2 = Matrix::new(2, 2, data2);
        let result = matrix1 * matrix2;
        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 2);
    }

    #[test]
    fn test_matrix_determinant() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let matrix = Matrix::new(2, 2, data);
        let det = matrix.determinant().unwrap();
        // 期待される値は -2.0
        assert!((det - (-2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_matrix_is_square() {
        let square_matrix = Matrix::new(3, 3, vec![0.0; 9]);
        let rect_matrix = Matrix::new(2, 3, vec![0.0; 6]);
        assert!(square_matrix.is_square());
        assert!(!rect_matrix.is_square());
    }

    #[test]
    fn test_matrix_indexing() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let matrix = Matrix::new(2, 2, data);
        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(0, 1)], 2.0);
        assert_eq!(matrix[(1, 0)], 3.0);
        assert_eq!(matrix[(1, 1)], 4.0);
    }
}
