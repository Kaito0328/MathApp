use polynomial::Polynomial;
use signal_processing::iir::IIRFilter;
use signal_processing::signal::Signal;

fn main() {
    let fs = 1000.0;
    // H(z) = (1 - r)/(1 - r z^{-1})
    let a: f64 = 5.0;
    let r = (-a / fs).exp();
    let b = Polynomial::new(vec![1.0 - r]);
    let a = Polynomial::new(vec![1.0, -r]);
    let filt = IIRFilter::new_with_fs(b, a, fs);

    let n = 32;
    let mut x = vec![0.0; n];
    x[0] = 1.0;
    let xsig = Signal::new(x, fs);
    let y = filt.apply(&xsig);

    println!("Impulse response:");
    for (i, v) in y.data().iter().enumerate() {
        println!("n={i:02}: {v:.6}");
    }
}
