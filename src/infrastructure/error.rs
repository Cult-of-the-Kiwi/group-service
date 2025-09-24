use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfraError {
    #[error("Bar")]
    GenericError,
}
