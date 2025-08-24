use crate::error::Result as CodingResult;
use crate::types::{Codeword, GeneratorMatrix, Message};
use finite_field::gfp::GFp;
use linalg::Vector;

// (7,4) Hamming code over GF(2)
#[derive(Debug, Clone)]
pub struct Hamming74 {
    pub g: GeneratorMatrix<GFp<2>>, // 4x7 generator
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
        let g = match linalg::Matrix::new(4, 7, data) {
            Ok(m) => m,
            Err(e) => panic!("valid Hamming(7,4) generator: {e}"),
        };
        Self {
            g: GeneratorMatrix(g),
        }
    }
}

impl Hamming74 {
    pub fn encode(&self, u: &Message<GFp<2>>) -> CodingResult<Codeword<GFp<2>>> {
        let v: Vector<GFp<2>> = u.as_ref().clone();
        let g = &self.g.0;
        let row = (v * g).row(0)?;
        Ok(Codeword::from(row))
    }
}
