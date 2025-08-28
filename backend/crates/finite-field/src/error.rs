use std::fmt;

#[derive(Debug, Clone)]
pub enum FieldError {
    InvalidModulus { text: String },
    DivisionByZero,
    DegreeOverflow { max: usize, found: usize },
    InvalidArgument { text: String },
    NotImplemented,
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldError::InvalidModulus { text } => write!(f, "Invalid modulus: {text}"),
            FieldError::DivisionByZero => write!(f, "Division by zero"),
            FieldError::DegreeOverflow { max, found } => {
                write!(f, "Degree overflow: max {max}, found {found}")
            }
            FieldError::InvalidArgument { text } => write!(f, "Invalid argument: {text}"),
            FieldError::NotImplemented => write!(f, "Feature not yet implemented"),
        }
    }
}

impl std::error::Error for FieldError {}

pub type Result<T> = std::result::Result<T, FieldError>;
