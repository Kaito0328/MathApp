use std::fmt;

#[derive(Debug, Clone)]
pub enum CodingError {
    InvalidParameters { text: String },
    DecodeFailure { text: String },
    RankDeficient,
    InvalidArgument { text: String },
    NotImplemented,
}

impl fmt::Display for CodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodingError::InvalidParameters { text } => write!(f, "Invalid parameters: {text}"),
            CodingError::DecodeFailure { text } => write!(f, "Decode failure: {text}"),
            CodingError::RankDeficient => write!(f, "Matrix is rank-deficient for decoding"),
            CodingError::InvalidArgument { text } => write!(f, "Invalid argument: {text}"),
            CodingError::NotImplemented => write!(f, "Feature not yet implemented"),
        }
    }
}

impl std::error::Error for CodingError {}

pub type Result<T> = std::result::Result<T, CodingError>;

// Allow seamless conversion from linalg errors when coding uses matrix/vector ops
impl From<linalg::LinalgError> for CodingError {
    fn from(e: linalg::LinalgError) -> Self {
        // Map all linalg errors to InvalidArgument for now; detailed mapping can be added later
        CodingError::InvalidArgument {
            text: e.to_string(),
        }
    }
}
