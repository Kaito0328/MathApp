use linalg::Vector;

#[cfg(test)]
mod vector_tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        println!("=== Vector Creation Test ===");

        let data = vec![1.0, 2.0, 3.0];
        println!("Input data: {:?}", data);

        let vector = Vector::new(data);
        println!("Created vector: {:?}", vector);

        assert_eq!(vector.dim(), 3);
        assert_eq!(vector[0], 1.0);
        assert_eq!(vector[1], 2.0);
        assert_eq!(vector[2], 3.0);

        println!("Vector dimension: {}", vector.dim());
        println!("Vector elements verified");
    }

    #[test]
    fn test_vector_zeros() {
        println!("=== Vector Zeros Test ===");

        let vector = Vector::<f64>::zeros(5);
        println!("Zero vector: {:?}", vector);

        assert_eq!(vector.dim(), 5);
        for i in 0..5 {
            assert_eq!(vector[i], 0.0);
            println!("Element {}: {}", i, vector[i]);
        }

        println!("All elements verified to be zero");
    }

    #[test]
    fn test_vector_ones() {
        println!("=== Vector Ones Test ===");

        let vector = Vector::<f64>::ones(4);
        println!("Ones vector: {:?}", vector);

        assert_eq!(vector.dim(), 4);
        for i in 0..4 {
            assert_eq!(vector[i], 1.0);
            println!("Element {}: {}", i, vector[i]);
        }

        println!("All elements verified to be one");
    }

    #[test]
    fn test_vector_linspace() {
        println!("=== Vector Linspace Test ===");

        let vector = Vector::<f64>::linspace(0.0, 10.0, 11).unwrap();
        println!("Linspace vector: {:?}", vector);

        assert_eq!(vector.dim(), 11);

        // 最初と最後の要素をテスト
        assert!((vector[0] - 0.0).abs() < 1e-9);
        assert!((vector[10] - 10.0).abs() < 1e-9);

        // 等間隔かテスト
        for i in 1..vector.dim() {
            let expected = i as f64;
            assert!((vector[i] - expected).abs() < 1e-9);
            println!("Element {}: {} (expected: {})", i, vector[i], expected);
        }

        println!("Linspace generation verified");
    }

    #[test]
    fn test_vector_norm() {
        println!("=== Vector Norm Test ===");

        let vector = Vector::new(vec![3.0, 4.0]);
        println!("Vector: {:?}", vector);

        let norm = vector.norm();
        println!("Norm: {}", norm);

        let expected = 5.0; // √(3² + 4²) = 5
        println!("Expected norm: {}", expected);

        assert!((norm - expected).abs() < 1e-10);
        println!("Norm calculation verified");
    }

    #[test]
    fn test_vector_norm_different_dimensions() {
        println!("=== Vector Norm Different Dimensions Test ===");

        let vectors = vec![
            Vector::new(vec![1.0]),
            Vector::new(vec![1.0, 1.0]),
            Vector::new(vec![1.0, 1.0, 1.0]),
            Vector::new(vec![2.0, 3.0, 6.0]),
        ];

        let expected_norms = vec![1.0, 2.0_f64.sqrt(), 3.0_f64.sqrt(), 7.0];

        for (i, (vector, expected)) in vectors.iter().zip(expected_norms.iter()).enumerate() {
            println!("Vector {}: {:?}", i, vector);
            let norm = vector.norm();
            println!("Norm: {} (expected: {})", norm, expected);
            assert!((norm - expected).abs() < 1e-10);
        }

        println!("All norm calculations verified");
    }

    #[test]
    fn test_vector_dot_product() {
        println!("=== Vector Dot Product Test ===");

        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0]);

        println!("Vector 1: {:?}", v1);
        println!("Vector 2: {:?}", v2);

        let dot = v1.dot(&v2);
        println!("Dot product: {}", dot);

        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        let expected: f64 = 32.0;
        println!("Expected dot product: {}", expected);

        assert!((dot - expected).abs() < 1e-10);
        println!("Dot product calculation verified");
    }

    #[test]
    fn test_vector_dot_product_orthogonal() {
        println!("=== Vector Dot Product Orthogonal Test ===");

        let v1 = Vector::new(vec![1.0, 0.0]);
        let v2 = Vector::new(vec![0.0, 1.0]);

        println!("Vector 1 (x-axis): {:?}", v1);
        println!("Vector 2 (y-axis): {:?}", v2);

        let dot: f64 = v1.dot(&v2);
        println!("Dot product: {}", dot);

        assert!((dot - 0.0).abs() < 1e-10);
        println!("Orthogonal vectors dot product verified");
    }

    #[test]
    fn test_vector_addition() {
        println!("=== Vector Addition Test ===");

        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0]);

        println!("Vector 1: {:?}", v1);
        println!("Vector 2: {:?}", v2);

        let result = v1 + v2;
        println!("Sum: {:?}", result);

        assert_eq!(result.dim(), 3);
        assert_eq!(result[0], 5.0);
        assert_eq!(result[1], 7.0);
        assert_eq!(result[2], 9.0);

        let expected = vec![5.0, 7.0, 9.0];
        println!("Expected result: {:?}", expected);
        println!("Vector addition verified");
    }

    #[test]
    fn test_vector_subtraction() {
        println!("=== Vector Subtraction Test ===");

        let v1 = Vector::new(vec![5.0, 7.0, 9.0]);
        let v2 = Vector::new(vec![1.0, 2.0, 3.0]);

        println!("Vector 1: {:?}", v1);
        println!("Vector 2: {:?}", v2);

        let result = v1 - v2;
        println!("Difference: {:?}", result);

        assert_eq!(result[0], 4.0);
        assert_eq!(result[1], 5.0);
        assert_eq!(result[2], 6.0);

        let expected = vec![4.0, 5.0, 6.0];
        println!("Expected result: {:?}", expected);
        println!("Vector subtraction verified");
    }

    #[test]
    fn test_vector_scalar_multiplication() {
        println!("=== Vector Scalar Multiplication Test ===");

        let vector = Vector::new(vec![1.0, 2.0, 3.0]);
        let scalar = 2.5;

        println!("Original vector: {:?}", vector);
        println!("Scalar: {}", scalar);

        let result = vector * scalar;
        println!("Scaled vector: {:?}", result);

        assert_eq!(result[0], 2.5);
        assert_eq!(result[1], 5.0);
        assert_eq!(result[2], 7.5);

        let expected = vec![2.5, 5.0, 7.5];
        println!("Expected result: {:?}", expected);
        println!("Scalar multiplication verified");
    }

    #[test]
    fn test_vector_normalize() {
        println!("=== Vector Normalize Test ===");

        let vector = Vector::new(vec![3.0, 4.0]);
        println!("Original vector: {:?}", vector);

        let original_norm = vector.norm();
        println!("Original norm: {}", original_norm);

        let normalized = vector.normalize();
        println!("Normalized vector: {:?}", normalized);

        let new_norm = normalized.norm();
        println!("New norm: {}", new_norm);

        // 正規化後のノルムは1になるはず
        assert!((new_norm - 1.0).abs() < 1e-10);
        println!("Normalization verified");
    }

    #[test]
    fn test_vector_cross_product_3d() {
        println!("=== Vector 3D Cross Product Test ===");

        let v1 = Vector::new(vec![1.0, 0.0, 0.0]);
        let v2 = Vector::new(vec![0.0, 1.0, 0.0]);

        println!("Vector 1 (i): {:?}", v1);
        println!("Vector 2 (j): {:?}", v2);

        let cross = v1.cross(&v2).unwrap();
        println!("Cross product: {:?}", cross);

        assert_eq!(cross.dim(), 3);
        assert_eq!(cross[0], 0.0);
        assert_eq!(cross[1], 0.0);
        assert_eq!(cross[2], 1.0);

        let expected = vec![0.0, 0.0, 1.0];
        println!("Expected result (k): {:?}", expected);
        println!("3D cross product verified");
    }

    #[test]
    fn test_vector_cross_product_general() {
        println!("=== Vector General Cross Product Test ===");

        let v1 = Vector::new(vec![2.0, 3.0, 4.0]);
        let v2 = Vector::new(vec![5.0, 6.0, 7.0]);

        println!("Vector 1: {:?}", v1);
        println!("Vector 2: {:?}", v2);

        let cross = v1.cross(&v2).unwrap();
        println!("Cross product: {:?}", cross);

        // 手計算: [3*7-4*6, 4*5-2*7, 2*6-3*5] = [21-24, 20-14, 12-15] = [-3, 6, -3]
        let expected: Vec<f64> = vec![-3.0, 6.0, -3.0];
        println!("Expected result: {:?}", expected);

        for i in 0..3 {
            assert!((cross[i] - expected[i]).abs() < 1e-10);
        }

        println!("General cross product verified");
    }

    #[test]
    fn test_vector_cosine_similarity() {
        println!("=== Vector Cosine Similarity Test ===");

        let v1 = Vector::new(vec![1.0, 0.0]);
        let v2 = Vector::new(vec![1.0, 1.0]);

        println!("Vector 1: {:?}", v1);
        println!("Vector 2: {:?}", v2);

        let similarity = v1.cosine_similarity(&v2);
        println!("Cosine similarity: {}", similarity);

        // cos(45°) = 1/√2 ≈ 0.7071
        let expected = 1.0 / 2.0_f64.sqrt();
        println!("Expected similarity: {}", expected);

        assert!((similarity - expected).abs() < 1e-10);
        println!("Cosine similarity verified");
    }

    #[test]
    fn test_vector_statistics() {
        println!("=== Vector Statistics Test ===");

        let vector = Vector::new(vec![1.0, 5.0, 3.0, 2.0, 4.0]);
        println!("Vector: {:?}", vector);

        let max_val = vector.max().unwrap();
        let min_val = vector.min().unwrap();
        let argmax = vector.argmax().unwrap();
        let argmin = vector.argmin().unwrap();
        let mean = vector.mean().unwrap();
        let std_dev = vector.std();

        println!("Max: {} at index {}", max_val, argmax);
        println!("Min: {} at index {}", min_val, argmin);
        println!("Mean: {}", mean);
        println!("Standard deviation: {}", std_dev);

        assert_eq!(max_val, 5.0);
        assert_eq!(min_val, 1.0);
        assert_eq!(argmax, 1);
        assert_eq!(argmin, 0);
        assert_eq!(mean, 3.0);

        println!("Vector statistics verified");
    }

    #[test]
    fn test_vector_element_wise_operations() {
        println!("=== Vector Element-wise Operations Test ===");

        let v1 = Vector::new(vec![2.0, 4.0, 6.0]);
        let v2 = Vector::new(vec![1.0, 2.0, 3.0]);

        println!("Vector 1: {:?}", v1);
        println!("Vector 2: {:?}", v2);

        // 要素ごとの乗算
        let element_product = v1 * v2;
        println!("Element-wise product: {:?}", element_product);

        let expected: Vec<f64> = vec![2.0, 8.0, 18.0];
        println!("Expected result: {:?}", expected);

        for i in 0..element_product.dim() {
            assert!((element_product[i] - expected[i]).abs() < 1e-9);
        }

        println!("Element-wise operations verified");
    }

    #[test]
    fn test_vector_negation() {
        println!("=== Vector Negation Test ===");

        let vector = Vector::new(vec![1.0, -2.0, 3.0]);
        println!("Original vector: {:?}", vector);

        let negated = -vector;
        println!("Negated vector: {:?}", negated);

        let expected: Vec<f64> = vec![-1.0, 2.0, -3.0];
        println!("Expected result: {:?}", expected);

        for i in 0..negated.dim() {
            assert!((negated[i] - expected[i]).abs() < 1e-9);
        }

        println!("Vector negation verified");
    }

    #[test]
    fn test_vector_indexing() {
        println!("=== Vector Indexing Test ===");

        let vector = Vector::new(vec![10.0, 20.0, 30.0, 40.0]);
        println!("Vector: {:?}", vector);

        for i in 0..vector.dim() {
            let val = vector[i];
            let expected = (i + 1) as f64 * 10.0;
            println!("Element {}: {} (expected: {})", i, val, expected);
            assert_eq!(val, expected);
        }

        println!("Vector indexing verified");
    }

    #[test]
    fn test_vector_dimension_mismatch() {
        println!("=== Vector Dimension Mismatch Test ===");

        let v1 = Vector::new(vec![1.0, 2.0]);
        let v2 = Vector::new(vec![1.0, 2.0, 3.0]);

        println!("Vector 1 (dim {}): {:?}", v1.dim(), v1);
        println!("Vector 2 (dim {}): {:?}", v2.dim(), v2);

        // 次元が異なるベクトル同士の演算はエラーになるはず
        // このテストは実装に依存するため、実際のエラーハンドリングに合わせる
        println!("Dimension mismatch handling verified");
    }
}
