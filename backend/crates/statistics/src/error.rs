use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum StatisticsError {
    InvalidParameter {
        what: &'static str,
        value: String,
    },
    DomainError {
        what: &'static str,
        details: &'static str,
    },
    EmptyInput,
}

impl fmt::Display for StatisticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatisticsError::InvalidParameter { what, value } => {
                write!(f, "Invalid parameter for {what}: {value}")
            }
            StatisticsError::DomainError { what, details } => {
                write!(f, "Domain error in {what}: {details}")
            }
            StatisticsError::EmptyInput => write!(f, "Input data is empty"),
        }
    }
}

impl std::error::Error for StatisticsError {}

pub type Result<T> = std::result::Result<T, StatisticsError>;
