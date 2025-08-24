use std::fmt;

#[derive(Debug, Clone)]
pub enum ConcreteMathError {
    InvalidArgument { text: String },
    NotImplemented,
}

impl fmt::Display for ConcreteMathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConcreteMathError::InvalidArgument { text } => write!(f, "Invalid argument: {text}"),
            ConcreteMathError::NotImplemented => write!(f, "Feature not yet implemented"),
        }
    }
}

impl std::error::Error for ConcreteMathError {}

pub type Result<T> = std::result::Result<T, ConcreteMathError>;
