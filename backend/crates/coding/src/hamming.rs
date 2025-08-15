use crate::prime::GFp;
use linalg::{Matrix, Vector};

// (7,4) Hamming code over GF(2)
#[derive(Debug, Clone)]
pub struct Hamming74 {
    pub g: Matrix<GFp<2>>, // 4x7 generator
}

impl Default for Hamming74 {
    fn default() -> Self {
        // 標準形 G = [I_4 | P]
        let data: Vec<GFp<2>> = vec![
            1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1,
        ]
        .into_iter()
        .map(GFp::<2>)
        .collect();
        let g = Matrix::new(4, 7, data).unwrap();
        Self { g }
    }
}

impl Hamming74 {
    pub fn encode(&self, u: &Vector<GFp<2>>) -> Vector<GFp<2>> {
        (u.clone() * &self.g).row(0).unwrap()
    }
}
