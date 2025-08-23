use signal_processing::sampling::{decimate, down_sample, expand, resample, upsample};
use signal_processing::window::WindowType;

fn vec_close(a: &[f64], b: &[f64], eps: f64) {
    assert_eq!(a.len(), b.len());
    for i in 0..a.len() {
        assert!(
            (a[i] - b[i]).abs() <= eps,
            "idx {}: {} vs {}",
            i,
            a[i],
            b[i]
        );
    }
}

#[test]
fn expand_inserts_zeros() {
    let x = vec![1.0, -2.0, 3.0];
    let y = expand(&x, 3);
    assert_eq!(y, vec![1.0, 0.0, 0.0, -2.0, 0.0, 0.0, 3.0, 0.0, 0.0]);

    // factor<=1 はそのまま返す
    assert_eq!(expand(&x, 1), x);
}

#[test]
fn decimate_keeps_every_mth() {
    let x: Vec<f64> = (0..10).map(|v| v as f64).collect();
    let y = decimate(&x, 3);
    assert_eq!(y, vec![0.0, 3.0, 6.0, 9.0]);

    // factor==0 -> 空
    assert_eq!(decimate(&x, 0), Vec::<f64>::new());
}

#[test]
fn upsample_equals_filter_of_expand() {
    // ランダムでなく決定的な小信号
    let x = vec![1.0, 0.5, -1.0, 0.25];
    let l = 2usize;
    let taps = 41usize;
    let win = WindowType::Hamming;

    let y1 = upsample(&x, l, taps, win);

    // 期待値: expand -> LPF（同じ設計）
    use signal_processing::dft::conv_auto_f64;
    use signal_processing::fir::design_fir_lowpass;
    let xp = expand(&x, l);
    let mut h = design_fir_lowpass(taps, 0.5 / l as f64, win);
    for c in &mut h {
        *c *= l as f64;
    }
    let y2 = conv_auto_f64(&xp, &h);

    vec_close(&y1, &y2, 1e-9);
}

#[test]
fn downsample_equals_filter_then_pick() {
    let x = vec![1.0, -1.0, 0.5, -0.5, 2.0, -2.0, 1.5, -1.5];
    let m = 2usize;
    let taps = 31usize;
    let win = WindowType::Hamming;

    let y1 = down_sample(&x, m, taps, win);

    use signal_processing::dft::conv_auto_f64;
    use signal_processing::fir::design_fir_lowpass;
    let h = design_fir_lowpass(taps, 0.5 / m as f64, win);
    let xf = conv_auto_f64(&x, &h);
    let y2 = decimate(&xf, m);

    vec_close(&y1, &y2, 1e-9);
}

#[test]
fn resample_composes_up_and_down() {
    let x: Vec<f64> = (0..16)
        .map(|i| (2.0 * std::f64::consts::PI * (i as f64) / 8.0).sin())
        .collect();
    let l = 3usize;
    let m = 2usize;
    let taps = 41usize;
    let win = WindowType::Hamming;

    let y = resample(&x, l, m, taps, win);

    let up = upsample(&x, l, taps, win);
    let y_ref = down_sample(&up, m, taps, win);

    vec_close(&y, &y_ref, 1e-9);
}
