use num_complex::Complex;
use poly::polynomial::Polynomial;

/// 連続系 ZPK: G(s) = gain * prod(s - z_i) / prod(s - p_i)
#[derive(Clone, Debug, PartialEq)]
pub struct ContinuousZpk {
    pub zeros: Vec<Complex<f64>>,
    pub poles: Vec<Complex<f64>>,
    pub gain: f64,
}

/// 離散系 ZPK: H(z) = gain * prod(z - z_i) / prod(z - p_i)
#[derive(Clone, Debug, PartialEq)]
pub struct DiscreteZpk {
    pub zeros: Vec<Complex<f64>>,
    pub poles: Vec<Complex<f64>>,
    pub gain: f64,
}

impl ContinuousZpk {
    pub fn new(zeros: Vec<Complex<f64>>, poles: Vec<Complex<f64>>, gain: f64) -> Self {
        Self { zeros, poles, gain }
    }
    pub fn eval_s(&self, s: Complex<f64>) -> Complex<f64> {
        let mut num = Complex::new(self.gain, 0.0);
        for z in &self.zeros {
            num *= s - *z;
        }
        let mut den = Complex::new(1.0, 0.0);
        for p in &self.poles {
            den *= s - *p;
        }
        num / den
    }
    pub fn to_transfer_function(&self) -> crate::continuous::TransferFunction {
        let num_poly = poly_from_roots_real(&self.zeros);
        let num = &num_poly * self.gain;
        let den = poly_from_roots_real(&self.poles);
        crate::continuous::TransferFunction::new(num, den)
    }
    pub fn from_transfer_function(tf: &crate::continuous::TransferFunction) -> Self {
        let zeros = tf.zeros();
        let poles = tf.poles();
        let num_lead = tf.b_coeffs().last().copied().unwrap_or(0.0);
        let den_lead = tf.a_coeffs().last().copied().unwrap_or(1.0);
        let gain = if den_lead != 0.0 {
            num_lead / den_lead
        } else {
            0.0
        };
        Self { zeros, poles, gain }
    }
}

impl DiscreteZpk {
    pub fn new(zeros: Vec<Complex<f64>>, poles: Vec<Complex<f64>>, gain: f64) -> Self {
        Self { zeros, poles, gain }
    }
    pub fn eval_z(&self, z: Complex<f64>) -> Complex<f64> {
        let mut num = Complex::new(self.gain, 0.0);
        for r in &self.zeros {
            num *= z - *r;
        }
        let mut den = Complex::new(1.0, 0.0);
        for p in &self.poles {
            den *= z - *p;
        }
        num / den
    }
    pub fn to_transfer_function(&self, sample_rate: f64) -> crate::discrete::TransferFunction {
        let num_poly = poly_from_roots_real(&self.zeros);
        let num = &num_poly * self.gain;
        let den = poly_from_roots_real(&self.poles);
        crate::discrete::TransferFunction::new_with_fs(num, den, sample_rate)
    }
    pub fn from_transfer_function(tf: &crate::discrete::TransferFunction) -> Self {
        let zeros = tf.zeros();
        let poles = tf.poles();
        let num_lead = tf.b_coeffs().last().copied().unwrap_or(0.0);
        let den_lead = tf.a_coeffs().last().copied().unwrap_or(1.0);
        let gain = if den_lead != 0.0 {
            num_lead / den_lead
        } else {
            0.0
        };
        Self { zeros, poles, gain }
    }
}

/// 実係数多項式を、複素根の集合から構成する（共役ペアを探して2次実多項式化）
fn poly_from_roots_real(roots: &[Complex<f64>]) -> Polynomial<f64> {
    let mut rem: Vec<Complex<f64>> = roots.to_vec();
    let mut poly = Polynomial::new(vec![1.0]); // 1
    let tol = 1e-10;
    while let Some(r) = rem.pop() {
        if r.im.abs() < tol {
            // (x - r)
            poly = poly * Polynomial::new(vec![-r.re, 1.0]);
        } else {
            // 探す conj
            let mut found = None;
            for (i, c) in rem.iter().enumerate() {
                if (c.re - r.re).abs() < 1e-9 && (c.im + r.im).abs() < 1e-9 {
                    found = Some(i);
                    break;
                }
            }
            if let Some(i) = found {
                // remove the conjugate root; value is not needed further
                rem.remove(i);
                // (x - r)(x - c) = x^2 - 2 Re(r) x + |r|^2
                let a1 = -2.0 * r.re;
                let a0 = r.norm_sqr();
                poly = poly * Polynomial::new(vec![a0, a1, 1.0]);
            } else {
                // 単独の非実根: 近似的に無視できないので線形実近似は不適切。安全のためpanic。
                panic!("Unpaired complex root found when building real-coefficient polynomial");
            }
        }
    }
    poly
}
