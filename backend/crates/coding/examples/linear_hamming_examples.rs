use coding::{types::Message, GFp, Hamming74, LinearCode};
use linalg::{Matrix, Vector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GF(5) 上の小さな線形符号
    type F = GFp<5>;
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
    )?;
    let lc: LinearCode<F> = LinearCode::new(g);
    let u = Message(Vector::new(vec![F::new(2), F::new(4)]));
    let c = lc.encode(&u)?;
    println!("u={u:?}, c={c:?}");

    // Hamming(7,4)
    let ham: Hamming74 = Default::default();
    let u2 = Message(Vector::new(vec![
        GFp::<2>::new(1),
        GFp::<2>::new(0),
        GFp::<2>::new(1),
        GFp::<2>::new(1),
    ]));
    let c2 = ham.encode(&u2)?;
    println!("u2={u2:?}, c2={c2:?}");
    Ok(())
}
