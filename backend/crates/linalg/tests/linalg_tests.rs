use linalg::{Matrix, Vector};

#[cfg(test)]
mod ops_tests {
    use super::*;

    #[test]
    fn test_matrix_vector_multiplication() {
        let matrix_data = vec![1.0, 2.0, 3.0, 4.0];
        let matrix = Matrix::new(2, 2, matrix_data);
        let vector = Vector::new(vec![1.0, 2.0]);
        let result = matrix * vector;
        assert_eq!(result.dim(), 2);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix_data = vec![1.0, 2.0, 3.0, 4.0];
        let matrix = Matrix::new(2, 2, matrix_data);
        let result = matrix * 2.0;
        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 2);
    }

    #[test]
    fn test_matrix_hstack() {
        let m1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let m2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let result = m1.hstack(&m2).unwrap();
        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 4);
    }

    #[test]
    fn test_matrix_vstack() {
        let m1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let m2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let result = m1.vstack(&m2).unwrap();
        assert_eq!(result.rows, 4);
        assert_eq!(result.cols, 2);
    }

    #[test]
    fn test_matrix_subtraction() {
        let data1 = vec![5.0, 6.0, 7.0, 8.0];
        let data2 = vec![1.0, 2.0, 3.0, 4.0];
        let matrix1 = Matrix::new(2, 2, data1);
        let matrix2 = Matrix::new(2, 2, data2);
        let result = matrix1 - matrix2;
        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 2);
    }

    #[test]
    fn test_generic_types() {
        // ジェネリック型でのテスト（f32）
        let matrix_f32 = Matrix::<f32>::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let vector_f32 = Vector::<f32>::new(vec![1.0, 2.0]);

        assert_eq!(matrix_f32.rows, 2);
        assert_eq!(vector_f32.dim(), 2);
    }
}
