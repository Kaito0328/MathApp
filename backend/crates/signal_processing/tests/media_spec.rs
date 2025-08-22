use signal_processing::media::{audio_io, image_io};

#[test]
fn grayscale_image_round_trip_vec() {
    // Create a tiny 4x3 gradient in memory and save via API, then load and compare
    let w = 4u32;
    let h = 3u32;
    let data: Vec<f64> = (0..(w * h) as usize)
        .map(|i| (i as f64) / ((w * h - 1) as f64))
        .collect();
    let v = data.clone();
    let tmp = std::env::temp_dir().join("sp_img_rt.png");
    image_io::save_vec_to_grayscale(tmp.to_str().unwrap(), &v, w, h).unwrap();
    let (v2, w2, h2) = image_io::load_grayscale_to_vec(tmp.to_str().unwrap()).unwrap();
    assert_eq!((w2, h2), (w, h));
    assert_eq!(v2.len(), v.len());
    // Allow small quantization diffs
    for i in 0..v.len() {
        assert!((v[i] - v2[i]).abs() <= 3.0 / 255.0);
    }
}

#[test]
fn rgb_image_round_trip_vec() {
    // 3x2 color pattern in memory -> save -> load -> compare
    let w = 3u32;
    let h = 2u32;
    // pattern: R, G, B, then grayscale-ish
    let data: Vec<f64> = vec![
        1.0, 0.0, 0.0, // red
        0.0, 1.0, 0.0, // green
        0.0, 0.0, 1.0, // blue
        0.2, 0.5, 0.8, 0.9, 0.2, 0.1, 0.1, 0.9, 0.4,
    ];
    let v = data.clone();
    let tmp = std::env::temp_dir().join("sp_img_rgb_rt.png");
    image_io::save_vec_to_rgb(tmp.to_str().unwrap(), &v, w, h).unwrap();
    let (v2, w2, h2) = image_io::load_rgb_to_vec(tmp.to_str().unwrap()).unwrap();
    assert_eq!((w2, h2), (w, h));
    assert_eq!(v2.len(), v.len());
    for i in 0..v.len() {
        // allow 1/255 rounding
        assert!((v[i] - v2[i]).abs() <= 3.0 / 255.0);
    }
}

#[test]
fn wav_round_trip_vec() {
    // Write a short sinusoid and read back
    let sr = 8000u32;
    let n = 800usize;
    let tone: Vec<f64> = (0..n)
        .map(|i| (2.0 * std::f64::consts::PI * 440.0 * (i as f64) / sr as f64).sin() * 0.5)
        .collect();
    let v = tone;
    let tmp = std::env::temp_dir().join("sp_audio_rt.wav");
    audio_io::save_wav_mono_from_vec(tmp.to_str().unwrap(), &v, sr).unwrap();
    let (v2, info) = audio_io::load_wav_mono_to_vec(tmp.to_str().unwrap()).unwrap();
    assert_eq!(info.sample_rate, sr);
    assert!(v2.len() >= v.len()); // wav writer may pad
                                  // compare first N within small tolerance
    for i in 0..v.len() {
        assert!((v[i] - v2[i]).abs() <= 2.0 / 32768.0);
    }
}
