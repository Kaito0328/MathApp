use std::fmt;

#[derive(Debug, Clone)]
pub enum LtiError {
    InvalidArgument { text: String },
    NotImplemented,
}

impl fmt::Display for LtiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LtiError::InvalidArgument { text } => write!(f, "Invalid argument: {text}"),
            LtiError::NotImplemented => write!(f, "Feature not yet implemented"),
        }
    }
}

impl std::error::Error for LtiError {}

pub type Result<T> = std::result::Result<T, LtiError>;
