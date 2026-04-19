# Project Intelligence & Operational Logic

This file is the operational core and the foundation for all operations. Gemini CLI MUST follow these protocols to maintain project integrity.

## 🎯 Architectural Intent
- **Core Mission:** Chronos: The Graph-based AI Cognition Layer with Formal Proofs.
  - **Goal:** To become the global standard for memory and common sense in Artificial General Intelligence (AGI).
- **Primary Stack:**
  - **Rust (Axum):** For high-concurrency orchestration and performance.
  - **Cntm-Graph:** A custom cognitive memory engine using zero-copy mapping (Sibling Repo: `../cntm-graph/`).
  - **Isotime:** A custom temporal persistence layer for high-throughput time-series deltas (Sibling Repo: `../isotime/`).
  - **Mojo (CUDA):** For high-performance AI/neural predictive modeling.
  - **Lean Proof Assistant:** For formal verification of cognitive logic, security protocols, and graph integrity.
- **System Nature:** The orchestration layer that fuses symbolic mathematics (formal logic) with neural networks to provide AI with verifiable memory and reasoning capabilities.

## 🌐 Co-operative Ecosystem & Dependencies
- **Sibling Repositories:** `chronos` depends on `../cntm-graph/` (Cognitive Memory) and `../isotime/` (Temporal Persistence).
- **Integration Points:**
  - `cntm-graph`: Provides the zero-copy memory mapping bridge.
  - `isotime`: Records real-time delta logs from `cntm-graph` to provide historical context data.
- **Workflow Mandate:** If integration issues occur, investigate sibling repositories to find the root cause and create GitHub Issues in those repositories to resolve the blockers.

## 🗝️ Key Principles
- **Verifiability:** Moving beyond probabilistic AI to logically proven reasoning.
- **Zero-cost Abstractions:** Prioritizing performance and safety.
- **SOLID & Resilience:** Ensuring long-term maintainability and high availability.
- **High Security:** Zero-trust and formally verified security protocols (Mission Critical).

## 🧬 Automated Lifecycle Management
1. **Research Sync:** When running `./scripts/update_notebookLM.sh`:
   - Must update `DESIGN_DECISIONS.md` with any new ADRs (Architecture Decision Records) found.
   - **Constraint:** Maintain a record of the 10 most recent ADRs.
2. **Logic Verification:**
   - All new graph mutations and cognitive kernels must pass Lean verification before commit.
   - Run `lean verify src/logic` if applicable.
3. **PR Creation Protocol:** When instructed to create a Pull Request:
   - **Summarize:** Analyze all commit messages since the last merge into `main`.
   - **Template:** Read and populate `.github/PULL_REQUEST_TEMPLATE.md`.
   - **Assign:** Automatically set the current developer as the Assignee.
4. **Pre-Commit Action:** Before every commit:
   - Run `tree -a -I 'node_modules|.git|target' > STRUCTURE.tree`.
   - Run stack-specific formatting (`cargo fmt`, `mojo format`).
   - Run `pre-commit run --all-files`.

## 🛠️ Tooling & Standards
- **Translation:** All technical specifications are in English. `locales/` must always be kept in sync.
- **Workflow Mastery:** Use `/superpower:executing-plans` for feature development tasks.
- **Security:** High security level (Mission Critical). All security protocols must be formally verified.

## 📂 Template Inventory
You manage: ARCHITECTURE.md, ROADMAP.md, CONTRIBUTING.md, DESIGN_DECISIONS.md, STRUCTURE.tree, SECURITY.md, LICENSE.md, FAQ.md, GOVERNANCE.md, SUPPORT.md, TROUBLESHOOTING.md, PHILOSOPHY.md, MANIFESTO.md, and `locales/README.{th,ja,zh}.md`.