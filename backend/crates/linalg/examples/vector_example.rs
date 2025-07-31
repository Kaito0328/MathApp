use linalg::Vector;

fn main() {
    println!("=== ベクトル操作の例 ===");

    // 様々な方法でベクトルを作成
    let v1 = Vector::new(vec![1.0, -2.0, 3.0]);
    let v2 = Vector::zeros(3);
    let v3 = Vector::ones(3);
    let v4 = Vector::linspace(0.0, 10.0, 11);

    println!("v1: {:?}", v1.data);
    println!("v2 (zeros): {:?}", v2.data);
    println!("v3 (ones): {:?}", v3.data);
    println!("v4 (linspace): {:?}", v4.data);

    // ベクトルの統計量
    println!("\nv1の統計量:");
    println!("  最大値: {} (インデックス: {})", v1.max(), v1.argmax());
    println!("  最小値: {} (インデックス: {})", v1.min(), v1.argmin());
    println!("  平均値: {}", v1.mean());
    println!("  標準偏差: {}", v1.std());
    println!("  ノルム: {}", v1.norm());

    // ベクトルの正規化
    let v1_normalized = v1.normalize();
    println!("\nv1の正規化: {:?}", v1_normalized.data);
    println!("正規化後のノルム: {}", v1_normalized.norm());

    // 3次元ベクトルの外積
    let a = Vector::new(vec![1.0, 0.0, 0.0]);
    let b = Vector::new(vec![0.0, 1.0, 0.0]);
    match a.cross(&b) {
        Ok(cross_product) => {
            println!("\na × b = {:?}", cross_product.data);
        }
        Err(e) => println!("外積の計算でエラー: {}", e),
    }

    // ベクトルのコサイン類似度
    let similarity = v1.cosine_similarity(&v3);
    println!("v1とv3のコサイン類似度: {}", similarity);
}
