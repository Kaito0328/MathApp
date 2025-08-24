use std::fmt;

#[derive(Debug, Clone)]
pub enum PolynomialError {
    DivisionByZero,
    DegreeMismatch { lhs: usize, rhs: usize },
    InvalidArgument { text: String },
    NotImplemented,
}

impl fmt::Display for PolynomialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolynomialError::DivisionByZero => write!(f, "Division by zero"),
            PolynomialError::DegreeMismatch { lhs, rhs } => write!(f, "Degree mismatch: {lhs} vs {rhs}"),
            PolynomialError::InvalidArgument { text } => write!(f, "Invalid argument: {text}"),
            PolynomialError::NotImplemented => write!(f, "Feature not yet implemented"),
        }
    }
}

impl std::error::Error for PolynomialError {}

pub type Result<T> = std::result::Result<T, PolynomialError>;
