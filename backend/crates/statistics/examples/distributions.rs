use statistics::distribution::continuous::{core::Distribution, normal::Normal, uniform::Uniform, exponential::Exponential};

fn main() {
    // Fallible constructors
    let n = Normal::new(0.0, 1.0).expect("valid normal");
    let u = Uniform::new(-1.0, 2.0).expect("valid uniform");
    let e = Exponential::new(2.0).expect("valid exponential");

    // Use a few methods
    println!("normal mean={} var={} pdf(0)={}", n.mean(), n.variance(), n.pdf(0.0));
    println!("uniform q(0.25)={}", u.quantile(0.25));
    println!("exp cdf(0.5)={}", e.cdf(0.5));
}
