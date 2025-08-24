use num_complex::Complex;
use std::fmt;

use crate::polynomial::Polynomial;
use crate::rational_function::RationalFunction;
use linalg::Field;

/// Format a real number with trimming trailing zeros and a sensible precision.
pub fn fmt_real(x: f64) -> String {
    if x.is_nan() {
        return "NaN".to_string();
    }
    if x.is_infinite() {
        return if x.is_sign_positive() { "+inf" } else { "-inf" }.to_string();
    }
    // Snap tiny values to 0
    let v = if x.abs() < 1e-12 { 0.0 } else { x };
    // Use up to 12 sig figs, then trim
    let s = format!("{v:.12}");
    trim_trailing_zeros(&s)
}

/// Format a complex number as a+bi with compact zero handling.
pub fn fmt_complex(z: Complex<f64>) -> String {
    let re = z.re;
    let im = z.im;
    let re_s = fmt_real(re);
    let im_s = fmt_real(im.abs());

    if im.abs() < 1e-12 {
        return re_s;
    }
    if re.abs() < 1e-12 {
        return if im.is_sign_negative() {
            format!("-{im_s}i")
        } else {
            format!("{im_s}i")
        };
    }
    if im.is_sign_negative() {
        format!("{re_s}-{im_s}i")
    } else {
        format!("{re_s}+{im_s}i")
    }
}

fn trim_trailing_zeros(s: &str) -> String {
    if !s.contains('.') {
        return s.to_string();
    }
    let mut t = s.trim_end_matches('0').to_string();
    if t.ends_with('.') {
        t.pop();
    }
    if t.is_empty() {
        "0".to_string()
    } else {
        t
    }
}

// -------- Pretty printers with customizable variable name --------

fn to_superscript(n: isize) -> String {
    // Map digits to Unicode superscripts; fall back to ASCII if missing.
    let digits = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
    let minus = '⁻';
    let mut x = n;
    if x == 0 {
        return digits[0].to_string();
    }
    let mut out = String::new();
    if x < 0 {
        out.push(minus);
        x = -x;
    }
    let mut buf = Vec::new();
    while x > 0 {
        buf.push((x % 10) as usize);
        x /= 10;
    }
    for d in buf.iter().rev() {
        out.push(digits[*d]);
    }
    out
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PolyStyle {
    pub unicode_superscript: bool,
    /// 係数と変数の間に '*' を入れるか（既定: false = 入れない, 例: 2x^2）
    pub use_asterisk: bool,
}

pub struct PolyDisplay<'a, F: Field> {
    pub poly: &'a Polynomial<F>,
    pub var: &'a str,
    pub style: PolyStyle,
}

impl<'a, F> PolyDisplay<'a, F>
where
    F: Field + fmt::Display + Clone + PartialEq,
{
    pub fn new(poly: &'a Polynomial<F>, var: &'a str) -> Self {
        Self {
            poly,
            var,
            style: PolyStyle::default(),
        }
    }
    pub fn unicode_superscript(mut self, on: bool) -> Self {
        self.style.unicode_superscript = on;
        self
    }
    pub fn mul_asterisk(mut self, on: bool) -> Self {
        self.style.use_asterisk = on;
        self
    }
}

impl<'a, F> fmt::Display for PolyDisplay<'a, F>
where
    F: Field + fmt::Display + Clone + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.poly.is_zero() {
            return write!(f, "0");
        }
        let var = self.var;
        let mut first = true;
        for (k, c) in self.poly.coeffs.iter().enumerate().rev() {
            if c.is_zero() {
                continue;
            }
            let deg = k as isize;
            let s = c.to_string();
            let is_neg = s.starts_with('-');
            let abs_s: &str = if is_neg { &s[1..] } else { &s };
            let is_one = *c == F::one();
            let is_neg_one = *c == (F::zero() - F::one());

            if first {
                if is_neg {
                    write!(f, "-")?;
                }
            } else if is_neg {
                write!(f, " - ")?;
            } else {
                write!(f, " + ")?;
            }

            if deg == 0 {
                if first || is_neg {
                    write!(f, "{abs_s}")?;
                } else {
                    write!(f, "{s}")?;
                }
            } else {
                let omit_coeff = (is_neg && is_neg_one) || (!is_neg && is_one);
                if !omit_coeff {
                    if self.style.use_asterisk {
                        if first || is_neg {
                            write!(f, "{abs_s}*")?;
                        } else {
                            write!(f, "{s}*")?;
                        }
                    } else if first || is_neg {
                        write!(f, "{abs_s}")?;
                    } else {
                        write!(f, "{s}")?;
                    }
                }
                if deg == 1 {
                    write!(f, "{var}")?;
                } else if self.style.unicode_superscript {
                    write!(f, "{var}{}", to_superscript(deg))?;
                } else {
                    write!(f, "{var}^{deg}")?;
                }
            }
            first = false;
        }
        Ok(())
    }
}

pub struct RfDisplay<'a, F: Field> {
    pub rf: &'a RationalFunction<F>,
    pub var: &'a str,
    pub style: PolyStyle,
}

impl<'a, F> RfDisplay<'a, F>
where
    F: Field + fmt::Display + Clone + PartialEq,
{
    pub fn new(rf: &'a RationalFunction<F>, var: &'a str) -> Self {
        Self {
            rf,
            var,
            style: PolyStyle::default(),
        }
    }
    pub fn unicode_superscript(mut self, on: bool) -> Self {
        self.style.unicode_superscript = on;
        self
    }
    pub fn mul_asterisk(mut self, on: bool) -> Self {
        self.style.use_asterisk = on;
        self
    }
}

impl<'a, F> fmt::Display for RfDisplay<'a, F>
where
    F: Field + fmt::Display + Clone + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.rf.denominator.is_zero() {
            return write!(f, "(invalid: denominator=0)");
        }
        let n = PolyDisplay::<F> {
            poly: &self.rf.numerator,
            var: self.var,
            style: self.style,
        };
        let d = PolyDisplay::<F> {
            poly: &self.rf.denominator,
            var: self.var,
            style: self.style,
        };
        if self.rf.denominator.deg() == 0 {
            write!(f, "{n}")
        } else {
            write!(f, "({n})/({d})")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_formatting() {
        assert_eq!(fmt_real(0.0), "0");
        assert_eq!(fmt_real(1.0), "1");
        assert_eq!(fmt_real(-1.0), "-1");
        assert_eq!(fmt_real(1e-13), "0");
        assert_eq!(fmt_real(1.234567890123), "1.234567890123");
        assert_eq!(fmt_real(1.2345678901234), "1.234567890123");
    }

    #[test]
    fn complex_formatting() {
        use num_complex::Complex;
        assert_eq!(fmt_complex(Complex::new(0.0, 0.0)), "0");
        assert_eq!(fmt_complex(Complex::new(1.0, 0.0)), "1");
        assert_eq!(fmt_complex(Complex::new(0.0, 2.0)), "2i");
        assert_eq!(fmt_complex(Complex::new(0.0, -3.0)), "-3i");
        assert_eq!(fmt_complex(Complex::new(1.0, 2.0)), "1+2i");
        assert_eq!(fmt_complex(Complex::new(1.0, -2.0)), "1-2i");
    }
}
