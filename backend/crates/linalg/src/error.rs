use std::fmt;

#[derive(Debug, Clone)]
pub enum LinalgError {
    DimensionMismatch { expected: String, found: String },
    NotSquareMatrix,
    SingularMatrix,
    IndexOutOfBounds { index: usize, size: usize },
    InvalidDimension { dim: usize },
    NotImplemented,
}

impl fmt::Display for LinalgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinalgError::DimensionMismatch { expected, found } => {
                write!(
                    f,
                    "Dimension mismatch: expected {}, found {}",
                    expected, found
                )
            }
            LinalgError::NotSquareMatrix => write!(f, "Operation requires a square matrix"),
            LinalgError::SingularMatrix => write!(f, "Matrix is singular (not invertible)"),
            LinalgError::IndexOutOfBounds { index, size } => {
                write!(f, "Index {} is out of bounds for size {}", index, size)
            }
            LinalgError::InvalidDimension { dim } => {
                write!(f, "Invalid dimension: {}", dim)
            }
            LinalgError::NotImplemented => write!(f, "Feature not yet implemented"),
        }
    }
}

impl std::error::Error for LinalgError {}

pub type Result<T> = std::result::Result<T, LinalgError>;
