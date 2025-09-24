use thiserror::Error;

use crate::infrastructure::error::InfraError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bar")]
    ErrorBar,
    #[error("Infra error: {0}")]
    InfraError(#[from] InfraError),
}
