use std::fmt;

#[derive(Debug, Clone)]
pub enum NumberTheoryError {
    InvalidArgument { text: String },
    NotImplemented,
}

impl fmt::Display for NumberTheoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberTheoryError::InvalidArgument { text } => write!(f, "Invalid argument: {text}"),
            NumberTheoryError::NotImplemented => write!(f, "Feature not yet implemented"),
        }
    }
}

impl std::error::Error for NumberTheoryError {}

pub type Result<T> = std::result::Result<T, NumberTheoryError>;
