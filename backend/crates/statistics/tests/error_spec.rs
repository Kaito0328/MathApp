use statistics::distribution::{
    continuous::{normal::Normal, uniform::Uniform, exponential::Exponential, chi_square::ChiSquare, gamma::Gamma, f::F, t::T},
    discrete::{bernoulli::Bernoulli, binomial::Binomial, categorical::Categorical, poisson::Poisson},
};

#[test]
fn constructor_errors() {
    assert!(Normal::new(0.0, 0.0).is_err());
    assert!(Uniform::new(1.0, -1.0).is_err());
    assert!(Exponential::new(0.0).is_err());
    assert!(ChiSquare::new(0).is_err());
    assert!(Gamma::new(-1.0, 1.0).is_err());
    assert!(F::new(0, 1).is_err());
    assert!(T::new(0).is_err());
    assert!(Bernoulli::new(2.0).is_err());
    assert!(Binomial::new(0, 0.5).is_err());
    assert!(Categorical::new(vec![]).is_err());
    assert!(Poisson::new(-1.0).is_err());
}
