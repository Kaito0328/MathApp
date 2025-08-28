use std::fmt;

use linalg::LinalgError;
use statistics::error::StatisticsError;

#[derive(Debug, Clone)]
pub enum StatsModelsError {
    InvalidParameter { what: &'static str, details: String },
    DimensionMismatch { expected: String, found: String },
    EmptyInput,
    ConvergenceFailure { details: String },
    Linalg(LinalgError),
    Statistics(StatisticsError),
}

impl fmt::Display for StatsModelsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatsModelsError::InvalidParameter { what, details } => {
                write!(f, "Invalid parameter for {what}: {details}")
            }
            StatsModelsError::DimensionMismatch { expected, found } => {
                write!(f, "Dimension mismatch: expected {expected}, found {found}")
            }
            StatsModelsError::EmptyInput => write!(f, "Input data is empty"),
            StatsModelsError::ConvergenceFailure { details } => {
                write!(f, "Convergence failure: {details}")
            }
            StatsModelsError::Linalg(e) => write!(f, "Linalg error: {e}"),
            StatsModelsError::Statistics(e) => write!(f, "Statistics error: {e}"),
        }
    }
}

impl std::error::Error for StatsModelsError {}

impl From<LinalgError> for StatsModelsError {
    fn from(value: LinalgError) -> Self {
        Self::Linalg(value)
    }
}

impl From<StatisticsError> for StatsModelsError {
    fn from(value: StatisticsError) -> Self {
        Self::Statistics(value)
    }
}

pub type Result<T> = std::result::Result<T, StatsModelsError>;
