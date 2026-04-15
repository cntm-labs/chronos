use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CognitiveNode {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryDelta {
    pub id: Uuid,
    pub node_id: Uuid,
    pub delta_type: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Proof {
    pub id: Uuid,
    pub target_id: Uuid,
    pub result: bool,
    pub evidence: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_serialization() {
        let id = Uuid::new_v4();
        let node = CognitiveNode {
            id,
            name: "Test Node".to_string(),
        };

        let serialized = serde_json::to_string(&node).unwrap();
        let deserialized: CognitiveNode = serde_json::from_str(&serialized).unwrap();

        assert_eq!(node.id, deserialized.id);
        assert_eq!(node.name, deserialized.name);
    }

    #[test]
    fn test_delta_serialization() {
        let id = Uuid::new_v4();
        let node_id = Uuid::new_v4();
        let delta = MemoryDelta {
            id,
            node_id,
            delta_type: "Update".to_string(),
            data: "Test Data".to_string(),
        };

        let serialized = serde_json::to_string(&delta).unwrap();
        let deserialized: MemoryDelta = serde_json::from_str(&serialized).unwrap();

        assert_eq!(delta.id, deserialized.id);
        assert_eq!(delta.node_id, deserialized.node_id);
    }

    #[test]
    fn test_proof_serialization() {
        let id = Uuid::new_v4();
        let target_id = Uuid::new_v4();
        let proof = Proof {
            id,
            target_id,
            result: true,
            evidence: "Test Evidence".to_string(),
        };

        let serialized = serde_json::to_string(&proof).unwrap();
        let deserialized: Proof = serde_json::from_str(&serialized).unwrap();

        assert_eq!(proof.id, deserialized.id);
        assert_eq!(proof.result, deserialized.result);
    }
}
