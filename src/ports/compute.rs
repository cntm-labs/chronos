use crate::domain::error::ChronosError;
use crate::domain::models::CognitiveNode;
use async_trait::async_trait;

#[async_trait]
pub trait NeuralComputePort: Send + Sync {
    async fn predict(&self, context: Vec<CognitiveNode>) -> Result<String, ChronosError>;
}
