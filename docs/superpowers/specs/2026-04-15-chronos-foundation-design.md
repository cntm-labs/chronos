# Design Spec: Chronos Foundation - Hexagonal Orchestrator & API Gateway

**Date:** 2026-04-15  
**Status:** Draft  
**Author:** Gemini CLI  
**Topic:** Foundation Layer (Phase 1) - Orchestration & Security

## 1. Background & Goals
Chronos is the "Standard Memory for AGI," fusing symbolic reasoning with neural networks. The goal of this Foundation Phase is to establish a robust, maintainable, and formally verifiable orchestration layer in Rust (Axum) that can handle high-concurrency cognitive tasks while ensuring strict data integrity.

## 2. Architecture: Hexagonal (Clean) Architecture
To ensure long-term maintainability and decoupling from specific technology implementations (e.g., Mojo, Cntm-Graph), we adopt a Hexagonal approach.

### 2.1 Component Breakdown
- **Domain Layer:** Pure business logic and data entities. Contains `ChronosError` (Domain-Driven Errors) and core types like `CognitiveNode`, `MemoryDelta`, and `Proof`. No dependencies on external frameworks.
- **Ports (Interfaces):** Rust Traits defining the "Contract" for:
    - `CognitiveGraphPort`: Interface for `Cntm-Graph` (mutations and traversals).
    - `TemporalStoragePort`: Interface for `Isotime` (delta persistence and history).
    - `NeuralComputePort`: Interface for `Mojo Engine` (predictive modeling and inference).
- **Adapters (Implementation):**
    - `AxumAdapter`: HTTP/REST API gateway.
    - `MockAdapters`: Initial implementations for rapid prototyping and testing.
    - `LogicGuard`: Middleware for integrating Lean formal verification checks.
- **Application Layer (Orchestrator):** The brain that coordinates data flow between ports to fulfill user/agent reasoning requests.

## 3. Data Flow & Orchestration
1. **Ingress:** `AxumAdapter` receives an encrypted request.
2. **Identity Verification:** Middleware validates the `Cryptographic Identity` of the requester.
3. **Orchestration:**
    - Orchestrator fetches current context from `CognitiveGraphPort`.
    - Fetches historical deltas from `TemporalStoragePort`.
    - Dispatches combined context to `NeuralComputePort` (Mojo).
4. **Verification:** The resulting `Inference` and `GraphDelta` are sent to the `LogicGuard` for Lean-based verification.
5. **Egress:** Result is returned to the user; state changes are committed to the Graph and Temporal storage.

## 4. Security & Error Handling (Zero-Trust)
- **Zero-Trust Identity:** Every request must carry a verifiable cryptographic signature. Data sovereignty is enforced at the orchestrator level.
- **Logic Guard Middleware:** A formal verification layer that intercepts mutations to ensure they align with proven logical properties.
- **Domain-Driven Error Handling:** 
    - Strict avoidance of `unwrap()` and `panic!`.
    - Centralized `ChronosError` enum defining system-critical failures (e.g., `GraphInconsistency`, `VerificationFailed`, `TemporalDeltaLoss`).
- **End-to-End Integrity:** Every mutation returns a `Proof` that must be validated before the transaction is finalized.

## 5. Implementation Strategy (Phase 1)
- **Step 1:** Scaffold Rust project with the proposed directory structure.
- **Step 2:** Define core Domain entities and the `ChronosError` system.
- **Step 3:** Implement Ports (Traits) and the Orchestrator logic using Mocks.
- **Step 4:** Set up the Axum API gateway with initial Zero-Trust middleware placeholders.
- **Step 5:** Integration testing of the full Hexagonal flow.

## 6. Success Criteria
- [ ] Project scaffolds correctly with no `unwrap()` in the core logic.
- [ ] Orchestrator can successfully process a "Reasoning Request" using Mock adapters.
- [ ] Error handling correctly categorizes and propagates Domain errors.
- [ ] Cryptographic identity placeholders are integrated into the request lifecycle.
