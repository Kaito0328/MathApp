use linalg::{Matrix, Vector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 基本的な行列・ベクトル操作の例 ===");

    // ベクトルの作成と基本操作
    let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::new(vec![4.0, 5.0, 6.0]);

    println!("ベクトル v1: {:?}", v1.data);
    println!("ベクトル v2: {:?}", v2.data);
    println!("v1のノルム: {}", v1.norm());
    println!("v1とv2の内積: {}", v1.dot(&v2));

    // ベクトルの加算
    let v_sum = v1 + v2;
    println!("v1 + v2: {:?}", v_sum.data);

    // 行列の作成
    let matrix_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let matrix = Matrix::new(2, 3, matrix_data)?;
    println!("\n行列 (2x3):");
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            print!("{:8.2} ", matrix[(i, j)]);
        }
        println!();
    }

    // 転置行列
    let transposed = matrix.transpose();
    println!("\n転置行列 (3x2):");
    for i in 0..transposed.rows {
        for j in 0..transposed.cols {
            print!("{:8.2} ", transposed[(i, j)]);
        }
        println!();
    }

    // 単位行列の作成
    let identity: Matrix<f64> = Matrix::identity(3);
    println!("\n単位行列 (3x3):");
    for i in 0..identity.rows {
        for j in 0..identity.cols {
            print!("{:8.2} ", identity[(i, j)]);
        }
        println!();
    }
    Ok(())
}
