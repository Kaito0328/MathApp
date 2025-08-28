use crate::matrix::Matrix;

/// Compute matrix exponential expm(A) for f64 matrices using scaling and squaring with
/// truncated Taylor (Padé not yet). Good enough for small sizes used in c2d.
pub trait MatrixExponential {
    fn expm(&self) -> Matrix<f64>;
}

impl MatrixExponential for Matrix<f64> {
    fn expm(&self) -> Matrix<f64> {
        // Scaling and squaring: find s so that ||A/2^s|| is small, then expm(A) = (expm(A/2^s))^(2^s)
        // Use 1-norm estimate by sum of absolute values per row.
        let n = self.rows;
        assert!(self.rows == self.cols, "expm requires square matrix");
        let mut a1 = 0.0f64;
        for i in 0..n {
            let mut row_sum = 0.0;
            for j in 0..n {
                row_sum += self[(i, j)].abs();
            }
            a1 = a1.max(row_sum);
        }
        // choose s so that ||A/2^s|| < 0.5 roughly
        let mut s: u32 = 0;
        let mut scale = 1.0f64;
        if a1 > 0.5 {
            s = (a1 / 0.5).log2().ceil().max(0.0) as u32;
            scale = 2f64.powi(s as i32);
        }
        let a_scaled = self * (1.0f64 / scale);

        // expm via truncated Taylor series up to m terms
        // exp(A) = I + A + A^2/2! + ... + A^m/m!
        let m = 12usize; // small default; OK for ZOH sizes up to ~10
        let mut term = Matrix::identity(n); // current term
        let mut result = Matrix::identity(n);
        for k in 1..=m {
            term = &term * &a_scaled;
            let coeff = 1.0 / (k as f64).factorial();
            result = &result + &(&term * coeff);
        }

        // squaring: result = result^(2^s)
        for _ in 0..s {
            result = &result * &result;
        }
        result
    }
}

trait Factorial {
    fn factorial(self) -> f64;
}

impl Factorial for f64 {
    fn factorial(self) -> f64 {
        // only defined for integer-like small values; used with k as usize casted
        let k = self as usize;
        let mut acc = 1.0;
        for i in 2..=k {
            acc *= i as f64;
        }
        acc
    }
}
