use std::fmt;

#[derive(Debug, Clone)]
pub enum ConvolutionError {
    InvalidArgument { text: String },
    FftFailed { text: String },
    NotImplemented,
}

impl fmt::Display for ConvolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConvolutionError::InvalidArgument { text } => write!(f, "Invalid argument: {text}"),
            ConvolutionError::FftFailed { text } => write!(f, "FFT failed: {text}"),
            ConvolutionError::NotImplemented => write!(f, "Feature not yet implemented"),
        }
    }
}

impl std::error::Error for ConvolutionError {}

pub type Result<T> = std::result::Result<T, ConvolutionError>;
