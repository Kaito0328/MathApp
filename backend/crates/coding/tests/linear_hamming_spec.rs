use coding::{types::Message, GFp, Hamming74, LinearCode};
use linalg::{Matrix, Vector};

#[test]
fn linear_code_encode_over_gf5() {
    type F = GFp<5>;
    // G: 2x3 over GF(5)
    let g = Matrix::new(
        2,
        3,
        vec![
            F::new(1),
            F::new(0),
            F::new(2),
            F::new(0),
            F::new(1),
            F::new(3),
        ],
    )
    .unwrap();
    let lc: LinearCode<F> = LinearCode::new(g);
    let u = Message(Vector::new(vec![F::new(2), F::new(4)]));
    let c = lc.encode(&u);
    // c = u * G = [2,4] * [[1,0,2],[0,1,3]] = [2,4, (2*2+4*3)=16≡1]
    assert_eq!(c.dim(), 3);
    assert_eq!(c[0].value(), 2);
    assert_eq!(c[1].value(), 4);
    assert_eq!(c[2].value(), 1);
}

#[test]
fn hamming74_encode() {
    let ham = Hamming74::default();
    let u = Message(Vector::new(vec![
        GFp::<2>::new(1),
        GFp::<2>::new(0),
        GFp::<2>::new(1),
        GFp::<2>::new(1),
    ]));
    let c = ham.encode(&u);
    assert_eq!(c.dim(), 7);
}
