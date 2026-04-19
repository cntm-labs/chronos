# Chronos Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Establish a robust, maintainable, and formally verifiable orchestration layer in Rust (Axum) using Hexagonal Architecture.

**Architecture:** Hexagonal (Clean) Architecture separating Domain, Ports, Adapters, and Application layers to ensure long-term decoupling and verifiability.

**Tech Stack:** Rust, Axum, Tokio, Serde, Tower-HTTP.

---

### Task 1: Project Scaffolding & Dependencies

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `src/domain/mod.rs`, `src/ports/mod.rs`, `src/application/mod.rs`, `src/adapters/mod.rs`

- [x] **Step 1: Create Cargo.toml with required dependencies**

```toml
[package]
name = "chronos"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tower-http = { version = "0.5", features = ["trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
thiserror = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
```

- [x] **Step 2: Initialize basic module structure**

```rust
// src/main.rs
mod domain;
mod ports;
mod application;
mod adapters;

#[tokio::main]
async fn main() {
    println!("Chronos Foundation Starting...");
}
```

- [x] **Step 3: Commit scaffolding**

```bash
git add Cargo.toml src/
git commit -m "chore: scaffold project structure"
```

---

### Task 2: Domain Layer - Error & Models

**Files:**
- Create: `src/domain/error.rs`
- Create: `src/domain/models.rs`
- Modify: `src/domain/mod.rs`

- [x] **Step 1: Implement Domain Errors (No unwrap policy)**

```rust
// src/domain/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChronosError {
    #[error("Graph inconsistency detected: {0}")]
    GraphInconsistency(String),
    #[error("Formal verification failed: {0}")]
    VerificationFailed(String),
    #[error("Temporal delta loss in Isotime: {0}")]
    TemporalDeltaLoss(String),
    #[error("Neural compute error: {0}")]
    ComputeError(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Internal system error: {0}")]
    Internal(String),
}
```

- [x] **Step 2: Define Core Models**

```rust
// src/domain/models.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveNode {
    pub id: Uuid,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDelta {
    pub node_id: Uuid,
    pub change: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub signature: String,
    pub verified: bool,
}
```

- [x] **Step 3: Export Domain modules**

```rust
// src/domain/mod.rs
pub mod error;
pub mod models;
```

- [x] **Step 4: Commit Domain layer**

```bash
git add src/domain/
git commit -m "feat: implement domain models and errors"
```

---

### Task 3: Ports Definition (Traits)

**Files:**
- Create: `src/ports/graph.rs`
- Create: `src/ports/storage.rs`
- Create: `src/ports/compute.rs`
- Modify: `src/ports/mod.rs`

- [x] **Step 1: Define CognitiveGraphPort**

```rust
// src/ports/graph.rs
use crate::domain::error::ChronosError;
use crate::domain::models::{CognitiveNode, MemoryDelta, Proof};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CognitiveGraphPort: Send + Sync {
    async fn get_node(&self, id: Uuid) -> Result<CognitiveNode, ChronosError>;
    async fn mutate_graph(&self, delta: MemoryDelta) -> Result<Proof, ChronosError>;
}
```

- [x] **Step 2: Define TemporalStoragePort**

```rust
// src/ports/storage.rs
use crate::domain::error::ChronosError;
use crate::domain::models::MemoryDelta;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait TemporalStoragePort: Send + Sync {
    async fn persist_delta(&self, delta: MemoryDelta) -> Result<Uuid, ChronosError>;
    async fn get_history(&self, node_id: Uuid) -> Result<Vec<MemoryDelta>, ChronosError>;
}
```

- [x] **Step 3: Define NeuralComputePort**

```rust
// src/ports/compute.rs
use crate::domain::error::ChronosError;
use crate::domain::models::CognitiveNode;
use async_trait::async_trait;

#[async_trait]
pub trait NeuralComputePort: Send + Sync {
    async fn predict(&self, context: Vec<CognitiveNode>) -> Result<String, ChronosError>;
}
```

- [x] **Step 4: Commit Ports**

```bash
git add src/ports/
git commit -m "feat: define ports for graph, storage, and compute"
```

---

### Task 4: Application Orchestrator

**Files:**
- Create: `src/application/orchestrator.rs`
- Modify: `src/application/mod.rs`

- [ ] **Step 1: Implement Orchestrator Logic**

```rust
// src/application/orchestrator.rs
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
        Self { graph, storage, compute }
    }

    pub async fn reason(&self, node_id: Uuid) -> Result<String, ChronosError> {
        let node = self.graph.get_node(node_id).await?;
        let _history = self.storage.get_history(node_id).await?;
        let result = self.compute.predict(vec![node]).await?;
        Ok(result)
    }
}
```

- [ ] **Step 2: Commit Orchestrator**

```bash
git add src/application/
git commit -m "feat: implement core orchestrator logic"
```

---

### Task 5: Mock Adapters & Integration Test

**Files:**
- Create: `src/adapters/mocks.rs`
- Create: `tests/integration_test.rs`

- [ ] **Step 1: Implement Mock Adapters for testing**

```rust
// src/adapters/mocks.rs
use crate::domain::error::ChronosError;
use crate::domain::models::{CognitiveNode, MemoryDelta, Proof};
use crate::ports::graph::CognitiveGraphPort;
use crate::ports::storage::TemporalStoragePort;
use crate::ports::compute::NeuralComputePort;
use async_trait::async_trait;
use uuid::Uuid;

pub struct MockGraph;
#[async_trait]
impl CognitiveGraphPort for MockGraph {
    async fn get_node(&self, id: Uuid) -> Result<CognitiveNode, ChronosError> {
        Ok(CognitiveNode { id, content: "Mock Node".to_string() })
    }
    async fn mutate_graph(&self, _delta: MemoryDelta) -> Result<Proof, ChronosError> {
        Ok(Proof { signature: "valid".to_string(), verified: true })
    }
}

pub struct MockStorage;
#[async_trait]
impl TemporalStoragePort for MockStorage {
    async fn persist_delta(&self, _delta: MemoryDelta) -> Result<Uuid, ChronosError> {
        Ok(Uuid::new_v4())
    }
    async fn get_history(&self, _node_id: Uuid) -> Result<Vec<MemoryDelta>, ChronosError> {
        Ok(vec![])
    }
}

pub struct MockCompute;
#[async_trait]
impl NeuralComputePort for MockCompute {
    async fn predict(&self, _context: Vec<CognitiveNode>) -> Result<String, ChronosError> {
        Ok("Reasoning Successful".to_string())
    }
}
```

- [ ] **Step 2: Write and Run Integration Test**

```rust
// tests/integration_test.rs
use chronos::application::orchestrator::Orchestrator;
use chronos::adapters::mocks::{MockGraph, MockStorage, MockCompute};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_orchestrator_flow() {
    let orchestrator = Orchestrator::new(
        Arc::new(MockGraph),
        Arc::new(MockStorage),
        Arc::new(MockCompute),
    );
    let result = orchestrator.reason(Uuid::new_v4()).await.unwrap();
    assert_eq!(result, "Reasoning Successful");
}
```

- [ ] **Step 3: Verify tests pass**

Run: `cargo test`
Expected: `test_orchestrator_flow ... ok`

- [ ] **Step 4: Commit Mocks and Tests**

```bash
git add src/adapters/ tests/
git commit -m "test: add mock adapters and verify orchestrator flow"
```
