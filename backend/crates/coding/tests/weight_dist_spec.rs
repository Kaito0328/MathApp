use coding::{code_utils::*, GFp};
use linalg::Matrix;

#[test]
fn weight_distribution_gf2_small() {
    type F = GFp<2>;
    let g = Matrix::new(
        2,
        3,
        vec![
            F::new(1),
            F::new(0),
            F::new(1),
            F::new(0),
            F::new(1),
            F::new(1),
        ],
    )
    .unwrap();
    let codebook = generate_codebook_gfp::<2>(&g);
    let bins = weight_distribution(&codebook);
    assert_eq!(bins.iter().sum::<usize>(), 4);
}
