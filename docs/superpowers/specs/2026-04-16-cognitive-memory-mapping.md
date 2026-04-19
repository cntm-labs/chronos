# Design Spec: Cognitive Memory Mapping (Cntm-Graph & Isotime)

**Date:** 2026-04-16  
**Status:** Draft  
**Author:** Gemini CLI  
**Topic:** Integration Layer - Memory Graph & Temporal Persistence

## 1. Overview
This specification defines how **Chronos** interacts with `Cntm-Graph` (Cognitive Memory) and `Isotime` (Temporal Delta Logs) to provide AI with verifiable, persistent, and historical context.

## 2. Deep Dive: Cognitive Node Design (The AGI Brain Cell)
To achieve "Common Sense" in AGI, a node must store more than data; it must store **context**, **logic**, and **embeddings**.

```rust
pub struct CognitiveNode {
    pub id: Uuid,                 // Unique ID for Symbolic Reasoning (Lean)
    pub semantics: Vec<f32>,      // Neural Vector Embedding (for Mojo Engine)
    pub label: String,            // Human-readable/Symbolic identifier
    pub relations: Vec<Relation>, // Edges connecting to other nodes
    pub metadata: Metadata,       // Flexible attributes (Schema-less)
    pub integrity: Integrity,     // Formal Verification data
}

pub struct Relation {
    pub target_id: Uuid,
    pub rel_type: String,         // e.g., "IS_A", "PART_OF", "CAUSES"
    pub strength: f32,            // 0.0 to 1.0 (Probability/Confidence)
    pub proof_id: Option<Uuid>,   // Link to a Lean formal proof for this relation
}

pub struct Metadata {
    pub attention_weight: f32,    // Current "Interest" level of the system
    pub decay_rate: f32,          // How fast this memory fades over time
    pub epoch: u64,               // Versioning from Isotime
}

pub struct Integrity {
    pub hash: String,             // SHA-256 of the node content
    pub sign: String,             // Cryptographic signature (Zero-Trust)
    pub status: ProofStatus,      // [Unverified | Verified | Disproven]
}
```

### **Design Principles for Cognitive Node:**
- **Hybrid Representation:** Combines neural vectors (`Vec<f32>`) with symbolic labels and relations.
- **Verifiable Links:** Every relation can optionally point to a `proof_id`, allowing the AI to "explain" why it thinks two concepts are related.
- **Dynamic Importance:** `attention_weight` and `decay_rate` allow the memory to be "alive" (frequently used memories stay strong; irrelevant ones fade).

## 3. Zero-Copy Integration (Cntm-Graph)
Leveraging Rust's safety and `Cntm-Graph`'s performance:
- **Direct Mapping:** Use `cntm-graph`'s memory mapping to read `CognitiveNode` fields without copying.
- **Pointer Safety:** Ensure Rust handles lifetime for memory-mapped pointers safely.

## 4. Temporal Delta Persistence (Isotime)
- **Delta Generation:** Every change to a node (even weight updates) generates a `MemoryDelta`.
- **Sequential Context:** Isotime stores these deltas in order, allowing the Orchestrator to "reconstruct" the brain state at any point in time (Time-travel reasoning).

## 5. Implementation Strategy
1. **Model Expansion:** Update `src/domain/models.rs` with the new structure.
2. **Adapter Stubbing:** Implement `CntmGraphAdapter` and `IsotimeAdapter` with initial FFI bridge placeholders.
3. **Integration Test:** Extend `tests/integration_test.rs` to simulate a "Memory Flashback" (fetching historical context).
