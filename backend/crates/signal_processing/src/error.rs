use std::fmt;

#[derive(Debug, Clone)]
pub enum SignalError {
    InvalidArgument { text: String },
    NotImplemented,
}

impl fmt::Display for SignalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignalError::InvalidArgument { text } => write!(f, "Invalid argument: {text}"),
            SignalError::NotImplemented => write!(f, "Feature not yet implemented"),
        }
    }
}

impl std::error::Error for SignalError {}

pub type Result<T> = std::result::Result<T, SignalError>;
