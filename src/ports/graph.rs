use crate::domain::error::ChronosError;
use crate::domain::models::{CognitiveNode, MemoryDelta, Proof};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CognitiveGraphPort: Send + Sync {
    async fn get_node(&self, id: Uuid) -> Result<CognitiveNode, ChronosError>;
    async fn mutate_graph(&self, delta: MemoryDelta) -> Result<Proof, ChronosError>;
}
