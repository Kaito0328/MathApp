use num_complex::Complex;

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
