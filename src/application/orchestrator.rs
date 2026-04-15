use crate::domain::error::ChronosError;
use crate::ports::graph::CognitiveGraphPort;
use crate::ports::storage::TemporalStoragePort;
use crate::ports::compute::NeuralComputePort;
use std::sync::Arc;
use uuid::Uuid;

pub struct Orchestrator {
    graph: Arc<dyn CognitiveGraphPort>,
    storage: Arc<dyn TemporalStoragePort>,
    compute: Arc<dyn NeuralComputePort>,
}

impl Orchestrator {
    pub fn new(
        graph: Arc<dyn CognitiveGraphPort>,
        storage: Arc<dyn TemporalStoragePort>,
        compute: Arc<dyn NeuralComputePort>,
    ) -> Self {
        Self {
            graph,
            storage,
            compute,
        }
    }

    pub async fn reason(&self, node_id: Uuid) -> Result<String, ChronosError> {
        let node = self.graph.get_node(node_id).await?;
        let _history = self.storage.get_history(node_id).await?;
        let result = self.compute.predict(vec![node]).await?;
        Ok(result)
    }
}
