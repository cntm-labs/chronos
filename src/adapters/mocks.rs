use crate::domain::error::ChronosError;
use crate::domain::models::{CognitiveNode, Integrity, MemoryDelta, Metadata, Proof, ProofStatus};
use crate::ports::compute::NeuralComputePort;
use crate::ports::graph::CognitiveGraphPort;
use crate::ports::storage::TemporalStoragePort;
use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

pub struct MockGraph;
#[async_trait]
impl CognitiveGraphPort for MockGraph {
    async fn get_node(&self, id: Uuid) -> Result<CognitiveNode, ChronosError> {
        Ok(CognitiveNode {
            id,
            semantics: vec![0.1, 0.2, 0.3],
            label: "Mock Node".to_string(),
            relations: vec![],
            metadata: Metadata {
                attention_weight: 1.0,
                decay_rate: 0.01,
                epoch: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            integrity: Integrity {
                hash: "mock_hash".to_string(),
                sign: "mock_signature".to_string(),
                status: ProofStatus::Verified,
            },
        })
    }
    async fn mutate_graph(&self, _delta: MemoryDelta) -> Result<Proof, ChronosError> {
        Ok(Proof {
            signature: "mock_mutation_proof".to_string(),
            verified: true,
        })
    }
}

pub struct MockStorage;
#[async_trait]
impl TemporalStoragePort for MockStorage {
    async fn persist_delta(&self, _delta: MemoryDelta) -> Result<Uuid, ChronosError> {
        Ok(Uuid::new_v4())
    }
    async fn get_history(&self, _node_id: Uuid) -> Result<Vec<MemoryDelta>, ChronosError> {
        Ok(vec![MemoryDelta {
            node_id: Uuid::new_v4(),
            change: "Initial creation".to_string(),
            timestamp: Utc::now(),
        }])
    }
}

pub struct MockCompute;
#[async_trait]
impl NeuralComputePort for MockCompute {
    async fn predict(&self, _context: Vec<CognitiveNode>) -> Result<String, ChronosError> {
        Ok("Reasoning Successful".to_string())
    }
}
