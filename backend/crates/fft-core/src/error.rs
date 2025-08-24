use std::fmt;

#[derive(Debug, Clone)]
pub enum FftError {
    InvalidLength { n: usize },
    InvalidArgument { text: String },
    NotImplemented,
}

impl fmt::Display for FftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FftError::InvalidLength { n } => write!(f, "Invalid FFT length: {n}"),
            FftError::InvalidArgument { text } => write!(f, "Invalid argument: {text}"),
            FftError::NotImplemented => write!(f, "Feature not yet implemented"),
        }
    }
}

impl std::error::Error for FftError {}

pub type Result<T> = std::result::Result<T, FftError>;
