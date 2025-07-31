use linalg::{Matrix, Vector};

fn main() {
    println!("=== 線形代数の高度な操作例 ===");

    // 正方行列の作成
    let square_matrix = Matrix::new(3, 3, vec![2.0, -1.0, 0.0, -1.0, 2.0, -1.0, 0.0, -1.0, 2.0]);

    println!("正方行列 A:");
    print_matrix(&square_matrix);

    // 行列式の計算
    match square_matrix.determinant() {
        Ok(det) => println!("行列式: {}", det),
        Err(e) => println!("行列式の計算でエラー: {}", e),
    }

    // トレースの計算
    let trace = square_matrix.trace();
    println!("トレース: {}", trace);

    // 逆行列の計算
    match square_matrix.inverse() {
        Some(inv) => {
            println!("\n逆行列:");
            print_matrix(&inv);
        }
        None => println!("逆行列は存在しません"),
    }

    // LU分解
    match square_matrix.lu_decomposition() {
        Some((l, u)) => {
            println!("\nLU分解 - L行列:");
            print_matrix(&l);
            println!("LU分解 - U行列:");
            print_matrix(&u);
        }
        None => println!("LU分解できませんでした"),
    }

    // 固有値・固有ベクトルの計算
    match square_matrix.eigen_decomposition() {
        Some(eigen) => {
            println!("\n固有値: {:?}", eigen.eigenvalues);
            println!("固有ベクトル数: {}", eigen.eigenvectors.len());
        }
        None => println!("固有値分解できませんでした"),
    }
}

fn print_matrix(matrix: &Matrix) {
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            print!("{:8.3} ", matrix[(i, j)]);
        }
        println!();
    }
}
