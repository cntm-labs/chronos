use crate::domain::error::ChronosError;
use crate::domain::models::MemoryDelta;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TemporalStoragePort: Send + Sync {
    async fn persist_delta(&self, delta: MemoryDelta) -> Result<Uuid, ChronosError>;
    async fn get_history(&self, node_id: Uuid) -> Result<Vec<MemoryDelta>, ChronosError>;
}
