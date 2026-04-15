use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChronosError {
    #[error("Internal system error: {0}")]
    Internal(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Graph mutation error: {0}")]
    GraphError(String),

    #[error("Temporal persistence error: {0}")]
    TemporalError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ChronosError::NotFound("Node 123 not found".to_string());
        assert_eq!(err.to_string(), "Resource not found: Node 123 not found");
    }
}
