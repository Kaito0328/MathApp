use linalg::Vector;

#[cfg(test)]
mod vector_tests {
    use num_traits::Float;

    use super::*;

    #[test]
    fn test_vector_creation() {
        let data = vec![1.0, 2.0, 3.0];
        let vector = Vector::new(data);
        assert_eq!(vector.dim(), 3);
        assert_eq!(vector[0], 1.0);
        assert_eq!(vector[1], 2.0);
        assert_eq!(vector[2], 3.0);
    }

    #[test]
    fn test_vector_zeros() {
        let vector = Vector::<f64>::zeros(5);
        assert_eq!(vector.dim(), 5);
        for i in 0..5 {
            assert_eq!(vector[i], 0.0);
        }
    }

    #[test]
    fn test_vector_ones() {
        let vector = Vector::<f64>::ones(4);
        assert_eq!(vector.dim(), 4);
        for i in 0..4 {
            assert_eq!(vector[i], 1.0);
        }
    }

    #[test]
    fn test_vector_norm() {
        let vector = Vector::new(vec![3.0, 4.0]);
        let norm = vector.norm();
        assert!((norm - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
        let dot = v1.dot(&v2);
        // 1*4 + 2*5 + 3*6 = 32
        assert!((dot - 32.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector_addition() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
        let result = v1 + v2;
        assert_eq!(result.dim(), 3);
        assert_eq!(result[0], 5.0);
        assert_eq!(result[1], 7.0);
        assert_eq!(result[2], 9.0);
    }

    #[test]
    fn test_vector_normalize() {
        let vector = Vector::new(vec![3.0, 4.0]);
        let normalized = vector.normalize();
        let norm = normalized.norm();
        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector_cross_product_3d() {
        let v1 = Vector::new(vec![1.0, 0.0, 0.0]);
        let v2 = Vector::new(vec![0.0, 1.0, 0.0]);
        let cross = v1.cross(&v2).unwrap();
        assert_eq!(cross.dim(), 3);
        assert_eq!(cross[2], 1.0);
    }

    #[test]
    fn test_vector_scalar_multiplication() {
        let vector = Vector::new(vec![1.0, 2.0, 3.0]);
        let result = vector * 2.0;
        assert_eq!(result[0], 2.0);
        assert_eq!(result[1], 4.0);
        assert_eq!(result[2], 6.0);
    }

    #[test]
    fn test_vector_subtraction() {
        let v1 = Vector::new(vec![5.0, 7.0, 9.0]);
        let v2 = Vector::new(vec![1.0, 2.0, 3.0]);
        let result = v1 - v2;
        assert_eq!(result[0], 4.0);
        assert_eq!(result[1], 5.0);
        assert_eq!(result[2], 6.0);
    }
}
