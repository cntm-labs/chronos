use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChronosError {
    #[error("Graph inconsistency detected: {0}")]
    GraphInconsistency(String),
    #[error("Formal verification failed: {0}")]
    VerificationFailed(String),
    #[error("Temporal delta loss in Isotime: {0}")]
    TemporalDeltaLoss(String),
    #[error("Neural compute error: {0}")]
    ComputeError(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Internal system error: {0}")]
    Internal(String),
}
