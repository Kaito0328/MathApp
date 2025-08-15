use coding::{code_utils::*, GFp};
use linalg::{Matrix, Vector};

#[test]
fn hamming_distance_basic() {
    let a = Vector::new(vec![GFp::<2>::new(1), GFp::<2>::new(0), GFp::<2>::new(1)]);
    let b = Vector::new(vec![GFp::<2>::new(1), GFp::<2>::new(1), GFp::<2>::new(1)]);
    assert_eq!(hamming_distance(&a, &b), 1);
}

#[test]
fn generate_codebook_gf2_small() {
    type F = GFp<2>;
    // G = [I2 | P]
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
    assert_eq!(codebook.len(), 4);
}

#[test]
fn formed_g_to_h_standard_shape() {
    type F = GFp<3>;
    let g = Matrix::new(
        2,
        4,
        vec![
            F::new(1),
            F::new(0),
            F::new(2),
            F::new(1),
            F::new(0),
            F::new(1),
            F::new(1),
            F::new(2),
        ],
    )
    .unwrap();
    let h = formed_g_to_h(&g);
    assert_eq!(h.rows, 2);
    assert_eq!(h.cols, 4);
}
