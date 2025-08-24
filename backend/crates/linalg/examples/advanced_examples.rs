use linalg::matrix::numerical::{EigenDecomposition, LuDecomposition};
use linalg::Matrix;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 線形代数の高度な操作例 ===");

    // 正方行列の作成
    let square_matrix =
        Matrix::new(3, 3, vec![2.0, -1.0, 0.0, -1.0, 2.0, -1.0, 0.0, -1.0, 2.0])?;

    println!("正方行列 A:");
    print_matrix(&square_matrix);

    // 行列式の計算（f64専用メソッド）
    match square_matrix.determinant() {
        Ok(det) => println!("行列式: {det}"),
        Err(e) => println!("行列式の計算でエラー: {e}"),
    }

    // トレースの計算
    match square_matrix.trace() {
        Ok(trace) => println!("トレース: {trace}"),
        Err(e) => println!("トレース計算でエラー: {e}"),
    }

    // 逆行列の計算（f64専用メソッド）
    match square_matrix.inverse() {
        Some(inv) => {
            println!("\n逆行列:");
            print_matrix(&inv);
        }
        None => println!("逆行列は存在しません"),
    }

    // LU分解（f64専用メソッド）
    match square_matrix.lu_decomposition() {
        Ok(lu) => {
            println!("\nLU分解 - L行列:");
            print_matrix(&lu.l);
            println!("LU分解 - U行列:");
            print_matrix(&lu.u);
            println!("LU分解 - P行列:");
            print_matrix(&lu.p);
        }
        Err(e) => println!("LU分解できませんでした: {e}"),
    }

    // 固有値・固有ベクトルの計算（f64専用メソッド）
    match square_matrix.eigen_decomposition() {
        Ok(eigen) => {
            println!("\n固有値: {:?}", eigen.eigen_values);
            println!("固有ベクトル数: {}", eigen.eigen_vectors.cols);
        }
        Err(e) => println!("固有値分解できませんでした: {e}"),
    }

    // 水平結合の例
    let m1 = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0])?;
    let m2 = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0])?;
    match m1.hstack(&m2) {
        Ok(hstacked) => {
            println!("\n水平結合 (hstack):");
            print_matrix(&hstacked);
        }
        Err(e) => println!("水平結合でエラー: {e}"),
    }
    Ok(())
}

fn print_matrix(matrix: &Matrix<f64>) {
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            print!("{:8.3} ", matrix[(i, j)]);
        }
        println!();
    }
}
