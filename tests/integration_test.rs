use chronos::adapters::mocks::{MockCompute, MockGraph, MockStorage};
use chronos::application::orchestrator::Orchestrator;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_orchestrator_flow() {
    let graph = Arc::new(MockGraph);
    let storage = Arc::new(MockStorage);
    let compute = Arc::new(MockCompute);

    let orchestrator = Orchestrator::new(graph, storage, compute);

    let node_id = Uuid::new_v4();
    let result = orchestrator.reason(node_id).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Mock Prediction");
}
