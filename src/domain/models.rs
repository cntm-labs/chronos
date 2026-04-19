use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveNode {
    pub id: Uuid,                 // Unique ID for Symbolic Reasoning (Lean)
    pub semantics: Vec<f32>,      // Neural Vector Embedding (for Mojo Engine)
    pub label: String,            // Human-readable/Symbolic identifier
    pub relations: Vec<Relation>, // Edges connecting to other nodes
    pub metadata: Metadata,       // Flexible attributes (Schema-less)
    pub integrity: Integrity,     // Formal Verification data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub target_id: Uuid,
    pub rel_type: String,       // e.g., "IS_A", "PART_OF", "CAUSES"
    pub strength: f32,          // 0.0 to 1.0 (Probability/Confidence)
    pub proof_id: Option<Uuid>, // Link to a Lean formal proof for this relation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub attention_weight: f32, // Current "Interest" level of the system
    pub decay_rate: f32,       // How fast this memory fades over time
    pub epoch: u64,            // Versioning from Isotime
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integrity {
    pub hash: String,        // SHA-256 of the node content
    pub sign: String,        // Cryptographic signature (Zero-Trust)
    pub status: ProofStatus, // [Unverified | Verified | Disproven]
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProofStatus {
    Unverified,
    Verified,
    Disproven,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDelta {
    pub node_id: Uuid,
    pub change: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub signature: String,
    pub verified: bool,
}
